//! Reward text resolution for divination cards from poewiki.net.
//!
//! Fetches card descriptions via the MediaWiki API, parses the `|description=`
//! field from wikitext, and converts the inline template markup (e.g.
//! `{{c|unique|{{iil|Headhunter}}}}`) into poe-ninja-style markup (e.g.
//! `<uniqueItem>{Headhunter}`) for downstream HTML generation.

use std::collections::HashMap;


/// Max titles per request. MediaWiki API silently returns 0 pages above this limit.
const BATCH: usize = 50;

#[derive(Debug)]
pub struct RewardInfo {
    pub reward_name: String,
    pub is_unique: bool,
    pub markup: String,
}

async fn fetch_batch(
    chunk: &[String],
) -> anyhow::Result<HashMap<String, RewardInfo>> {
    let titles: Vec<&str> = chunk.iter().map(|s| s.as_str()).collect();
    let resp = crate::HTTP_CLIENT
        .get(crate::WIKI_API_URL)
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("titles", &titles.join("|")),
            ("prop", "revisions"),
            ("rvprop", "content"),
            ("formatversion", "2"),
        ])
        .send()
        .await?
        .error_for_status()?;
    let data: serde_json::Value = resp.json().await?;

    let mut results = HashMap::new();
    if let Some(pages) = data["query"]["pages"].as_array() {
        for page in pages {
            let title = page["title"].as_str().unwrap_or("");
            if title.is_empty() {
                continue;
            }
            let wt = page["revisions"]
                .as_array()
                .and_then(|r| r.first())
                .and_then(|r| r["content"].as_str());
            if let Some(info) = wt.and_then(extract_reward) {
                results.insert(title.to_string(), info);
            }
        }
    }
    Ok(results)
}

pub async fn fetch_all_rewards(names: &[String]) -> anyhow::Result<HashMap<String, RewardInfo>> {
    let batch_count = names.len().div_ceil(BATCH);
    let batches: Vec<_> = names
        .chunks(BATCH)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            async move { fetch_batch(&chunk).await }
        })
        .collect();

    let all_results = futures::future::try_join_all(batches).await?;

    let mut results = HashMap::new();
    for batch in all_results {
        results.extend(batch);
    }
    eprintln!(
        "  rewards (poewiki): {} batches, {} results",
        batch_count,
        results.len()
    );

    Ok(results)
}

fn extract_reward(wt: &str) -> Option<RewardInfo> {
    let desc = extract_description(wt)?;
    if desc.is_empty() || desc == "Disabled" {
        return None;
    }
    let markup = wiki_desc_to_markup(&desc)?;
    let reward_name = markup
        .lines()
        .next()
        .and_then(|l| {
            l.split('}')
                .next()
                .and_then(|s| s.split_once('{'))
                .map(|(_, name)| name.trim().to_string())
        })
        .unwrap_or_default();
    if reward_name.is_empty() {
        return None;
    }
    let reward_name = if reward_name == "Divination Card" {
        "Random Divination Card".to_string()
    } else {
        reward_name
    };
    let is_unique = markup.contains("uniqueItem");

    Some(RewardInfo {
        reward_name,
        is_unique,
        markup,
    })
}

fn extract_description(wt: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?m)^\|description\s*=\s*(.*?)$").ok()?;
    let m = re.find(wt)?;
    let desc = m.as_str().split_once('=')?.1.trim().to_string();
    Some(desc)
}

fn find_matching_brace(text: &str, start: usize) -> Option<usize> {
    if !text[start..].starts_with("{{") {
        return None;
    }
    let mut depth = 1u32;
    let mut i = start + 2;
    let bytes = text.as_bytes();
    while i + 1 < bytes.len() && depth > 0 {
        if bytes[i] == b'{' && bytes[i + 1] == b'{' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'}' && bytes[i + 1] == b'}' {
            depth -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    if depth == 0 { Some(i) } else { None }
}

fn wiki_desc_to_markup(desc: &str) -> Option<String> {
    let mut desc = desc.replace('\n', " ").trim().to_string();
    if desc == "Disabled" {
        return None;
    }

    // Pre-resolve [[links]]
    let re_html = regex::Regex::new(r"\[\[([^]|]+)\|html[^]]*\]\]").unwrap();
    desc = re_html.replace_all(&desc, "$1").to_string();
    let re_link_alias = regex::Regex::new(r"\[\[([^]|]+)\|([^]]+)\]\]").unwrap();
    desc = re_link_alias.replace_all(&desc, "$2").to_string();
    let re_link = regex::Regex::new(r"\[\[([^]]+)\]\]").unwrap();
    desc = re_link.replace_all(&desc, "$1").to_string();

    let mut markup_parts: Vec<String> = Vec::new();
    let re_br = regex::Regex::new(r"<br\s*/?>").unwrap();
    let lines: Vec<&str> = re_br.split(&desc).collect();
    let re_c = regex::Regex::new(r"\{\{c\|([^|]+)\|").unwrap();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens: Vec<String> = Vec::new();
        let mut pos = 0;
        while pos < line.len() {
            if let Some(cap) = re_c.captures(&line[pos..]) {
                let ctype = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
                let content_start = pos + cap.get(0).map(|m| m.end()).unwrap_or(0);
                if let Some(end) = find_matching_brace(line, pos) {
                    let full_inner = &line[content_start..end - 2];
                    let mut content = full_inner.to_string();
                    resolve_inner_braces(&mut content);
                    let content = content.trim();
                    let cls = wiki_type_to_class(ctype);
                    tokens.push(format!("<{}>{{{}}}", cls, content));
                    pos = end;
                    continue;
                }
            }
            pos += 1;
        }
        if !tokens.is_empty() {
            markup_parts.push(tokens.join(""));
        }
    }

    if markup_parts.is_empty() {
        None
    } else {
        Some(markup_parts.join("\n"))
    }
}

fn resolve_inner_braces(content: &mut String) {
    let re_iil = regex::Regex::new(r"(?:iil|il)\|(.+)").unwrap();
    let mut i = 0;
    loop {
        if i + 1 >= content.len() {
            break;
        }
        if let Some(end) = find_matching_brace(content, i) {
            let inner = &content[i + 2..end - 2];
            if let Some(cap) = re_iil.captures(inner) {
                let rest = cap.get(1).unwrap().as_str();
                let resolved = rest
                    .split('|')
                    .find(|p| {
                        let p = p.trim();
                        !p.is_empty() && !p.starts_with("html=")
                    })
                    .unwrap_or("");
                let resolved = resolved.trim().to_string();
                content.replace_range(i..end, &resolved);
                i += resolved.len();
                continue;
            }
        }
        i += 1;
    }
    // Cleanup artifacts
    *content = content.replace("{{", "").replace("}}", "");
    *content = content.replace("[[", "").replace("]]", "");
}

fn wiki_type_to_class(ctype: &str) -> &str {
    match ctype {
        "unique" => "uniqueItem",
        "currency" => "currencyItem",
        "rare" => "rareItem",
        "white" => "whiteItem",
        "magic" => "magicItem",
        "gem" => "gemItem",
        "divination" => "divination",
        "corrupted" => "corrupted",
        "default" => "default",
        "normal" => "normal",
        "enchanted" => "enchanted",
        "augmented" => "augmented",
        "fractured" => "fractured",
        "foulborn" => "default",
        "influenced" => "default",
        _ => ctype,
    }
}
