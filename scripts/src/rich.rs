use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RichSourcesColumn {
    sheets: Vec<Sheet>,
}

impl RichSourcesColumn {
    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        let sheet = &self.sheets[0];
        let data = &sheet.data[0];
        let row_data = &data.row_data;
        row_data.iter().map(|row| {
            if row.values.len() > 1 {
                panic!("Expected values array length to be 1");
            };
            &row.values[0]
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub data: Vec<Data>,
    pub properties: Properties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    sheet_id: usize,
    title: String,
    index: usize,
    sheet_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub start_column: Option<usize>,
    pub row_data: Vec<RowData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RowData {
    pub values: Vec<Cell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub effective_format: EffectiveFormat,
    #[serde(alias = "formattedValue")]
    pub text_content: Option<String>,
    pub text_format_runs: Option<Vec<TextFormatRun>>,
}

impl Cell {
    pub fn drops_from(&self) -> Vec<DropsFrom> {
        self.text_fragments()
            .into_iter()
            .flat_map(|t| t.drops_from())
            .collect()
    }

    pub fn italics(&self) -> impl Iterator<Item = DropsFrom> {
        self.drops_from().into_iter().filter(|d| d.styles.italic)
    }

    pub fn text_fragments(&self) -> Vec<Text> {
        let text_format_runs = self.text_format_runs();
        let text_content = self.text_content.clone().unwrap_or_default();
        let cell_styles = self.font_styles();
        match text_format_runs.len() {
            0 => {
                let text = Text {
                    content: text_content,
                    styles: cell_styles.to_owned(),
                };
                if text.content.len() == 0 || text.content.trim() == "n/a" {
                    vec![]
                } else {
                    vec![text]
                }
            }
            2 => {
                let start_indexes = text_format_runs
                    .iter()
                    .map(|r| r.start_index.unwrap_or_default())
                    .collect::<Vec<usize>>();
                let first_range = start_indexes[0]..start_indexes[1];
                let second_range = start_indexes[1]..text_content.len();

                let vec = vec![
                    Text {
                        content: text_content[first_range].to_string(),
                        styles: text_format_runs[0].styles(&cell_styles),
                    },
                    Text {
                        content: text_content[second_range].to_string(),
                        styles: text_format_runs[1].styles(&cell_styles),
                    },
                ];

                if vec[0].styles.italic == true && vec[0].styles.italic == false {
                    panic!("Italic text cannot preceed the normal text");
                }

                vec
            }
            len => panic!("Invalid number of text fragments, expected 0 or 2, got {len}"),
        }
    }

    pub fn has_italics(&self) -> bool {
        match self.text_format_runs {
            Some(ref text_format_runs) => text_format_runs
                .iter()
                .any(|r| r.is_italic(&self.font_styles())),
            None => false,
        }
    }

    pub fn font_styles(&self) -> FontStyles {
        self.effective_format.font_styles.to_owned().into()
    }

    pub fn text_format_runs(&self) -> Vec<TextFormatRun> {
        self.text_format_runs.clone().unwrap_or_else(Vec::new)
    }

    // pub fn text_fragments(&self) {
    //     let runs = self.text_format_runs();
    //     if self.has_more_than_one_italic_fragments() {
    //         panic!("More than one italic fragment is not expected");
    //     }

    //     for italic in self.italics() {}
    // }

    // pub fn italic_preceeds_normal_text(&self) -> bool {
    //     for run in self.text_format_runs() {}
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EffectiveFormat {
    background_color: ProtobufColor,
    #[serde(alias = "textFormat")]
    font_styles: TextFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProtobufColor {
    blue: Option<f32>,
    green: Option<f32>,
    red: Option<f32>,
}

impl ProtobufColor {
    pub fn into_hexcolor(self) -> String {
        let red_frac = self.red.unwrap_or(0.0);
        let green_frac = self.green.unwrap_or(0.0);
        let blue_frac = self.blue.unwrap_or(0.0);

        let red = (red_frac * 255.0).floor() as u8;
        let green = (green_frac * 255.0).floor() as u8;
        let blue = (blue_frac * 255.0).floor() as u8;

        return Self::rgb_to_css_color(red, green, blue);
    }

    pub fn rgb_to_css_color(red: u8, green: u8, blue: u8) -> String {
        let rgb_number = (red as u32) << 16 | (green as u32) << 8 | blue as u32;
        let hex_string = format!("{:06X}", rgb_number);
        return format!("#{}", hex_string);
    }

    pub fn as_hex(&self) -> String {
        self.clone().into_hexcolor()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HexColor(pub String);

impl HexColor {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<ProtobufColor> for HexColor {
    fn from(value: ProtobufColor) -> Self {
        HexColor(value.into_hexcolor())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextFormat {
    #[serde(alias = "foregroundColor")]
    color: ProtobufColor,
    italic: bool,
    strikethrough: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FontStyles {
    pub color: HexColor,
    pub italic: bool,
    pub strikethrough: bool,
}

impl From<TextFormat> for FontStyles {
    fn from(value: TextFormat) -> Self {
        FontStyles {
            color: value.color.into(),
            italic: value.italic,
            strikethrough: value.strikethrough,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Text {
    pub content: String,
    pub styles: FontStyles,
}

impl Text {
    pub fn items(&self) -> impl Iterator<Item = &str> {
        self.content
            .split(";")
            .map(|s| s.trim())
            .filter(|s| *s != "n/a" && !s.is_empty())
    }

    pub fn drops_from(&self) -> Vec<DropsFrom> {
        self.items()
            .map(|i| DropsFrom {
                name: i.to_string(),
                styles: self.styles.to_owned(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DropsFrom {
    pub name: String,
    pub styles: FontStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextFormatRun {
    pub start_index: Option<usize>,
    pub format: Option<Format>,
    pub font_family: Option<String>,
}

impl TextFormatRun {
    pub fn is_italic(&self, parent_styles: &FontStyles) -> bool {
        match &parent_styles.italic {
            true => match &self.format {
                Some(format) => match format.italic {
                    Some(italic) => italic,
                    None => true,
                },
                None => true,
            },
            false => match &self.format {
                Some(format) => match format.italic {
                    Some(italic) => italic,
                    None => false,
                },
                None => false,
            },
        }
    }

    pub fn is_strikethrough(&self, parent_styles: &FontStyles) -> bool {
        match &parent_styles.strikethrough {
            true => match &self.format {
                Some(format) => match format.strikethrough {
                    Some(strikethrough) => strikethrough,
                    None => true,
                },
                None => true,
            },
            false => match &self.format {
                Some(format) => match format.strikethrough {
                    Some(strikethrough) => strikethrough,
                    None => false,
                },
                None => false,
            },
        }
    }

    pub fn styles(&self, parent_styles: &FontStyles) -> FontStyles {
        FontStyles {
            color: parent_styles.color.to_owned(),
            italic: self.is_italic(parent_styles),
            strikethrough: self.is_strikethrough(parent_styles),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
}

async fn _load_drops_column() -> Result<Value, Error> {
    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
    let sheet = "Cards_and_Hypotheses";
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!B3:B&ranges={sheet}!F3:F&includeGridData=true&key={key}");
    // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!B3:B&includeGridData=true&key={key}");
    // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!A:A&ranges={sheet}!F:F&includeGridData=true&key={key}");
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn load_t() -> Result<RichSourcesColumn, Error> {
    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
    let sheet = "Cards_and_Hypotheses";
    // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!B3:B&ranges={sheet}!F3:F&includeGridData=true&key={key}");
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&includeGridData=true&key={key}");
    // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!A:A&ranges={sheet}!F:F&includeGridData=true&key={key}");
    Ok(reqwest::get(url).await?.json().await?)
}
