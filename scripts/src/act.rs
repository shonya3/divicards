use std::{env, fs, path::Path};

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

    pub async fn download_images() {
        // for act in 1..=10 {}

        let act = 1;

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
            let name_element = name_column.query_selector("a").await.unwrap().unwrap();
            name_element.hover_builder().goto().await.unwrap();
            // page.wait_for_timeout(2000.0).await;
            let a = page
                .wait_for_selector_builder("div.tippy-content[data-state=visible]:has(img)")
                .wait_for_selector()
                .await
                .unwrap()
                .unwrap();
            dbg!(a);
        }
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

                let name_element_ref = name_column.select(&selector("a")).next().unwrap();
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

pub struct Row<'a>(ElementRef<'a>);
impl<'a> Row<'a> {}

// #[tokio::main]
// pub async fn main() {
//     let areas = ActArea::_load_areas_from_file("files/areas.json");
//     dbg!(areas);
// }

/// let areas = ActArea::_load_areas_from_file("files/areas.json");
///     for area in ACT_AREA_NAMES {
///        find_by_name(area, &areas)
///     }
pub fn find_by_name(area_name_from_table: &str, _areas: &[ActArea]) {
    let mut split = area_name_from_table.split("(");

    if let Some(name) = split.next() {
        if name.contains("1/2") {
            let name = name.replace("1/2", "");
            let name = name.trim();
            println!("{}, {}", format!("{name} 1"), format!("{name} 2"))
        }

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

                println!("{acts} {left} {right}");
            }
        }
    }
}

pub fn parse_area_name(s: &str) -> Vec<String> {
    if !s.contains("(") && !s.contains("/") {
        return vec![String::from(s)];
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

            let a: Vec<_> = names
                .into_iter()
                .flat_map(|n| [format!("{n} (Act {left})"), format!("{n} (Act {right})")])
                .collect();
            return a;
        } else {
            let left = acts
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            return names
                .into_iter()
                .map(|n| format!("{n} (Act {left})"))
                .collect();

            // panic!("There is (), but no / inside it, {names:?}");
        }
    }

    names
}

// fn area_id_from_str(s: &str) {}
