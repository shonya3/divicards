use std::fs::read_to_string;

use divi::{
    prices::Prices,
    sample::{Column, DivinationCardsSample, Order, SampleData, TablePreferences},
};

fn main() -> Result<(), divi::error::Error> {
    let csv = read_to_string("sample.csv").expect("Could not read sample.csv");
    let sample = DivinationCardsSample::create(SampleData::Csv(csv), Some(Prices::default()))?;

    let preferences = TablePreferences {
        columns: vec![
            Column::Name,
            Column::Amount,
            Column::Weight,
            Column::Price,
            Column::Sum,
        ],
        ordered_by: Column::Amount,
        order: Order::Desc,
        cards_must_have_amount: false,
        min_price: 0.0,
    };

    let values = sample.into_serde_values(Some(preferences));
    let json = serde_json::to_string(&values)?;
    dbg!(json);

    Ok(())
}
