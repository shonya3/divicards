use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapBoss {
    pub name: String,
    pub maps: Vec<String>,
}

#[cfg(feature = "fs_cache_fetcher")]
pub async fn fetch() -> Result<Vec<MapBoss>, crate::error::Error> {
    use crate::consts::MAPBOSSES_POEWIKI_URL;
    use playwright::Playwright;

    let script = format!(
        "() => {{{} return fetchMapBosses()}}",
        include_str!("../fetchMapBosses.js")
    );
    let playwright = Playwright::initialize().await.unwrap();
    playwright.install_chromium().unwrap();
    let chrome = playwright.chromium();
    let browser = chrome.launcher().headless(false).launch().await.unwrap();
    let context = browser
        .context_builder()
        .clear_user_agent()
        .build()
        .await
        .unwrap();
    let page = context.new_page().await.unwrap();
    page.goto_builder(MAPBOSSES_POEWIKI_URL)
        .goto()
        .await
        .unwrap();
    page.wait_for_selector_builder("tbody")
        .wait_for_selector()
        .await
        .unwrap();
    let bosses: Vec<MapBoss> = page.eval(&script).await.unwrap();
    Ok(bosses)
}
