use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Modifier {
    Normal,
    #[serde(alias = "whiteitem")]
    WhiteItem,
    #[serde(alias = "magicitem")]
    MagicItem,
    #[serde(alias = "rareitem")]
    RareItem,
    #[serde(alias = "uniqueitem")]
    UniqueItem,
    Fractured,
    Enchanted,
    Default,
    Augmented,
    Corrupted,
    #[serde(alias = "currencyitem")]
    CurrencyItem,
    Divination,
    #[serde(alias = "gemitem")]
    GemItem,
    #[serde(alias = "size:25")]
    Size25,
    #[serde(alias = "size:26")]
    Size26,
    #[serde(alias = "size:27")]
    Size27,
    #[serde(alias = "size:28")]
    Size28,
    #[serde(alias = "size:29")]
    Size29,
    #[serde(alias = "size:30")]
    Size30,
    #[serde(alias = "size:31")]
    Size31,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(&self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HighlightedExpression {
    pub line: usize,
    pub order: usize,
    pub modifier: Modifier,
    pub text: String,
}

impl HighlightedExpression {
    // pub fn as_html(vec: &Vec<HighlightedExpression>) -> String {
    //     for expr in vec {}
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Span(HighlightedExpression);

impl Span {
    pub fn as_html(&self) -> String {
        format!("<span class={}>{}</span>", &self.0.modifier, &self.0.text)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paragraph(Vec<Span>);

impl Paragraph {
    pub fn as_html(&self) -> String {
        let spans_html: String = self.0.iter().map(|s| s.as_html()).collect();
        format!("<p>{spans_html}</p>")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Div(Vec<Paragraph>, Option<Modifier>);

impl Div {
    pub fn as_html(&self) -> String {
        let paragraphs_html: String = self.0.iter().map(|p| p.as_html()).collect();

        let Some(modifier) = self.1.as_ref() else {
            return format!("<div class=reward>{paragraphs_html}</div>");
        };

        format!("<div class='reward {}'>{paragraphs_html}</div>", modifier)
    }
}

impl Div {
    pub fn parse(s: &str) -> Div {
        let re = Regex::new(r"<[^}]+}").unwrap();

        let mut s = String::from(s);
        let mut size: Option<Modifier> = None;

        if s.starts_with("<size:") {
            size = serde_json::from_str(&json!(&s[1..=7]).to_string()).unwrap();
            s = s[10..s.len() - 1].to_string();
        }

        if let Some(size_not_in_start) = s.find("<size:") {
            let len = s.len();
            let (left, _last_char) = s.split_at_mut(len);
            s = String::from(left);
            let _drained = s.drain(size_not_in_start..=size_not_in_start + 9);
        }

        // dbg!(&size);

        let inside_angle_brackets_re = Regex::new(r"<([^>]+)>").unwrap();
        let inside_curly_brackets_re = Regex::new(r"\{([^}]+)\}").unwrap();

        let mut _html = format!(r#"<div class="reward">"#);

        let mut div = Div(vec![], size);
        for (line_n, line) in s.lines().enumerate() {
            let mut paragraph = Paragraph(vec![]);
            for (order_n, exp) in re.find_iter(&line).enumerate() {
                let modifier_str = inside_angle_brackets_re
                    .captures(exp.as_str())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();

                let modifier_str = &modifier_str[1..modifier_str.len() - 1];
                let modifier = serde_json::from_str(&json!(modifier_str).to_string()).unwrap();

                let text = inside_curly_brackets_re
                    .captures(exp.as_str())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();

                let text = &text[1..text.len() - 1];
                // println!("{modifier_str} , {text}");

                paragraph.0.push(Span(HighlightedExpression {
                    line: line_n,
                    modifier,
                    text: text.to_string(),
                    order: order_n,
                }));
            }

            div.0.push(paragraph);
        }

        div
    }
}

pub fn reward_to_html(s: &str) -> String {
    Div::parse(s).as_html()
}
