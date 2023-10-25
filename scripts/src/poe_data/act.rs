use std::env::current_dir;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{error::Error, loader::DataLoader};

pub const TOWN_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelTownPinIcon.webp";
pub const WAYPOINT_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelWaypointIcon.webp";
pub const LABYRINTH_TRIAL_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelLabyrinthWaypointPinIcon.webp";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActArea {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub area_level: u8,
    pub image_url: String,
    pub poedb_image_url: String,
    pub has_waypoint: bool,
    pub has_labyrinth_trial: bool,
    pub is_town: bool,
    pub bossfights: Vec<Bossfight>,
    pub flavour_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bossfight {
    pub name: String,
    pub url: String,
}

impl ActArea {
    fn act_url(act: u8) -> String {
        format!("https://poedb.tw/us/Act_{act}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActAreaDivcordNotation(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActAreaName {
    #[serde(untagged)]
    Name(String),
    #[serde(untagged)]
    NameWithAct((String, u8)),
}

pub struct ActsLoader(reqwest::Client);
impl ActsLoader {
    pub const fn new(client: reqwest::Client) -> Self {
        Self(client)
    }
}

#[async_trait]
impl DataLoader<Vec<ActArea>> for ActsLoader {
    fn filename(&self) -> &'static str {
        "acts.json"
    }

    fn reload(&self) -> bool {
        false
    }

    async fn fetch(&self) -> Result<Vec<ActArea>, Error> {
        let script = format!(
            "(el) => {{{} return extractActAreaPopupData(el)}}",
            &std::fs::read_to_string(
                current_dir()
                    .unwrap()
                    .join("src")
                    .join("poe_data")
                    .join("extractActAreaPopupData.js")
            )
            .unwrap()
        );

        let playwright = playwright::Playwright::initialize().await.unwrap();
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

        let mut act_areas: Vec<ActArea> = Vec::new();

        for act in 1..=10 {
            println!("Doing act {act}");
            page.goto_builder(&ActArea::act_url(act))
                .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
                .goto()
                .await
                .unwrap();

            let tbody = page.query_selector("tbody").await.unwrap().unwrap();
            let rows = tbody.query_selector_all("tr").await.unwrap();
            for row in rows {
                let columns = row.query_selector_all("td").await.unwrap();
                let name_column = &columns[1];
                for name_element in name_column.query_selector_all("a").await.unwrap() {
                    let area = name_element.inner_text().await.unwrap();
                    println!("{area}");
                    name_element.hover_builder().goto().await.unwrap();
                    let tippy_content = page
                        .wait_for_selector_builder("div.tippy-content[data-state=visible]:has(img)")
                        .wait_for_selector()
                        .await
                        .unwrap()
                        .unwrap();

                    let a: ActArea = page.evaluate(&script, tippy_content).await.unwrap();
                    act_areas.push(a);
                }
            }
        }

        Ok(act_areas)
    }
}
