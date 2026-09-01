//! Community-submitted per-league card discovery weights.
//!
//! Reads from a community-maintained Google Sheets spreadsheet where players
//! submit how many of each divination card they found per league. Weights are
//! computed by [`divi::sample::Sample::create`] using its built-in normalization
//! against "Rain of Chaos".
//!
//! Requires `GOOGLE_API_KEY` environment variable. The pipeline errors out if
//! the key is missing or the fetch fails.
//!
//! Spreadsheet: <https://docs.google.com/spreadsheets/d/1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80>
//!
//! # League ranges
//!
//! | League | Names range | Amounts range |
//! |--------|-------------|---------------|
//! | 3.29   | `3.29!G3:G` | `3.29!R3:R` |
//! | 3.28   | `3.28!F3:F` | `3.28!Q3:Q` |
//! | 3.27   | `3.27!H3:H` | `3.27!S3:S` |
//! | 3.26   | `3.26!H3:H` | `3.26!S3:S` |
//! | 3.25   | `3.25!F3:F` | `3.25!Q3:Q` |
//! | 3.24   | `3.24!D3:D` | `3.24!O3:O` |
//! | 3.23   | `3.23!D3:D` | `3.23!P3:P` |

use anyhow::{Context, Result};
use divcord::poe_data::cards::Contributor;
use divi::sample::{Input, Sample};
use googlesheets::sheet::Credential;
use std::collections::HashMap;

const SPREADSHEET_ID: &str = "1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80";

struct LeagueRange {
    version: String,
    names_range: String,
    /// Total amounts per card (sum of all contributors).
    amounts_range: String,
}

fn league_ranges() -> [LeagueRange; 7] {
    [
        LeagueRange {
            version: "3.29".into(),
            names_range: "3.29!G3:G".into(),
            amounts_range: "3.29!R3:R".into(),
        },
        LeagueRange {
            version: "3.28".into(),
            names_range: "3.28!F3:F".into(),
            amounts_range: "3.28!Q3:Q".into(),
        },
        LeagueRange {
            version: "3.27".into(),
            names_range: "3.27!H3:H".into(),
            amounts_range: "3.27!S3:S".into(),
        },
        LeagueRange {
            version: "3.26".into(),
            names_range: "3.26!H3:H".into(),
            amounts_range: "3.26!S3:S".into(),
        },
        LeagueRange {
            version: "3.25".into(),
            names_range: "3.25!F3:F".into(),
            amounts_range: "3.25!Q3:Q".into(),
        },
        LeagueRange {
            version: "3.24".into(),
            names_range: "3.24!D3:D".into(),
            amounts_range: "3.24!O3:O".into(),
        },
        LeagueRange {
            version: "3.23".into(),
            names_range: "3.23!D3:D".into(),
            amounts_range: "3.23!P3:P".into(),
        },
    ]
}

/// Community-submitted per-league card discovery weights.
///
/// `versions` lists all league versions fetched (e.g. `["3.28", "3.27", …]`).
/// `per_card` maps card name → per-league weight; every card that appeared in
/// any league has all `versions` as keys (`0.0` for missing leagues).
/// `total_cards` is the sum of all submitted amounts in the latest league.
#[derive(Clone, Debug)]
pub struct Weights {
    pub versions: Vec<String>,
    pub per_card: HashMap<String, HashMap<String, f32>>,
    pub total_cards: u32,
    pub contributors: HashMap<String, Vec<Contributor>>,
}

struct LeagueData {
    idx: usize,
    version: String,
    sample: Option<Sample>,
    contributors: Vec<Contributor>,
}

/// Extract the first column letter from a range string.
/// e.g. `"3.29!G3:G"` → `'G'`
fn first_col_from_range(range: &str) -> Option<char> {
    range
        .split('!')
        .nth(1)?
        .chars()
        .find(|c| c.is_ascii_alphabetic())
}

/// Build the range for contributor headers + totals (rows 1-2).
/// Derived from names_range and amounts_range:
/// e.g. (`"3.29!G3:G"`, `"3.29!R3:R"`) → `"3.29!H1:Q2"`
fn contributors_range(sheet: &str, names_range: &str, amounts_range: &str) -> Option<String> {
    let names_col = first_col_from_range(names_range)?;
    let amounts_col = first_col_from_range(amounts_range)?;
    let start = (names_col as u8 + 1) as char;
    let end = (amounts_col as u8 - 1) as char;
    Some(format!("{sheet}!{start}1:{end}2"))
}

/// Parse contributor names (row 1) and their totals (row 2).
/// Merges duplicate names by summing their totals.
fn parse_contributors(values: &[Vec<serde_json::Value>]) -> Vec<Contributor> {
    if values.len() < 2 {
        return Vec::new();
    }
    let names = &values[0];
    let totals = &values[1];
    let mut merged: HashMap<String, u32> = HashMap::new();
    for (name, total) in names.iter().zip(totals.iter()) {
        let Some(name) = name.as_str() else {
            continue;
        };
        if name.is_empty() || name == "Total" {
            break;
        }
        if let Some(amount) = total.as_str().and_then(|s| s.parse::<u32>().ok()) {
            *merged.entry(name.to_string()).or_insert(0) += amount;
        }
    }
    merged
        .into_iter()
        .map(|(name, total_cards)| Contributor { name, total_cards })
        .collect()
}

async fn fetch_league_data(api_key: String, lr: &LeagueRange, idx: usize) -> Result<LeagueData> {
    eprintln!("  fetching {}...", lr.version);

    let contrib_range =
        contributors_range(&lr.version, &lr.names_range, &lr.amounts_range).unwrap_or_default();

    let resp = googlesheets::sheet::read_batch(
        SPREADSHEET_ID,
        &[&lr.names_range, &lr.amounts_range, &contrib_range],
        Credential::ApiKey(api_key),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to read Google Sheets range {}: {}", lr.version, e))?;

    let contributors = resp
        .value_ranges
        .get(2)
        .map(|vr| parse_contributors(&vr.values))
        .unwrap_or_default();

    let (sample, row_count) = if resp.value_ranges.len() >= 2 {
        let row_count = resp.value_ranges[0]
            .values
            .iter()
            .filter(|row| {
                row.first()
                    .and_then(|v| v.as_str())
                    .is_some_and(|s| !s.trim().is_empty())
            })
            .count();
        let input = Input::try_from(resp)
            .map_err(|e| anyhow::anyhow!("Failed to parse sheet data for {}: {}", lr.version, e))?;
        let s = Sample::create(input, None)
            .map_err(|e| anyhow::anyhow!("Failed to compute weights for {}: {}", lr.version, e))?;
        (Some(s), row_count)
    } else {
        (None, 0)
    };

    eprintln!(
        "  {} done ({} cards in sheet, {} contributors)",
        lr.version,
        row_count,
        contributors.len()
    );
    Ok(LeagueData {
        idx,
        version: lr.version.clone(),
        sample,
        contributors,
    })
}

/// Fetch community-submitted drop weights from Google Sheets.
///
/// Reads card discovery counts per league from the community spreadsheet,
/// normalizes against "Rain of Chaos" as reference.
pub async fn fetch() -> Result<Weights> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").context("GOOGLE_API_KEY not set")?;
    let ranges = league_ranges();
    let versions: Vec<String> = ranges.iter().map(|r| r.version.clone()).collect();

    let all_data = futures::future::try_join_all(
        ranges
            .iter()
            .enumerate()
            .map(|(idx, lr)| fetch_league_data(api_key.clone(), lr, idx)),
    )
    .await?;

    let mut card_weights: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut total_cards: u32 = 0;
    let mut contributors: HashMap<String, Vec<Contributor>> = HashMap::new();

    for data in &all_data {
        let Some(ref sample) = data.sample else {
            continue;
        };

        if data.idx == 0 {
            total_cards = sample.cards.0.iter().map(|c| c.amount).sum();
        }

        if !data.contributors.is_empty() {
            // Panics if sum of contributor totals != total cards in sample for this league.
            {
                let contributors_total: u32 = data.contributors.iter().map(|c| c.total_cards).sum();
                let sample_total: u32 = sample.cards.0.iter().map(|c| c.amount).sum();
                assert_eq!(
                    contributors_total, sample_total,
                    "Contributors total ({}) != sample total ({}) for league {}",
                    contributors_total, sample_total, data.version
                );
            }
            contributors.insert(data.version.clone(), data.contributors.clone());
        }

        for card in &sample.cards.0 {
            if let Some(weight) = card.weight {
                card_weights
                    .entry(card.name.clone())
                    .or_default()
                    .insert(data.version.clone(), weight);
            }
        }
    }

    // Fill in missing league versions with 0.0 for any card that has partial entries
    for weights in card_weights.values_mut() {
        for v in &versions {
            weights.entry(v.clone()).or_insert(0.0);
        }
    }

    Ok(Weights {
        versions,
        per_card: card_weights,
        total_cards,
        contributors,
    })
}
