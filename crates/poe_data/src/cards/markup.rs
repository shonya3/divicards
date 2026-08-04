//! Converts poe-ninja-style markup to HTML for reward display.
//!
//! Handles `<uniqueItem>{Headhunter}` → `<div class=reward><p><span class=uniqueItem>
//! Headhunter</span></p></div>`. Also cleans flavour text by stripping
//! `<size:NN>` and `<smaller>` tags.

pub fn markup_to_html(markup: &str) -> String {
    let mut paragraphs = Vec::new();
    let re = regex::Regex::new(r"<(\w+)>\{([^}]*)\}").unwrap();

    for line in markup.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut spans = Vec::new();
        let mut pos = 0;
        while pos < line.len() {
            if let Some(cap) = re.captures(&line[pos..]) {
                let cls = cap.get(1).unwrap().as_str();
                let text = cap.get(2).unwrap().as_str();
                spans.push(format!("<span class={}>{}</span>", cls, text));
                pos += cap.get(0).unwrap().len();
            } else {
                pos += 1;
            }
        }
        if !spans.is_empty() {
            paragraphs.push(format!("<p>{}</p>", spans.join("")));
        }
    }

    if paragraphs.is_empty() {
        String::new()
    } else {
        format!("<div class=reward>{}</div>", paragraphs.join(""))
    }
}

pub fn clean_flavour_text(raw: &str) -> String {
    let re_size = regex::Regex::new(r"(?s)<(?:size:\d+|smaller)>\{(.*)\}").unwrap();
    let cleaned = if let Some(cap) = re_size.captures(raw) {
        cap.get(1).unwrap().as_str().to_string()
    } else {
        raw.to_string()
    };
    let cleaned = cleaned.replace("\r\n", "\n").replace('\r', "");
    cleaned.trim().to_string()
}
