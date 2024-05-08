#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]

//! A library for parsing sets of divination cards(samples) and calculating the real weight for each card
//!
//!
//!```
//!use divi::{
//!    TradeLeague,
//!    prices::Prices,
//!    sample::{Sample, Input},
//!};
//!
//!#[tokio::main]
//!async fn main() -> Result<(), divi::error::Error> {
//!    let prices = Prices::fetch(&TradeLeague::Standard).await?;
//!    let csv = r#"name,amount
//!    The Doctor,2
//!    Rain of Chaos,30"#;
//!    let sample = Sample::create(Input::Csv(String::from(csv)), Some(prices))?;
//!    // output: The Doctor: DivinationCardRecord { name: "The Doctor", amount: 2, price: Some(869.1), sum: Some(1738.2), weight: Some(2090.8254) }
//!    println!("The Doctor: {:?}", sample.cards.get_card("The Doctor"));
//!    Ok(())
//!}
//! ```
//!
//! ##Example
//! Prepare the sample for sending to Google Sheets
//!```
//!use std::fs::read_to_string;
//!
//!use divi::{
//!    prices::Prices,
//!    sample::{Column, Sample, Order, Input, TablePreferences},
//!};
//!
//!fn main() -> Result<(), divi::error::Error> {
//!let csv = read_to_string("examples/sample.csv").expect("Could not read sample.csv");
//!    let sample = Sample::create(Input::Csv(csv), Some(Prices::default()))?;
//!
//!    let preferences = TablePreferences {
//!        columns: vec![
//!            Column::Name,
//!            Column::Amount,
//!            Column::Weight,
//!            Column::Price,
//!            Column::Sum,
//!        ],
//!        ordered_by: Column::Amount,
//!        order: Order::Desc,
//!        cards_must_have_amount: false,
//!        min_price: 0.
//!    };
//!
//!    let values = sample.into_serde_values(Some(preferences));
//!    let json = serde_json::to_string(&values)?;
//!    dbg!(json);
//!
//!    Ok(())
//!}

pub mod card_record;
pub mod cards;
pub mod consts;
pub mod error;
pub mod prices;
pub mod sample;

pub use crate::{
    card_record::CardRecord,
    cards::{check_card_name, Cards, CheckCardName, GetRecordMut},
    consts::{CARDS, CONDENSE_FACTOR, LEGACY_CARDS},
    error::Error,
    prices::{DivinationCardPrice, Prices},
    sample::{CardNameAmount, Column, Input, Order, Sample, TablePreferences},
};
pub use poe::league::{League, TradeLeague};

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
        sample::{Input, Sample},
    };

    #[test]
    fn calc_real_rain_of_chaos_weight() {
        let sample = Sample::create(
            Input::Csv(String::from("name,amount\rRain of Chaos,1")),
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
        let sample = Sample::create(
            Input::Csv(String::from("name,amount\rRain of Chaos,2\rThe Doctor,1")),
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
        let data = Input::Csv(fs::read_to_string("examples/example-2.csv").unwrap());
        let sample = Sample::create(data, None).unwrap();

        let fox = sample.cards.get_card("The Fox in the Brambles");
        assert_eq!(557.44556, fox.weight.unwrap());
    }
}

#[cfg(test)]
mod fix_typos {
    use crate::sample::{Input, Sample};
    use std::fs;
    #[test]
    fn fix_typos() {
        let sample_data = Input::Csv(fs::read_to_string("examples/example-3.csv").unwrap());
        let sample = Sample::create(sample_data, None).unwrap();
        assert_eq!(sample.fixed_names.len(), 26);
    }
}
