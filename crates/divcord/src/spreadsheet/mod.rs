//! Defines and loads [Divcord Spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

pub mod fs_cache_fetcher;
pub mod record;
pub mod rich;

pub use fetch::fetch_spreadsheet;

#[allow(unused_imports)]
use self::rich::Cell;
use self::{record::Dumb, rich::RichColumn};
use googlesheets::sheet::ValueRange;
use record::ParseDumbError;
use serde::{Deserialize, Serialize};

pub const DROPS_COLUMN_LETTER: char = 'G';
pub const DROPS_DATAMINED_COLUMN_LETTER: char = 'H';
pub const DROPS_VERIFY_COLUMN_LETTER: char = 'I';

/// [Divcord Spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)
/// This simple struct consist of api results:
/// - whole sheet in simplest possible form
/// - font styles data for sources column
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spreadsheet {
    pub sheet: ValueRange,
    pub styled_columns: StyledDropsColumns,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyledDropsColumns {
    drops: RichColumn,
    drops_datamined: RichColumn,
    drops_verify: RichColumn,
}

pub struct StyledDropsCells<'a> {
    drops: &'a Cell,
    drops_datamined: &'a Cell,
    drops_verify: &'a Cell,
}

impl Spreadsheet {
    pub const fn new(sheet: ValueRange, styled_columns: StyledDropsColumns) -> Self {
        Self {
            sheet,
            styled_columns,
        }
    }

    /// Load cached data or fetch fresh based on config conditions or if there is no cached data.
    #[cfg(feature = "fs_cache_fetcher")]
    pub async fn load() -> Result<Self, fs_cache_fetcher::FetcherError> {
        use crate::spreadsheet::fs_cache_fetcher::SpreadsheetFetcher;
        use fs_cache_fetcher::DataFetcher;
        SpreadsheetFetcher::default().load().await
    }

    /// Fetch fresh spreadsheet data.
    pub async fn fetch(google_api_key: &str) -> Result<Spreadsheet, reqwest::Error> {
        fetch_spreadsheet(google_api_key).await
    }

    /// iterator over dumb records - initial preparation of data for each spreadsheet row.
    /// Zips each row of simple format with rich format [`Cell`] for sources column(s)
    /// to produce a [`Dumb`]
    pub fn dumb_records(&self) -> impl Iterator<Item = Result<Dumb, ParseDumbError>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.styled_columns.drops.cells())
            .zip(self.styled_columns.drops_datamined.cells())
            .zip(self.styled_columns.drops_verify.cells())
            .enumerate()
            .map(
                |(
                    row_index,
                    (((spreadsheet_row, drops_cell), drops_datamined_cell), drops_verify_cell),
                )| {
                    let cells = StyledDropsCells {
                        drops: drops_cell,
                        drops_datamined: drops_datamined_cell,
                        drops_verify: drops_verify_cell,
                    };

                    Dumb::create(row_index, spreadsheet_row, cells)
                },
            )
    }
}

mod fetch {
    use crate::spreadsheet::{
        StyledDropsColumns, DROPS_COLUMN_LETTER, DROPS_DATAMINED_COLUMN_LETTER,
        DROPS_VERIFY_COLUMN_LETTER,
    };

    use super::{rich::RichColumn, Spreadsheet};
    use googlesheets::sheet::ValueRange;

    /// Fetch fresh spreadsheet data.
    pub async fn fetch_spreadsheet(google_api_key: &str) -> Result<Spreadsheet, reqwest::Error> {
        let sheet = fetch_table_sheet(google_api_key).await?;
        let number_of_rows = sheet.values.len();
        let styled_columns = fetch_styled_columns(google_api_key, number_of_rows).await?;

        Ok(Spreadsheet {
            sheet,
            styled_columns,
        })
    }

    async fn fetch_styled_columns(
        google_api_key: &str,
        number_of_rows: usize,
    ) -> Result<StyledDropsColumns, reqwest::Error> {
        let drops_rich = fetch_rich_column(google_api_key, DROPS_COLUMN_LETTER).await?;
        let drops_datamined_rich =
            fetch_rich_column(google_api_key, DROPS_DATAMINED_COLUMN_LETTER).await?;
        let drops_verify_rich =
            fetch_rich_column(google_api_key, DROPS_VERIFY_COLUMN_LETTER).await?;

        let drops = RichColumn::new(drops_rich.sheets, number_of_rows);
        let drops_datamined = RichColumn::new(drops_datamined_rich.sheets, number_of_rows);
        let drops_verify = RichColumn::new(drops_verify_rich.sheets, number_of_rows);

        Ok(StyledDropsColumns {
            drops,
            drops_datamined,
            drops_verify,
        })
    }

    async fn fetch_table_sheet(google_api_key: &str) -> Result<ValueRange, reqwest::Error> {
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let range = format!("{sheet}!A3:Z");

        let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}?key={google_api_key}"
    );
        let value_range: ValueRange = reqwest::get(url).await?.error_for_status()?.json().await?;
        Ok(value_range)
    }

    async fn fetch_rich_column(
        google_api_key: &str,
        letter: char,
    ) -> Result<RichColumn, reqwest::Error> {
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let column = letter;
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!{column}3:{column}&includeGridData=true&key={google_api_key}");
        reqwest::get(url).await?.error_for_status()?.json().await
    }
}
