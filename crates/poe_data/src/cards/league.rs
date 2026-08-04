//! Wiki Cargo API helpers for matching cards to their release leagues.
//!
//! # Data flow
//!
//! 1. [`fetch_league_info`] queries the `events` table for challenge leagues.
//! 2. [`fetch_card_release_versions`] queries the `items` table for each card's release version.
//! 3. [`match_league`] joins the two: finds the league whose `major.minor` version
//!    matches the card's release version.
//!
//! HTML entities (`&#039;` etc.) in wiki responses are decoded by [`decode_html_entities`],
//! which is a workaround for an upstream issue where the Cargo API returns numeric
//! HTML entities for apostrophes and other characters in card names.

use anyhow::{Context, Result};
use divcord::poe_data::league::{LeagueReleaseInfo, ReleaseVersion};
use std::collections::HashMap;



pub async fn fetch_league_info() -> Result<Vec<LeagueReleaseInfo>> {
    let rows = cargo_query(&[
        ("action", "cargoquery"),
        ("format", "json"),
        ("tables", "events"),
        (
            "fields",
            "events.name,events.release_date,events.release_version",
        ),
        ("where", "events.type=\"Challenge league\""),
        ("limit", "100"),
    ])
    .await?;
    let mut leagues = Vec::new();
    for row in &rows {
        let title = &row["title"];
        let name = title["name"].as_str().unwrap_or_default().to_string();
        if !name.to_lowercase().contains("league") {
            continue;
        }
        leagues.push(LeagueReleaseInfo {
            name,
            date: title["release date"]
                .as_str()
                .unwrap_or(title["release_date"].as_str().unwrap_or_default())
                .to_string(),
            version: ReleaseVersion::new(
                title["release version"]
                    .as_str()
                    .unwrap_or(title["release_version"].as_str().unwrap_or_default())
                    .to_string(),
            ),
        });
    }
    Ok(leagues)
}

pub async fn fetch_card_release_versions() -> Result<HashMap<String, String>> {
    let rows = cargo_query(&[
        ("action", "cargoquery"),
        ("format", "json"),
        ("tables", "items"),
        ("fields", "items.name,items.release_version"),
        (
            "where",
            "items.class_id='DivinationCard' AND items._pageName NOT LIKE \"%User:%\"",
        ),
        ("limit", "500"),
    ])
    .await?;
    let mut map = HashMap::new();
    for row in &rows {
        let title = &row["title"];
        let name = decode_html_entities(title["name"].as_str().unwrap_or_default());
        let version = title["release version"]
            .as_str()
            .or_else(|| title["release_version"].as_str())
            .unwrap_or_default()
            .to_string();
        if !name.is_empty() && !version.is_empty() {
            map.insert(name, version);
        }
    }
    Ok(map)
}

pub fn match_league(
    card_version: &str,
    leagues: &[LeagueReleaseInfo],
) -> Option<LeagueReleaseInfo> {
    let parts: Vec<&str> = card_version.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    leagues
        .iter()
        .find(|l| {
            let lp: Vec<&str> = l.version.as_str().split('.').collect();
            lp.len() >= 2 && lp[0] == parts[0] && lp[1] == parts[1]
        })
        .cloned()
}

/// Decodes numeric HTML entities (`&#NNN;`) in a string. Named entities are left as-is.
pub fn decode_html_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' && chars.peek() == Some(&'#') {
            chars.next();
            let mut num = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            if chars.next() == Some(';')
                && let Ok(cp) = num.parse::<u32>()
                && let Some(c) = char::from_u32(cp)
            {
                out.push(c);
                continue;
            }
            out.push('&');
            out.push('#');
            out.push_str(&num);
        }
        out.push(c);
    }
    out
}

async fn cargo_query(params: &[(&str, &str)]) -> Result<Vec<serde_json::Value>> {
    let url = reqwest::Url::parse_with_params(crate::WIKI_API_URL, params)?;
    let resp = crate::HTTP_CLIENT
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;
    resp["cargoquery"]
        .as_array()
        .cloned()
        .context("cargoquery response missing or not an array")
}
