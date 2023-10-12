use std::{collections::HashMap, fs, path::PathBuf};

use async_trait::async_trait;
use googlesheets::sheet::ValueRange;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::Error,
    loader::DataLoader,
    rich::{DropsFrom, RichSourcesColumn},
    table_record::{CardDropTableRecord, Confidence, GreyNote, RemainingWork},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table(pub Vec<CardDropTableRecord>);
impl Table {
    pub fn parse(rows: &[Vec<Value>]) -> Result<Table, Error> {
        let mut records: Vec<CardDropTableRecord> = Vec::new();
        for row in rows {
            let record = CardDropTableRecord::parse(row)?;
            records.push(record);
        }

        Ok(Table(records))
    }

    pub fn read_file(path: Option<&str>) -> Result<Table, Error> {
        let p = path.unwrap_or("jsons/sheet.json");
        let s = fs::read_to_string(p)?;
        let sheet: ValueRange = serde_json::from_str(&s)?;
        Table::try_from(&sheet)
    }
}

impl TryFrom<&ValueRange> for Table {
    type Error = Error;

    fn try_from(sheet: &ValueRange) -> Result<Self, Self::Error> {
        Table::parse(&sheet.values[2..])
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DivcordTable {
    pub sheet: ValueRange,
    pub rich_sources_column: RichSourcesColumn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivcordTableRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greynote: Option<GreyNote>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_work: Option<RemainingWork>,
    pub drops_from: Vec<DropsFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_disagreements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl DivcordTable {
    pub const fn new(sheet: ValueRange, rich_sources_column: RichSourcesColumn) -> Self {
        Self {
            sheet,
            rich_sources_column,
        }
    }

    pub fn records_by_card(&self) -> Result<HashMap<String, Vec<DivcordTableRecord>>, Error> {
        let mut map: HashMap<String, Vec<DivcordTableRecord>> = HashMap::new();
        for record in self.records() {
            let record = record?;
            map.entry(record.name.clone())
                .and_modify(|vec| vec.push(record.clone()))
                .or_insert(vec![record]);
        }

        Ok(map)
    }

    pub fn records(&self) -> impl Iterator<Item = Result<DivcordTableRecord, Error>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_sources_column.cells())
            .map(|(row, cell)| {
                let greynote = GreyNote::parse(&row[0])?;
                let name = crate::table_record::parse_name(&row[1])?;
                let tag_hypothesis = crate::table_record::parse_string_cell(&row[2]);
                let confidence = Confidence::parse(&row[3])?;
                let remaining_work = RemainingWork::parse(&row[4])?;
                let wiki_disagreements = row
                    .get(6)
                    .map(|val| crate::table_record::parse_string_cell(val))
                    .flatten();
                let sources_with_tag_but_not_on_wiki = row
                    .get(7)
                    .map(|val| crate::table_record::parse_string_cell(val))
                    .flatten();
                let notes = row
                    .get(8)
                    .map(|val| crate::table_record::parse_string_cell(val))
                    .flatten();
                let drops_from = cell.drops_from();

                Ok(DivcordTableRecord {
                    greynote,
                    name,
                    tag_hypothesis,
                    confidence,
                    remaining_work,
                    drops_from,
                    wiki_disagreements,
                    sources_with_tag_but_not_on_wiki,
                    notes,
                })
            })
    }
}

pub struct DivcordTableLoader(reqwest::Client);
impl DivcordTableLoader {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub async fn fetch_rich_sources_column(
        &self,
        client: &reqwest::Client,
    ) -> Result<RichSourcesColumn, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&includeGridData=true&key={key}");
        Ok(client.get(url).send().await?.json().await?)
    }

    pub async fn fetch_table_sheet(&self, client: &reqwest::Client) -> Result<ValueRange, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let range = format!("{sheet}!A3:Z");

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}?key={key}"
        );
        let value_range: ValueRange = client.get(url).send().await?.json().await?;
        Ok(value_range)
    }
}
#[async_trait]
impl DataLoader<DivcordTable> for DivcordTableLoader {
    fn filename(&self) -> &'static str {
        "divcord_table.json"
    }

    async fn fetch(&self) -> Result<DivcordTable, Error> {
        let sheet = self.fetch_table_sheet(&self.0).await?;
        let len = sheet.values.len();

        Ok(DivcordTable {
            sheet: self.fetch_table_sheet(&self.0).await?,
            rich_sources_column: RichSourcesColumn::new(
                self.fetch_rich_sources_column(&self.0).await?.sheets,
                len,
            ),
        })
    }
}
