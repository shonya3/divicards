use std::{
    env,
    fs::{self, File},
    path::Path,
};

use async_trait::async_trait;
use playwright::{api::DocumentLoadState, Playwright};
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
            &std::fs::read_to_string("extractActAreaPopupData.js").unwrap()
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

        let mut act_areas: Vec<ActArea> = Vec::new();

        for act in 1..=10 {
            println!("Doing act {act}");
            page.goto_builder(&ActArea::act_url(act))
                .wait_until(DocumentLoadState::DomContentLoaded)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AreaNameAct {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub act: Option<u8>,
}

impl AreaNameAct {
    pub const fn new(name: String, act: Option<u8>) -> Self {
        Self { name, act }
    }
}

impl From<String> for AreaNameAct {
    fn from(value: String) -> Self {
        AreaNameAct {
            name: value,
            act: None,
        }
    }
}

pub fn parse_area_name(s: &str) -> Vec<AreaNameAct> {
    if !s.contains("(") && !s.contains("/") {
        return vec![AreaNameAct::from(s.to_string())];
    };

    let mut split = s.split("(");

    let name = split.next().expect("No name, {s}");
    let mut names: Vec<String> = Vec::new();

    if name.contains("1/2") {
        let name = name.replace("1/2", "");
        let name = name.trim();
        for n in [1, 2] {
            let name = format!("{name} {n}");
            // println!("PUSHING {name}");
            names.push(name);
        }
    } else {
        names.push(name.to_string());
    }

    let names = match name.contains("1/2") {
        true => {
            let name = name.replace("1/2", "");
            let name = name.trim();
            [1, 2].iter().map(|n| format!("{name} {n}")).collect()
        }
        false => vec![name.trim().to_string()],
    };

    if let Some(acts) = split.next() {
        if acts.contains("/") {
            let (left, right) = acts.split_once("/").unwrap();

            let left = left
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            let right = right
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            // println!("{acts} {left} {right}");

            // for name in &names {
            //     let n1 = format!("{name} {left}");
            //     let n2 = format!("{name} {right}");
            // }

            names
                .into_iter()
                .flat_map(|name| {
                    [
                        AreaNameAct::new(name.clone(), Some(left)),
                        AreaNameAct::new(name, Some(right)),
                    ]
                })
                .collect()
        } else {
            let left = acts
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            names
                .into_iter()
                .map(|name| AreaNameAct::new(name, Some(left)))
                .collect()
        }
    } else {
        names
            .into_iter()
            .map(|name| AreaNameAct::from(name))
            .collect()
    }
}

// fn area_id_from_str(s: &str) {}

pub fn download_images(urls: Vec<String>) -> Result<(), Error> {
    let act_images_dir = env::current_dir()
        .unwrap()
        .join("public")
        .join("images")
        .join("acts");

    if !act_images_dir.exists() {
        std::fs::create_dir_all(&act_images_dir).unwrap();
    }

    tokio::task::spawn_blocking(move || {
        for url in urls {
            let (_, filename) = url.rsplit_once("/").unwrap();
            let path = act_images_dir.join(filename);
            let mut file = File::create(path).unwrap();
            let _ = reqwest::blocking::get(url)
                .unwrap()
                .copy_to(&mut file)
                .unwrap();
        }
    });

    Ok(())
}
