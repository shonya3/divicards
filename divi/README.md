# divi

A library for parsing sets of divination cards(samples) and calculating the real weight for each card

## Example

Add divi and Tokio to your dependencies:

```toml
tokio = { version = "1", features = ["full"] }
divi = {path = "../../"}
```

And then get started in your `main.rs`:

```rust
use divi::{
    league::TradeLeague,
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
};

#[tokio::main]
async fn main() -> Result<(), divi::error::Error> {
    let prices = Prices::fetch(&TradeLeague::Standard).await?;
    let csv = r#"name,amount
    The Doctor,2
    Rain of Chaos,30"#;
    let sample = DivinationCardsSample::create(SampleData::Csv(String::from(csv)), Some(prices))?;
    // output: The Doctor: DivinationCardRecord { name: "The Doctor", amount: 2, price: Some(869.1), sum: Some(1738.2), weight: Some(2090.8254) }
    println!("The Doctor: {:?}", sample.cards.get_card("The Doctor"));
    Ok(())
}
```

## Example

Prepare the sample for sending to Google Sheets

```rust
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
```
