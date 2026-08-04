use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapBoss {
    pub name: String,
    pub maps: Vec<String>,
}
