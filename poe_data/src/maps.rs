use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub tier: u32,
    pub available: bool,
    pub unique: bool,
    pub icon: String,
}

impl Map {
    pub fn level(&self) -> u32 {
        67 + self.tier
    }
}

#[cfg(feature = "fetch")]
pub mod fetch {
    use super::Map;
    use crate::consts::{POEDB_MAPS_URL, WIKI_API_URL};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MapDataFromWiki {
        pub name: String,
        pub tier: u32,
    }

    pub async fn fetch() -> Result<Vec<Map>, crate::error::Error> {
        let wiki_maps = load_from_wiki().await?;
        let available_maps = load_poedb_non_unique_available_maplist().await?;

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

        let mut maps: Vec<Map> = vec![];
        for map in wiki_maps.into_iter() {
            let unique = !map.name.ends_with(" Map");
            let available = unique || available_maps.contains(&map.name);
            let icon = match get_map_icon_url(&map.name, &page).await {
                Ok(icon) => icon,
                Err(err) => {
                    eprintln!("{err}");
                    err
                }
            };

            maps.push(Map {
                name: map.name,
                tier: map.tier,
                available,
                unique,
                icon,
            })
        }

        Ok(maps)
    }

    async fn load_from_wiki() -> Result<Vec<MapDataFromWiki>, crate::error::Error> {
        #[derive(Deserialize)]
        pub struct WikiResponse {
            pub cargoquery: Vec<Title>,
        }

        #[derive(Deserialize)]
        pub struct Title {
            pub title: MapRecord,
        }

        #[derive(Deserialize)]
        pub struct MapRecord {
            pub name: String,
            pub tier: String,
        }

        let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&smaxage=0&maxage=0&limit=500&tables=maps,items,areas&join_on=items._pageID=maps._pageID,maps.area_id=areas.id&fields=maps.tier,items.name,maps.area_id,maps.area_level,areas.boss_monster_ids,maps.unique_area_id&group_by=items.name&where=items.class_id='Map' AND maps.area_id LIKE '%MapWorlds%'");

        let response: WikiResponse = reqwest::get(url).await?.json().await?;

        Ok(response
            .cargoquery
            .into_iter()
            .map(|title| MapDataFromWiki {
                name: title.title.name,
                tier: title.title.tier.parse().unwrap(),
            })
            .collect::<Vec<MapDataFromWiki>>())
    }

    pub async fn load_poedb_non_unique_available_maplist(
    ) -> Result<Vec<String>, crate::error::Error> {
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

        page.goto_builder(POEDB_MAPS_URL)
            .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
            .goto()
            .await
            .unwrap();

        let maplist = page.query_selector("#MapsList").await.unwrap().unwrap();
        let table = maplist.query_selector("table").await.unwrap().unwrap();
        let tbody = table.query_selector("tbody").await.unwrap().unwrap();
        let rows = tbody.query_selector_all("tr").await.unwrap();
        dbg!(&rows);

        let mut maps: Vec<String> = Vec::new();
        for row in rows {
            let mapname_column = row
                .query_selector_all("td")
                .await
                .unwrap()
                .into_iter()
                .skip(3)
                .next()
                .unwrap();
            let map = mapname_column
                .text_content()
                .await
                .unwrap()
                .unwrap()
                .trim()
                .to_string();
            if !map.is_empty() {
                maps.push(map);
            };
        }

        maps.sort();

        Ok(maps)
    }

    fn poedb_page_url(boss: &str) -> String {
        let name = boss.split("(").next().unwrap().trim();
        let name = name.replace(" ", "_");
        let name = name.replace(",", "%2C");
        format!("https://poedb.tw/us/{name}")
    }

    async fn get_map_icon_url(map: &str, page: &playwright::api::Page) -> Result<String, String> {
        let url = poedb_page_url(&map);
        let Ok(_) = page
            .goto_builder(&url)
            .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
            .goto()
            .await
        else {
            return Err(format!("No page found for {url}"));
        };

        let script_get_icon_url_from_individual_map_page = r#"
        () => {
                const iconUrl = (icon) => `https://web.poecdn.com/image/${icon}.png`
                const {pathname} = new URL(location.href);
                switch (pathname) {
                    case '/us/Lighthouse_Map': {
                        return iconUrl('Art/2DItems/Maps/Atlas2Maps/New/Beacon')
                    };
                    case '/us/Shipyard_Map': {
                        return iconUrl('Art/2DItems/Maps/Atlas2Maps/New/SulphurVents')
                    };
                    case '/us/Acid_Caverns_Map': {
                        return iconUrl('Art/2DItems/Maps/Atlas2Maps/New/SulphurVents')
                    };
                    case '/us/Iceberg_Map': {
                        return iconUrl('Art/2DItems/Maps/Atlas2Maps/New/Iceberg')
                    }
                }
                for(const row of document.querySelector('tbody').querySelectorAll('tr')){
                    const [nameColumn, value] = Array.from(row.querySelectorAll('td'));
                    if (nameColumn.innerText.trim() === "Icon"){
                        const icon = value.innerText.trim();
                        return iconUrl(icon);
                }
            }
        }
        "#;

        match page
            .eval::<String>(script_get_icon_url_from_individual_map_page)
            .await
        {
            Ok(icon) => Ok(icon),
            Err(err) => {
                let s = format!("Could not extract icon from {url} {err}");
                Err(s)
            }
        }
    }
}
