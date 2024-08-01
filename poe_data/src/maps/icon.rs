use std::sync::Arc;

#[derive(Debug)]
pub enum FetchMapIconError {
    Playwright(Arc<playwright::Error>),
    IconNotFound { map_name: String, poedb_url: String },
    CouldNotOpenPoedbPageForMap { map_name: String, poedb_url: String },
}

impl From<Arc<playwright::Error>> for FetchMapIconError {
    fn from(value: Arc<playwright::Error>) -> Self {
        Self::Playwright(value)
    }
}

/// "Art/2DItems/Maps/Atlas2Maps/New/Arachnid". Wrap in [`poecdn_icon_url`]
/// if you need a url.
pub async fn get_map_icon(
    map: &str,
    page: &playwright::api::Page,
) -> Result<String, FetchMapIconError> {
    let url = poedb_page_url(map);

    if page
        .goto_builder(&url)
        .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .is_err()
    {
        return Err(FetchMapIconError::CouldNotOpenPoedbPageForMap {
            map_name: map.to_owned(),
            poedb_url: url,
        });
    }

    // Iterate each table on page and find table row with first column Icon.
    // Then grab it's value from second column.
    for row in page.query_selector_all("tr").await? {
        let cells = row.query_selector_all("td").await?;
        if let Some(first_cell) = cells.first() {
            let name = first_cell.text_content().await?;
            if let Some(name) = name {
                let name = name.trim();
                if name == "Icon" {
                    if let Some(second_cell) = cells.get(1) {
                        let icon = second_cell.text_content().await?;
                        if let Some(icon) = icon {
                            return Ok(icon.trim().to_owned());
                        }
                    }
                }
            }
        }
    }

    Err(FetchMapIconError::IconNotFound {
        map_name: map.to_owned(),
        poedb_url: url,
    })
}

pub fn poecdn_icon_url(icon: &str) -> String {
    format!("https://web.poecdn.com/image/{icon}.png")
}

fn poedb_page_url(boss: &str) -> String {
    let name = boss.split('(').next().unwrap().trim();
    let name = name.replace(' ', "_");
    let name = name.replace(',', "%2C");
    format!("https://poedb.tw/us/{name}")
}

#[cfg(test)]
#[cfg(feature = "fetch")]
mod tests {
    use playwright::{api::Page, Playwright};

    async fn create_page() -> Page {
        let playwright = Playwright::initialize().await.unwrap();
        let chrome = playwright.chromium();
        let browser = chrome.launcher().headless(true).launch().await.unwrap();
        let context = browser
            .context_builder()
            .clear_user_agent()
            .build()
            .await
            .unwrap();
        context.new_page().await.unwrap()
    }

    #[tokio::test]
    #[cfg(feature = "fetch")]
    async fn map_icon() {
        let page = create_page().await;
        let icon = super::get_map_icon("Arachnid Tomb Map", &page)
            .await
            .unwrap();
        assert_eq!(icon.as_str(), "Art/2DItems/Maps/Atlas2Maps/New/Arachnid")
    }
}
