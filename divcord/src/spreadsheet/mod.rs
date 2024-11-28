//! Defines and loads [Divcord Spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

pub mod fetcher;
pub mod record;
pub mod rich;

pub use fetch::fetch_spreadsheet;

#[allow(unused_imports)]
use self::rich::Cell;
use self::{record::Dumb, rich::RichColumn};
use googlesheets::sheet::ValueRange;
use record::ParseDumbError;
use serde::{Deserialize, Serialize};

/// [Divcord Spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)
/// This simple struct consist of api results:
/// - whole sheet in simplest possible form
/// - font styles data for sources column
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spreadsheet {
    pub sheet: ValueRange,
    pub rich_confirmations_new_325: RichColumn,
    pub rich_to_confirm_or_reverify: RichColumn,
}

impl Spreadsheet {
    pub const fn new(
        sheet: ValueRange,
        rich_confirmations_new_325: RichColumn,
        rich_to_confirm_or_reverify: RichColumn,
    ) -> Self {
        Self {
            sheet,
            rich_confirmations_new_325,
            rich_to_confirm_or_reverify,
        }
    }

    /// Load cached data or fetch fresh based on config conditions or if there is no cached data.
    #[cfg(feature = "fetch")]
    pub async fn load() -> Result<Self, fetcher::FetcherError> {
        use crate::spreadsheet::fetcher::SpreadsheetFetcher;
        use fetcher::DataFetcher;
        SpreadsheetFetcher::default().load().await
    }

    /// Fetch fresh spreadsheet data.
    #[cfg(feature = "fetch")]
    pub async fn fetch() -> Result<Self, fetcher::FetcherError> {
        use crate::spreadsheet::fetcher::SpreadsheetFetcher;
        use fetcher::DataFetcher;
        SpreadsheetFetcher::default().fetch().await
    }

    /// iterator over dumb records - initial preparation of data for each spreadsheet row.
    /// Zips each row of simple format with rich format [`Cell`] for sources column(s)
    /// to produce a [`Dumb`]
    pub fn dumb_records(&self) -> impl Iterator<Item = Result<Dumb, ParseDumbError>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_confirmations_new_325.cells())
            .zip(self.rich_to_confirm_or_reverify.cells())
            .enumerate()
            .map(
                |(
                    row_index,
                    ((spreadsheet_row, confirmations_new_325_cell), to_confirm_or_reverify_cell),
                )| {
                    Dumb::create(
                        row_index,
                        spreadsheet_row,
                        confirmations_new_325_cell,
                        to_confirm_or_reverify_cell,
                    )
                },
            )
    }
}

mod fetch {
    use super::{rich::RichColumn, Spreadsheet};
    use googlesheets::sheet::ValueRange;

    pub async fn fetch_spreadsheet() -> Result<Spreadsheet, reqwest::Error> {
        let sheet = fetch_table_sheet().await?;
        let number_of_rows = sheet.values.len();
        let rich_confirmations_new_325 = RichColumn::new(
            fetch_rich_column(crate::parse::RichColumnVariant::Sources)
                .await?
                .sheets,
            number_of_rows,
        );
        let rich_to_confirm_or_reverify = RichColumn::new(
            fetch_rich_column(crate::parse::RichColumnVariant::Verify)
                .await?
                .sheets,
            number_of_rows,
        );

        Ok(Spreadsheet {
            sheet,
            rich_confirmations_new_325,
            rich_to_confirm_or_reverify,
        })
    }

    async fn fetch_table_sheet() -> Result<ValueRange, reqwest::Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let range = format!("{sheet}!A3:Z");

        let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}?key={key}"
    );
        let value_range: ValueRange = reqwest::get(url).await?.json().await?;
        Ok(value_range)
    }

    async fn _fetch_rich_column(letter: char) -> Result<RichColumn, reqwest::Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let column = letter;
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!{column}3:{column}&includeGridData=true&key={key}");
        reqwest::get(url).await?.json().await
    }

    async fn fetch_rich_column(
        variant: crate::parse::RichColumnVariant,
    ) -> Result<RichColumn, reqwest::Error> {
        _fetch_rich_column(variant.column_letter()).await
    }
}
