use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    // #[serde(rename = "chaosValue")]
    // #[serde(rename(serialize = "calculated", deserialize = "chaosValue"))]
    // pub calculated: f32,
}
