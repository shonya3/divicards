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
/// - font styles data for two columns: "Wiki Map/Monster Agreements (F) and  Need to verify (H)"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spreadsheet {
    pub sheet: ValueRange,
    pub rich_confirmations_new_325: RichColumn,
    pub rich_sources_column: RichColumn,
    pub rich_verify_column: RichColumn,
}

impl Spreadsheet {
    pub const fn new(
        sheet: ValueRange,
        rich_confirmations_new_325: RichColumn,
        rich_sources_column: RichColumn,
        rich_verify_column: RichColumn,
    ) -> Self {
        Self {
            sheet,
            rich_confirmations_new_325,
            rich_sources_column,
            rich_verify_column,
        }
    }

    #[cfg(feature = "fetch")]
    pub async fn load() -> Result<Self, Error> {
        use crate::spreadsheet::load::SpreadsheetFetcher;
        use fetcher::DataFetcher;
        SpreadsheetFetcher::default().load().await
    }

    /// iterator over dumb records - initial preparation of data for each spreadsheet row.
    /// Zips each row of simple format with rich format [`Cell`] for 'F' and 'H' columns
    /// to produce a [`Dumb`]
    pub fn dumb_records(&self) -> impl Iterator<Item = Result<Dumb, Error>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_confirmations_new_325.cells())
            .zip(self.rich_sources_column.cells())
            .zip(self.rich_verify_column.cells())
            .enumerate()
            .map(
                |(
                    row_index,
                    (((spreadsheet_row, confirmations_new_325_cell), sources_cell), verify_cell),
                )| {
                    Dumb::create(
                        row_index,
                        spreadsheet_row,
                        confirmations_new_325_cell,
                        sources_cell,
                        verify_cell,
                    )
                },
            )
    }
}
