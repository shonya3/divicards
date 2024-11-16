//! Defines and loads [Divcord Spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

pub mod load;
pub mod record;
pub mod rich;

#[allow(unused_imports)]
use self::rich::Cell;
use self::{record::Dumb, rich::RichColumn};
use crate::error::Error;
use googlesheets::sheet::ValueRange;
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
    pub async fn load() -> Result<Self, Error> {
        use crate::spreadsheet::load::SpreadsheetFetcher;
        use fetcher::DataFetcher;
        SpreadsheetFetcher::default().load().await
    }

    /// Fetch fresh spreadsheet data.
    #[cfg(feature = "fetch")]
    pub async fn fetch() -> Result<Self, Error> {
        use crate::spreadsheet::load::SpreadsheetFetcher;
        use fetcher::DataFetcher;
        SpreadsheetFetcher::default().fetch().await
    }

    /// iterator over dumb records - initial preparation of data for each spreadsheet row.
    /// Zips each row of simple format with rich format [`Cell`] for sources column(s)
    /// to produce a [`Dumb`]
    pub fn dumb_records(&self) -> impl Iterator<Item = Result<Dumb, Error>> + '_ {
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
        // .map(
        //     |(row_index, (spreadsheet_row, confirmations_new_325_cell))| {
        //         Dumb::create(row_index, spreadsheet_row, confirmations_new_325_cell)
        //     },
        // )
    }
}
