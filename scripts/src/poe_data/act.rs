use std::{
    env::{self, current_dir},
    fs::File,
};

use async_trait::async_trait;
use playwright::{api::DocumentLoadState, Playwright};
use serde::{Deserialize, Serialize};

use crate::{error::Error, loader::DataLoader, table::rich::DropsFrom};

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

pub fn parse_act_areas(drops_from: &DropsFrom, acts: &[ActArea], min_level: u8) -> Vec<String> {
    if !drops_from.styles.italic {
        panic!("Act areas should be italic");
    }

    let s = &drops_from.name;
    let names = match is_act_notation(s) {
        true if s == "The Belly of the Beast (A4/A9)" => vec![
            ActAreaName::NameWithAct(("The Belly of the Beast Level 1".to_string(), 4)),
            ActAreaName::NameWithAct(("The Belly of the Beast Level 1".to_string(), 4)),
            ActAreaName::NameWithAct(("The Belly of the Beast".to_string(), 9)),
        ],
        true => parse_act_notation(s),
        false => vec![ActAreaName::Name(s.to_owned())],
    };

    names
        .iter()
        .flat_map(|name| find_ids(&name, acts, min_level))
        .collect()
}

pub fn is_act_notation(s: &str) -> bool {
    match s {
        s if s.contains("(") && s.contains(")") => true,
        s if s.contains("1/2") => true,
        _ => false,
    }
}

pub fn parse_act_notation(s: &str) -> Vec<ActAreaName> {
    if !s.contains("(") && !s.contains("/") {
        panic!("Expected act notation, got {s}");
    };

    let mut split = s.split("(");

    let name = split.next().expect("No name, {s}");
    let mut names: Vec<String> = Vec::new();

    if name.contains("1/2") {
        let name = name.replace("1/2", "");
        let name = name.trim();
        for n in [1, 2] {
            let name = format!("{name} {n}");
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

            names
                .into_iter()
                .flat_map(|name| {
                    [
                        ActAreaName::NameWithAct((name.clone(), left)),
                        ActAreaName::NameWithAct((name, right)),
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
                .map(|name| ActAreaName::NameWithAct((name, left)))
                .collect()
        }
    } else {
        names
            .into_iter()
            .map(|name| ActAreaName::Name(name))
            .collect()
    }
}

pub fn find_ids(name: &ActAreaName, acts: &[ActArea], min_level: u8) -> Vec<String> {
    match name {
        ActAreaName::Name(name) => acts
            .iter()
            .filter(|a| &a.name == name && a.is_town == false && a.area_level >= min_level)
            .map(|a| a.id.to_owned())
            .collect(),
        ActAreaName::NameWithAct((name, act)) => {
            let mut v = vec![];
            if let Some(a) = acts
                .iter()
                .find(|a| &a.name == name && &a.act == act && a.area_level >= min_level)
            {
                v.push(a.id.to_owned())
            };

            v
        }
    }
}

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
