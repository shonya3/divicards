use crate::{dropsource::id::Identified, spreadsheet::record::Record};
use poe_data::{mapbosses::MapBoss, PoeData};

pub fn bosses_in_map(map: &str, poe_data: &PoeData) -> Vec<MapBoss> {
    let Some(map) = poe_data.maps.iter().find(|m| m.name == map) else {
        return vec![];
    };

    poe_data
        .mapbosses
        .iter()
        .filter(|boss| boss.maps.contains(&map.name))
        .map(|b| b.to_owned())
        .collect::<Vec<_>>()
}

pub fn cards_by_mapboss(boss: &str, records: &[Record], poe_data: &PoeData) -> Vec<String> {
    let mut cards: Vec<String> = vec![];
    let Some(_) = poe_data.mapbosses.iter().find(|b| b.name == boss) else {
        return cards;
    };

    for record in records {
        if record.sources.iter().any(|s| s.id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}

pub fn cards_by_actboss(boss: &str, records: &[Record]) -> Vec<String> {
    let mut cards: Vec<String> = vec![];

    for record in records {
        if record.sources.iter().any(|s| s.id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}
