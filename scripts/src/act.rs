use std::{
    env,
    fs::{self, File},
    path::Path,
};

use playwright::{api::DocumentLoadState, Playwright};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub const TOWN_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelTownPinIcon.webp";
pub const WAYPOINT_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelWaypointIcon.webp";
pub const LABYRINTH_TRIAL_IMAGE_URL: &'static str =
    "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelLabyrinthWaypointPinIcon.webp";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActArea {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub area_level: u8,
    pub image_url: String,
    pub has_waypoint: bool,
    pub has_labyrinth_trial: bool,
    pub is_town: bool,
}

impl ActArea {
    fn act_url(act: u8) -> String {
        format!("https://poedb.tw/us/Act_{act}")
    }

    pub fn image_url(act: u8, id: &str) -> String {
        let (_, area) = id.split_once("_").unwrap();
        format!("https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/Acts/{act}/{area}.webp")
    }

    async fn act_html(act: u8) -> Result<String, Error> {
        Ok(reqwest::get(ActArea::act_url(act)).await?.text().await?)
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
                // let filename = format!("{}.png", card.art_filename);
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

    pub async fn collect_image_urls() -> Vec<String> {
        // for act in 1..=10 {}

        let mut urls: Vec<String> = vec![];

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

        for act in 1..=10 {
            dbg!("doing act {act}");
            page.goto_builder(&ActArea::act_url(act))
                .wait_until(DocumentLoadState::DomContentLoaded)
                .goto()
                .await
                .unwrap();

            // let res: u8 = page
            //     .eval(
            //         r#"
            //         () => {
            //             const tbody = document.querySelector('tbody');
            //             for

            //         }"#,
            //     )
            //     .await
            //     .unwrap();

            // dbg!(res);

            // page.keyboard.press("Escape", Some(2000.0)).await.unwrap();

            let tbody = page.query_selector("tbody").await.unwrap().unwrap();
            let rows = tbody.query_selector_all("tr").await.unwrap();
            for row in rows {
                let columns = row.query_selector_all("td").await.unwrap();
                let name_column = &columns[1];
                for name_element in name_column.query_selector_all("a").await.unwrap() {
                    name_column.query_selector("a").await.unwrap().unwrap();
                    name_element.hover_builder().goto().await.unwrap();
                    let tippy_content = page
                        .wait_for_selector_builder("div.tippy-content[data-state=visible]:has(img)")
                        .wait_for_selector()
                        .await
                        .unwrap()
                        .unwrap();
                    let img = tippy_content
                        .query_selector(".itemboximage img")
                        .await
                        .unwrap()
                        .unwrap();
                    let src = img.get_attribute("src").await.unwrap().unwrap();
                    urls.push(src);
                }
            }
        }

        urls
    }

    pub async fn download_data() -> Vec<Self> {
        let mut areas: Vec<ActArea> = Vec::new();

        // let html = ActArea::act1_html().await;

        for act in 1..=10 {
            println!("doing act {act}");
            let html = ActArea::act_html(act).await.unwrap();
            let html = Html::parse_document(&html);
            for row in html
                .select(&selector("tbody"))
                .next()
                .unwrap()
                .select(&selector("tr"))
            {
                let cols: Vec<ElementRef> = row.select(&selector("td")).collect();
                let act = cols[0]
                    .select(&selector("a"))
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .parse::<u8>()
                    .unwrap();
                let name_column = cols[1];

                for name_element_ref in name_column.select(&selector("a")) {
                    let id = name_element_ref
                        .value()
                        .attr("data-hover")
                        .unwrap()
                        .replace("?t=WorldAreas&c=", "");
                    let name = name_element_ref
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_string();

                    let area_level_column = cols[2];

                    let area_level = area_level_column
                        .first_child()
                        .unwrap()
                        .value()
                        .as_text()
                        .unwrap()
                        .trim()
                        .parse::<u8>()
                        .unwrap();

                    let mut has_waypoint = false;
                    let mut has_labyrinth_trial = false;
                    let mut is_town = false;
                    for img in area_level_column.select(&selector("img")) {
                        if let Some(src) = img.value().attr("src") {
                            match src.trim() {
                                TOWN_IMAGE_URL => is_town = true,
                                WAYPOINT_IMAGE_URL => has_waypoint = true,
                                LABYRINTH_TRIAL_IMAGE_URL => has_labyrinth_trial = true,
                                _ => {}
                            }
                        };
                    }

                    let image_url = ActArea::image_url(act, &id);

                    let area = ActArea {
                        id,
                        name,
                        act,
                        area_level,
                        image_url,
                        has_waypoint,
                        has_labyrinth_trial,
                        is_town,
                    };

                    areas.push(area);
                }
            }
        }
        areas
    }

    async fn _act1_html() -> String {
        let dir = env::current_dir().unwrap().join("files");
        let html_path = dir.join("act1.html");
        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        };

        let html = match std::fs::read_to_string(&html_path) {
            Ok(html) => html,
            Err(_) => {
                let html = reqwest::get(ActArea::act_url(1))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                std::fs::write(&dir.join(&html_path), &html).unwrap();
                html
            }
        };

        html
    }

    fn _write<P: AsRef<Path>>(areas: Vec<Self>, path: P) {
        fs::write(path, serde_json::to_string_pretty(&areas).unwrap()).unwrap();
    }

    pub fn _load_areas_from_file<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let s = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&s).unwrap()
    }
}

pub fn selector(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

// #[tokio::main]
// pub async fn main() {
//     let areas = ActArea::_load_areas_from_file("files/areas.json");
//     dbg!(areas);
// }

/// let areas = ActArea::_load_areas_from_file("files/areas.json");
///     for area in ACT_AREA_NAMES {
///        find_by_name(area, &areas)
///     }

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

    // names
}

// fn area_id_from_str(s: &str) {}
