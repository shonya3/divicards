use playwright::Playwright;
use serde::{Deserialize, Serialize};

use crate::{consts::MAPBOSSES_POEWIKI_URL, error::Error, loader::DataLoader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapBoss {
    pub name: String,
    pub maps: Vec<String>,
}

pub struct BossLoader;

impl BossLoader {
    pub const fn new() -> Self {
        BossLoader
    }
}

#[async_trait::async_trait]
impl DataLoader<Vec<MapBoss>> for BossLoader {
    fn filename(&self) -> &'static str {
        "mapBosses.json"
    }

    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        let script = format!(
            "() => {{{} return fetchMapBosses()}}",
            &std::fs::read_to_string("fetchMapBosses.js").unwrap()
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
}
