use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActArea {
    pub id: ActAreaId,
    pub name: String,
    pub act: u8,
    pub area_level: u8,
    pub image_url: String,
    pub poedb_image_url: String,
    pub has_waypoint: bool,
    pub has_labyrinth_trial: bool,
    pub is_town: bool,
    pub bossfights: Vec<Bossfight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ActAreaId(String);
impl ActAreaId {
    pub const fn new(act_area_id: String) -> Self {
        Self(act_area_id)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bossfight {
    pub name: String,
}

impl ActArea {
    pub fn act_url(act: u8) -> String {
        format!("https://poedb.tw/us/Act_{act}")
    }
}
