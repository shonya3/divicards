use googlesheets::{sheet::SheetUrl, Values};

use crate::{
    error::Error,
    google::{AccessTokenStorage, Persist},
};

#[tauri::command]
pub async fn add_sheet_with_values(
    spreadsheet_id: &str,
    title: &str,
    values: Values,
) -> Result<SheetUrl, Error> {
    let url = googlesheets::add_sheet_with_values(
        spreadsheet_id,
        title,
        values,
        &AccessTokenStorage::new().get().unwrap(),
    )
    .await?;
    Ok(url)
}
