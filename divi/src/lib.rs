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

#[cfg(test)]
mod weight_tests {
    use std::fs;

    use crate::{
        consts::{CONDENSE_FACTOR, RAIN_OF_CHAOS_WEIGHT},
        prices::Prices,
        sample::{DivinationCardsSample, SampleData},
    };

    #[test]
    fn calc_real_rain_of_chaos_weight() {
        let sample = DivinationCardsSample::create(
            SampleData::Csv(String::from("name,amount\rRain of Chaos,1")),
            Some(Prices::default()),
        )
        .unwrap();

        let card = sample.cards.get_card("Rain of Chaos");
        assert_eq!(
            //121465.99-ish
            RAIN_OF_CHAOS_WEIGHT.powf(1.0 / CONDENSE_FACTOR),
            card.weight.unwrap()
        );
    }

    #[test]
    fn card_weight_in_three_cards_sample() {
        let sample = DivinationCardsSample::create(
            SampleData::Csv(String::from("name,amount\rRain of Chaos,2\rThe Doctor,1")),
            None,
        )
        .unwrap();

        let doctor = sample.cards.get_card("The Doctor");
        assert_eq!(
            //42944.715-ish
            (RAIN_OF_CHAOS_WEIGHT / 2.0).powf(1.0 / CONDENSE_FACTOR),
            doctor.weight.unwrap()
        );
    }

    #[test]
    fn huge_sample() {
        let data = SampleData::Csv(fs::read_to_string("example-2.csv").unwrap());
        let sample = DivinationCardsSample::create(data, None).unwrap();

        let fox = sample.cards.get_card("The Fox in the Brambles");
        assert_eq!(557.44556, fox.weight.unwrap());
    }
}

#[cfg(test)]
mod fix_typos {
    use crate::sample::{DivinationCardsSample, SampleData};
    use std::fs;
    #[test]
    fn fix_typos() {
        let sample_data = SampleData::Csv(fs::read_to_string("example-3.csv").unwrap());
        let sample = DivinationCardsSample::create(sample_data, None).unwrap();
        assert_eq!(sample.fixed_names.len(), 26);
    }
}
