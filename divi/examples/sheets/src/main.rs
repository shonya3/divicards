use std::fs::read_to_string;

use divi::{
    prices::Prices,
    sample::{Column, DivinationCardsSample, Order, SampleData, TablePreferences},
};

fn main() -> Result<(), divi::error::Error> {
    let csv = read_to_string("example-2.csv").unwrap();
    let sample =
        DivinationCardsSample::create(SampleData::Csv(String::from(csv)), Some(Prices::default()))?;

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
    };

    let values = sample.into_values(Some(preferences));
    let json = serde_json::to_string(&values)?;
    dbg!(json);

    Ok(())
}
