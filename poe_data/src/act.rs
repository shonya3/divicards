use serde::{Deserialize, Serialize};

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

#[cfg(feature = "fetch")]
pub async fn fetch() -> Result<Vec<ActArea>, crate::error::Error> {
    let js = r#"
        //@ts-check

        /**
         * @typedef {Object} ActArea
         * @property {string} id
         * @property {string} name
         * @property {number} act
         * @property {number} areaLevel
         * @property {string} imageUrl
         * @property {string} poedbImageUrl
         * @property {boolean} hasWaypoint
         * @property {boolean} hasLabyrinthTrial
         * @property {boolean} isTown
         * @property {Bossfight[]} bossfights
         * @property {string} flavourText
         */

        /**
         * @typedef {Object} Bossfight
         * @property {string} name
         * @property {string} url
         */

        /**
         * Running this function in act.rs ActArea::collect_act_data
         * for each act area popup(that appears on hover area's name) with playwright's page.evaluate("actAreaPopup => extractActAreaPopupData(actAreaPopup)", actAreaPopup)
         * https://poedb.tw/us/Act_1
         * @param {HTMLElement} actAreaPopup
         * @returns {ActArea}
         */
        const extractActAreaPopupData = actAreaPopup => {
            const TOWN_IMAGE_URL = 'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelTownPinIcon.webp';
            const WAYPOINT_IMAGE_URL = 'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelWaypointIcon.webp';
            const LABYRINTH_TRIAL_IMAGE_URL =
                'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelLabyrinthWaypointPinIcon.webp';

            const img = actAreaPopup.querySelector('.itemboximage img');
            const stats = document.querySelector('.Stats');
            if (!(img instanceof HTMLImageElement)) throw new Error('no img element');
            if (!stats) throw new Error('no stats element');
            const hasWaypoint = stats.querySelector(`[src="${WAYPOINT_IMAGE_URL}"]`) !== null;
            const isTown = stats.querySelector(`[src="${TOWN_IMAGE_URL}"]`) !== null;
            const hasLabyrinthTrial = stats.querySelector(`[src="${LABYRINTH_TRIAL_IMAGE_URL}"]`) !== null;

            const itemHeader = actAreaPopup.querySelector('.itemHeader');
            if (!(itemHeader instanceof HTMLElement)) throw new Error('no itemHeader element');
            const name = itemHeader.innerText;

            /** @type string | null */
            let id = null;
            /** @type number | null */
            let act = null;
            /** @type number | null */
            let areaLevel = null;
            /** @type Bossfight[] */
            const bossfights = [];
            let flavourText = stats.querySelector('.FlavourText')?.textContent ?? null;
            const poedbImageUrl = img.src;
            const slashIndex = poedbImageUrl.lastIndexOf('/');
            const filename = poedbImageUrl.slice(slashIndex);
            const imageUrl = `/images/acts${filename}`;

            const props = stats.querySelectorAll('.property');
            for (const prop of props) {
                if (!(prop instanceof HTMLElement)) {
                    throw new Error(`==${name}== prop should be HTMLElement`);
                }

                const text = prop.innerText;

                const valueElement = prop.querySelector('.text-type0');
                if (!(valueElement instanceof HTMLElement)) {
                    continue;
                }

                if (text.includes('Id:')) {
                    id = valueElement.innerText;
                }

                if (text.includes('Act: ')) {
                    act = Number(valueElement.innerText);
                }

                if (text.includes('Area Level')) {
                    areaLevel = Number(valueElement.innerText);
                }

                if (text.includes('Boss Fights')) {
                    const span = prop.querySelector('span');
                    if (!span) {
                        continue;
                    }
                    const aElements = Array.from(prop.querySelector('span')?.querySelectorAll('a') ?? []);
                    for (const a of aElements) {
                        bossfights.push({
                            name: a.innerText,
                            url: a.href,
                        });
                    }
                }
            }

            if (!id) throw new Error(`==${name}== No id`);
            if (!act) throw new Error(`==${name}== no act`);
            if (!areaLevel) throw new Error(`==${name}== no area level`);
            if (!flavourText) throw new Error(`==${name}== No flavourText`);

            /** @type ActArea */
            const actArea = {
                id,
                name,
                act,
                areaLevel,
                imageUrl,
                poedbImageUrl,
                hasWaypoint,
                hasLabyrinthTrial,
                isTown,
                bossfights,
                flavourText,
            };

            return actArea;
        };
    "#;

    let script = format!("(el) => {{{js} return extractActAreaPopupData(el)}}");
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
