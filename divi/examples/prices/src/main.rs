use divi::{
    prices::Prices,
    sample::{Input, Sample},
    TradeLeague,
};

#[tokio::main]
async fn main() -> Result<(), divi::error::Error> {
    let prices = Prices::fetch(&TradeLeague::Standard).await?;
    let csv = r#"name,amount
    The Doctor,2
    Rain of Chaos,30"#;
    let sample = Sample::create(Input::Csv(String::from(csv)), Some(prices))?;
    println!(
        "Total price of The Doctor cards: {}",
        sample
            .cards
            .get("The Doctor")
            .unwrap()
            .sum
            .unwrap_or_default()
    );
    println!(
        "Total price of Rain of Chaos cards: {}",
        sample
            .cards
            .get("Rain of Chaos")
            .unwrap()
            .sum
            .unwrap_or_default()
    );
    Ok(())
}
