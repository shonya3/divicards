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
    #[serde(untagged)]
    Size(Size),
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Normal => write!(f, "normal"),
            Modifier::WhiteItem => write!(f, "whiteItem"),
            Modifier::MagicItem => write!(f, "magicItem"),
            Modifier::RareItem => write!(f, "rareItem"),
            Modifier::UniqueItem => write!(f, "uniqueItem"),
            Modifier::Fractured => write!(f, "fractured"),
            Modifier::Enchanted => write!(f, "enchanted"),
            Modifier::Default => write!(f, "default"),
            Modifier::Augmented => write!(f, "augmented"),
            Modifier::Corrupted => write!(f, "corrupted"),
            Modifier::CurrencyItem => write!(f, "currencyItem"),
            Modifier::Divination => write!(f, "divination"),
            Modifier::GemItem => write!(f, "gemItem"),
            Modifier::Size(size) => size.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Size {
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

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Size25 => write!(f, "size25"),
            Size::Size26 => write!(f, "size26"),
            Size::Size27 => write!(f, "size27"),
            Size::Size28 => write!(f, "size28"),
            Size::Size29 => write!(f, "size29"),
            Size::Size30 => write!(f, "size30"),
            Size::Size31 => write!(f, "size31"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HighlightedExpression {
    pub line: usize,
    pub order: usize,
    pub modifier: Modifier,
    pub text: String,
    pub size: Option<Size>,
    pub coordinates: (usize, usize),
}

impl HighlightedExpression {
    pub fn parse(s: &str) -> Vec<HighlightedExpression> {
        let angle_open_ocurrences: Vec<usize> =
            s.match_indices("<").map(|(index, _)| index).collect();
        // dbg!(&angle_open_ocurrences);

        let mut sizes: Vec<(Size, usize, usize)> = vec![];
        let newlines: Vec<usize> = s.match_indices("\n").map(|(index, _)| index).collect();
        // dbg!(&newlines);

        let mut exprs = vec![];

        for (order, open_angle) in angle_open_ocurrences.iter().enumerate() {
            let open_angle = *open_angle;
            let closing_angle = s[open_angle..].find('>').unwrap() + open_angle;
            if let Some(modifier_str) = s.get(open_angle + 1..closing_angle) {
                let modifier: Modifier =
                    serde_json::from_str(&json!(modifier_str).to_string()).unwrap();
                match modifier {
                    Modifier::Size(size) => {
                        let open_curly = s[closing_angle..].find("{").unwrap() + closing_angle;
                        let closing_curly = s[open_curly..].find("}}").unwrap() + open_curly;
                        let contents = s[open_curly + 1..closing_curly + 1].to_string();
                        sizes.push((size, open_curly + 1, closing_curly + 1));
                        dbg!(contents);
                    }
                    modifier => {
                        let line = Self::which_line(open_angle, &newlines);
                        // println!("{line}:  {modifier}");

                        let open_curly = s[closing_angle..].find("{").unwrap() + closing_angle;
                        let closing_curly = s[closing_angle..].find("}").unwrap() + closing_angle;

                        // println!("closing_angle: {closing_angle}");
                        // println!("{open_curly} {closing_curly}");
                        let text = s[open_curly + 1..closing_curly].to_string();

                        // modifiers.push((modifier, open_angle, closing_angle))

                        let expr = HighlightedExpression {
                            line,
                            order,
                            modifier,
                            text,
                            size: None,
                            coordinates: (open_curly, closing_curly),
                        };

                        dbg!(&expr);
                        exprs.push(expr);
                    }
                }
            }
        }

        for expr in exprs.iter_mut() {
            let size = Self::find_expr_size(&expr, &sizes);
            expr.size = size;
        }

        exprs
    }

    fn which_line(start: usize, newline_indexes: &[usize]) -> usize {
        let mut line = 0;

        for newline_index in newline_indexes {
            if start > *newline_index {
                line += 1
            } else {
                break;
            }
        }

        line
    }

    fn find_expr_size(
        expr: &HighlightedExpression,
        sizes: &[(Size, usize, usize)],
    ) -> Option<Size> {
        for (size, start, end) in sizes {
            if expr.coordinates.0 >= *start && expr.coordinates.1 <= *end {
                return Some(size.to_owned());
            }
        }

        None
    }
}

pub fn reward_to_html(s: &str) -> String {
    Div::from(HighlightedExpression::parse(s)).as_html()
}

impl From<Vec<HighlightedExpression>> for Div {
    fn from(mut exprs: Vec<HighlightedExpression>) -> Self {
        exprs.sort_by(|a, b| a.order.cmp(&b.order));
        let number_of_lines = exprs.iter().map(|e| e.line).max().unwrap() + 1;

        let mut div = Div(vec![], None);

        for line in 0..number_of_lines {
            let vec: Vec<Span> = exprs
                .iter()
                .filter(|e| e.line == line)
                .map(|e| Span(e.to_owned()))
                .collect();

            div.0.push(Paragraph(vec))
        }

        div
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Span(pub HighlightedExpression);

impl Span {
    pub fn as_html(&self) -> String {
        dbg!(&self.0.modifier.to_string());
        let classlist = match &self.0.size {
            Some(size) => format!("\"{} {}\"", &self.0.modifier.to_string(), size.to_string()),
            None => self.0.modifier.to_string(),
        };

        format!("<span class={}>{}</span>", classlist, &self.0.text)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paragraph(pub Vec<Span>);

impl Paragraph {
    pub fn as_html(&self) -> String {
        let spans_html: String = self.0.iter().map(|s| s.as_html()).collect();
        format!("<p>{spans_html}</p>")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Div(pub Vec<Paragraph>, pub Option<Modifier>);

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
                    size: None,
                    coordinates: (0, 0),
                }));
            }

            div.0.push(paragraph);
        }

        div
    }
}

// pub fn reward_to_html(s: &str) -> String {
//     Div::parse(s).as_html()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_reward_to_html() {
        for (_index, s) in [
        "<uniqueitem>{Skin of the Loyal}\n<size:26>{<default>{Item Level:} <normal>{25}\n<corrupted>{Two-Implicit}\n<corrupted>{Corrupted}}",
        "<size:30>{<gemitem>{Level 6 Awakened Support Gem}}\n<default>{Quality:} <augmented>{+20%}\n<corrupted>{Corrupted}",
        "<size:29>{<gemitem>{Level 6 Awakened Support Gem}}\n<default>{Quality:} <augmented>{+23%}\n<corrupted>{Corrupted}",
        "<rareitem> {Onyx Amulet}\n<size:25>{<default>{Item Level:} <normal>{85}\n<default>{Quality:} <augmented>{+20%}\n<default>{Influenced Item}\n<enchanted>{Four Anointments}\n<enchanted>{Incubating Talisman Item}\n<corrupted>{Corrupted}}",
        "<size:27>{<uniqueitem>{The Eternity Shroud}\n<corrupted>{Two-Implicit}\n<corrupted>{Corrupted}}",
        "<size:30>{<magicitem>{Diamond Ring of Redemption}\n<default>{Item Level:} <normal>{100}\n<default>{Redeemer Item}}",
        "<size:31>{<magicitem>{Merciless Two-Hand Weapon}\n<default>{Item Level:} <normal>{100}}",
        "<uniqueitem>{Helmet}\n<size:28>{<enchanted>{Eternal Labyrinth Enchantment}}",
        "<size:29>{<uniqueitem>{Attribute Transforming Jewel}}\n<size:29>{<corrupted>{Corrupted}}",
        "<size:28>{<rareitem>{Elder Guardian Occupied Map}\n<default>{Map Tier:} <normal>{14 to 16}\n<default>{Modifiers:} <normal>{8}\n<corrupted>{Corrupted}}",
        "<size:31>{<gemitem>{Support Gem}\n<default>{Quality:} <augmented>{+23%}\n<corrupted>{Corrupted}}",
        "<size:31>{<magicitem>{Merciless One-Hand Weapon}\n<default>{Item Level:} <normal>{100}}",
        "<size:31>{<magicitem>{Bloodthirsty Eternal Sword}\n<default>{Item Level:} <normal>{66}}",
        "<size:26>{<rareitem>{Map}}\n<size:26>{<default>{Map Tier:} <normal>{13}\n<default>{Quality:} <augmented>{+13%}\n<default>{Delirium:} <normal>{100%}\n<default>{Modifiers:} <normal>{8}\n<corrupted>{Corrupted}}"
    ].iter().enumerate(){
//    std::fs::write(std::env::current_dir().unwrap().join("html").join(format!("reward{index}.html")), reward_to_html(s)).unwrap();
            reward_to_html(s);
        }
    }
}
