use std::fs::read_to_string;

use divi::{
    prices::Prices,
    sample::{Column, Input, Order, Sample, TablePreferences},
};

#[tokio::main]
async fn main() -> Result<(), divi::error::Error> {
    let simple_sample = Sample::create(
        Input::Csv(String::from("name,amount\rRain of Chaos,2\rThe Doctor,1")),
        None,
    )?;

    let csv_from_file = read_to_string("example-2.csv").unwrap();
    let sample_from_file = Sample::create(Input::Csv(csv_from_file), Some(Prices::default()))?;

    let prices = Prices::fetch(&divi::TradeLeague::Standard).await?;
    let merged = Sample::merge(Some(prices), &[simple_sample, sample_from_file]);

    let rain_of_chaos = merged.cards.get_card("Rain of Chaos").to_owned();
    println!("Rain of Chaos amount: {}", rain_of_chaos.amount);

    let preferences = TablePreferences {
        columns: vec![Column::Name, Column::Amount, Column::Weight],
        ordered_by: Column::Amount,
        order: Order::Desc,
        cards_must_have_amount: true,
        min_price: 200.,
    };
    let csv = merged.into_csv(Some(preferences))?;

    // uncomment and write to file
    // std::fs::write("sample.csv", &csv).unwrap();

    println!("{csv}");

    Ok(())
}
