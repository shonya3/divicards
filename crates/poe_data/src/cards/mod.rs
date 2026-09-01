//! Divination card extraction pipeline with two outputs:
//!
//! - [`extract_cards`] → [`CardsData`] — game-file extraction plus atlas drop maps,
//!   release leagues, discovery weights and poe.ninja prices
//! - [`card_element_data`] → `Vec<DivinationCardElementData>` — enriches the cards
//!   with wiki reward descriptions and RePoE item classes for the site's card
//!   custom element
//!
//! # Stages
//!
//! 1. **`dat::extract`** — Reads `.datc64` game files (BaseItemTypes, CurrencyItems,
//!    DivinationCardArt, FlavourText) to produce raw card data.
//! 2. **`atlas::extract`** — Reads `AtlasNode.datc64` + `WorldAreas.datc64` to populate
//!    each card's `atlasMaps` (map area names where the card drops).
//! 3. **`league::fetch_card_release_versions` / `league::fetch_league_info`** — Fetches
//!    each card's release version and the league list from the wiki Cargo API, then
//!    matches them to populate each card's `league` field.
//! 4. **`weights::fetch`** — Fetches community-submitted per-league card discovery weights
//!    from [Google Sheets](https://docs.google.com/spreadsheets/d/1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80).
//! 5. **`DiviPrices::fetch`** — Fetches card prices from poe.ninja exchange economy API,
//!    merged into each card's `price` field.
//! 6. **`reward::fetch_all_rewards`** — Fetches reward descriptions from
//!    [poewiki.net](https://www.poewiki.net) MediaWiki API, parses wikitext.
//! 7. **`items::ItemDb::load`** — Fetches item class data from [RePoE fork](https://repoe-fork.github.io/)
//!    (`uniques.json` + `base_items.json`).
//! 8. **`card_element_data`** — Combines all sources, skips cards marked
//!    `disabled` (set from `divi::consts::LEGACY_CARDS` in `extract_cards`),
//!    and produces `Vec<DivinationCardElementData>` for JSON output.

pub mod atlas;
pub mod dat;
pub mod items;
pub mod league;
pub mod markup;
pub mod reward;
pub mod weights;

use anyhow::Result;
use card_element::{DivinationCardElementData, UniqueReward};
use divcord::poe_data::{
    cards::{Card, CardsData, LeagueWeightsCollected},
    league::ReleaseVersion,
};
use crate::{GameFiles, log};
use divi::prices::Prices as DiviPrices;
use divi::TradeLeague;
use std::collections::HashMap;

/// Opens game files, extracts cards, and attaches community weights, prices, and league info.
///
/// `league` is used for poe.ninja price fetching.
pub async fn extract_cards(source: &GameFiles, league: TradeLeague) -> Result<CardsData> {
    eprintln!("{}", log::ColoredLabel::Cards);
    let opened = crate::open_game_data(source).await?;

    // Game-file reads use blocking IO (the CDN backend in particular), so run
    // the extraction phases on the blocking pool.
    let mut cards = {
        tokio::task::spawn_blocking(move || {
            let opened = opened.lock().unwrap();
            eprintln!("extracting cards from .datc64...");
            let mut cards = dat::extract(&opened.fs, &opened.schemas)?;

            eprintln!("extracting atlas maps...");
            let atlas_maps = atlas::extract(&opened.fs, &opened.schemas)?;
            for card in &mut cards {
                card.atlas_maps = atlas_maps.get(&card.id).cloned().unwrap_or_default();
            }
            Ok::<_, anyhow::Error>(cards)
        })
        .await??
    };

    eprintln!("fetching league info + card release versions (wiki)...");
    let (leagues, release_versions) = tokio::join!(
        league::fetch_league_info(),
        league::fetch_card_release_versions(),
    );
    let leagues = leagues?;
    let release_versions = release_versions?;

    for card in &mut cards {
        card.league = release_versions
            .get(&card.name)
            .and_then(|v| league::match_league(v, &leagues));
    }

    eprintln!("fetching weights (Google Sheets) + prices (poe.ninja/{league})...");
    let (weights_data, prices_data) = tokio::join!(
        weights::fetch(),
        DiviPrices::fetch(league),
    );
    let weights_data = weights_data?;
    let price_lookup: HashMap<&str, f32> = match &prices_data {
        Ok(p) => p.0.iter().filter_map(|dp| dp.price.map(|v| (dp.name.as_str(), v))).collect(),
        Err(_) => HashMap::new(),
    };
    eprintln!(
        "prices (poe.ninja/{league}): {} cards with prices",
        price_lookup.len()
    );

    for card in &mut cards {
        let w = weights_data.per_card.get(&card.name);
        card.weights = w.cloned().unwrap_or_else(|| {
            weights_data
                .versions
                .iter()
                .map(|v| (v.clone(), 0.0))
                .collect()
        });
        card.disabled = divi::consts::LEGACY_CARDS.contains(&card.name.as_str());
        card.price = price_lookup.get(card.name.as_str()).copied();
    }

    let latest_version = weights_data.versions.first().cloned().unwrap_or_default();
    let latest_contributors = weights_data
        .contributors
        .get(&latest_version)
        .cloned()
        .unwrap_or_default();
    let latest = LeagueWeightsCollected {
        version: ReleaseVersion::new(latest_version),
        total_cards: weights_data.total_cards,
        contributors: latest_contributors,
    };
    let dict: HashMap<_, _> = cards.into_iter().map(|c| (c.name.clone(), c)).collect();

    Ok(CardsData {
        dict,
        latest_weights_collected: latest,
    })
}

pub async fn card_element_data(
    cards: &[Card],
) -> Result<(Vec<DivinationCardElementData>, items::ItemDb)> {
    let card_names: Vec<String> = cards.iter().map(|c| c.name.clone()).collect();
    eprintln!("fetching rewards (poewiki) + item classes (RePoE)...");
    let (item_db, rewards) = tokio::join!(
        items::ItemDb::load(),
        reward::fetch_all_rewards(&card_names),
    );
    let item_db = item_db?;
    let rewards = rewards?;

    let mut enriched = Vec::new();
    for card in cards {
        if card.disabled {
            continue;
        }
        let name = &card.name;
        let info = rewards.get(name.as_str());
        let reward_html = info
            .map(|i| {
                if i.reward_name == "Random Divination Card" {
                    "<div class=reward><p><span class=divination>Divination Card</span></p></div>"
                        .to_string()
                } else if !i.reward_name.is_empty() {
                    markup::markup_to_html(&i.markup)
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();

        let unique = info.filter(|i| i.is_unique).map(|i| {
            let lookup = i.reward_name.strip_prefix("The ").unwrap_or(&i.reward_name);
            let class = item_db
                .resolve_item_class(&i.reward_name)
                .or_else(|| item_db.resolve_item_class(&format!("The {}", lookup)))
                .or_else(|| item_db.resolve_item_class(lookup))
                .unwrap_or(&i.reward_name);
            UniqueReward {
                name: i.reward_name.clone(),
                item_class: class.to_string(),
            }
        });

        enriched.push(DivinationCardElementData {
            slug: slug::slugify(name),
            name: name.clone(),
            art_filename: card.art_filename.clone(),
            min_level: card.min_level,
            stack_size: card.stack_size,
            flavour_text: card.flavour_text.clone(),
            reward_html,
            unique,
        });
    }
    Ok((enriched, item_db))
}
