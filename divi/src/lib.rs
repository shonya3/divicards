//! A library for parsing sets of divination cards(samples) and calculating the real weight for each card
//!
//!
//!```
//!use divi::{
//!    league::TradeLeague,
//!    prices::Prices,
//!    sample::{DivinationCardsSample, SampleData},
//!};
//!
//!#[tokio::main]
//!async fn main() -> Result<(), divi::error::Error> {
//!    let prices = Prices::fetch(&TradeLeague::Standard).await?;
//!    let csv = r#"name,amount
//!    The Doctor,2
//!    Rain of Chaos,30"#;
//!    let sample = DivinationCardsSample::create(SampleData::Csv(String::from(csv)), Some(prices))?;
//!    // output: The Doctor: DivinationCardRecord { name: "The Doctor", amount: 2, price: Some(869.1), sum: Some(1738.2), weight: Some(2090.8254) }
//!    println!("The Doctor: {:?}", sample.cards.get_card("The Doctor"));
//!    Ok(())
//!}
//! ```
//!
//!
//!
//! Card records in [`DivinationCardsSample`] are stored inside [`Cards`] struct that provides strong types guarantees.
//! It holds an array of card records with length equal to the number of all divination cards.
//! ```rust
//! pub const CARDS_N: usize = 440;
//! pub struct Cards(pub [DivinationCardRecord; CARDS_N]);
//! ```
//! If you need to quickly strip all nullish records, transform [`DivinationCardsSample`] into [`NotNullishSample`]
//!
//! [`DivinationCardsSample`]: crate::sample::DivinationCardsSample
//! [`Cards`]: crate::cards::Cards
//! [`NotNullishSample`]: crate::sample::NotNullishSample
//!
//! ## Example
//! ```rust
//!fn main() -> Result<(), divi::error::Error> {
//!    let csv = r#"name,amount
//!    The Doctor,2
//!    Rain of Chaos,30"#;
//!    let sample = DivinationCardsSample::create(SampleData::Csv(String::from(csv)), None)?;
//!    let sample = sample.into_not_nullish();
//!    println!("cards len: {}", sample.cards.len()); // 2
//!
//!    //name,amount,price,sum,weight
//!    //Rain of Chaos,30,,0.0,121465.99
//!    //The Doctor,2,,0.0,2090.8254
//!    println!("{}", sample.csv);
//!    Ok(())
//!}
//! ```
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
