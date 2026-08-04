pub mod act;
pub mod cards;
pub mod league;
pub mod maps;
pub mod mapbosses;

use self::{act::ActArea, cards::CardsData, mapbosses::MapBoss, maps::Map};
use act::ActAreaId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PoeData {
    pub acts: Vec<ActArea>,
    pub cards: CardsData,
    pub maps: Vec<Map>,
    pub mapbosses: Vec<MapBoss>,
}

impl PoeData {
    pub fn mapboss(&self, name: &str) -> Option<&MapBoss> {
        self.mapbosses
            .iter()
            .find(|map_boss| map_boss.name.to_lowercase() == name.to_lowercase())
    }

    pub fn act_area_id(&self, id: &ActAreaId) -> Option<&ActArea> {
        self.acts.iter().find(|act_area| act_area.id == *id)
    }

    pub fn act_area_name(&self, name: &str) -> Option<&ActArea> {
        self.acts.iter().find(|act_area| act_area.name == name)
    }

    pub fn bosses_of_map(&self, map: &str) -> Vec<&MapBoss> {
        self.mapbosses
            .iter()
            .filter(|map_boss| {
                map_boss
                    .maps
                    .iter()
                    .any(|m| m.to_lowercase() == map.to_lowercase())
            })
            .collect()
    }
}
