use std::fs::read_to_string;

use divi::{
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
};

fn main() -> Result<(), divi::error::Error> {
    let simple_sample = DivinationCardsSample::create(
        SampleData::Csv(String::from("name,amount\rRain of Chaos,2\rThe Doctor,1")),
        None,
    )?;

    let csv_from_file = read_to_string("example-2.csv").unwrap();
    let sample_from_file = DivinationCardsSample::create(
        SampleData::Csv(String::from(csv_from_file)),
        Some(Prices::default()),
    )?;

    let merged =
        DivinationCardsSample::merge(Some(Prices::default()), &[simple_sample, sample_from_file]);

    let rain_of_chaos = merged.cards.get_card("Rain of Chaos").to_owned();
    println!("Rain of Chaos amount: {}", rain_of_chaos.amount);

    Ok(())
}
