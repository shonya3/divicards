// #![allow(unused)]

use consts::{CARDS, LEGACY_CARDS};
pub mod card_record;
pub mod cards;
pub mod consts;
pub mod error;
pub mod league;
pub mod prices;
pub mod sample;

impl IsCard for &str {
    fn is_card(&self) -> bool {
        CARDS.contains(self)
    }

    fn is_legacy_card(&self) -> bool {
        LEGACY_CARDS.contains(self)
    }
}

pub trait IsCard {
    fn is_card(&self) -> bool;
    fn is_legacy_card(&self) -> bool;
}

// #[cfg(test)]
// mod tests {
//     use std::fs;

//     use crate::{
//         prices::Prices,
//         sample::{DivinationCardsSample, SampleData},
//     };

//     #[test]
//     fn fix_typos() {
//         let prices: Prices =
//             serde_json::from_str(&fs::read_to_string(&"Crucible-prices.json").unwrap()).unwrap();

//         let sample_data = SampleData::Csv(fs::read_to_string("example-3.csv").unwrap());
//         let sample = DivinationCardsSample::create(sample_data, Some(prices)).unwrap();
//         assert_eq!(sample.fixed_names.len(), 26);
//     }
// }
