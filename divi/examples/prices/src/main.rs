use divi::{
    league::TradeLeague,
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
};

#[tokio::main]
async fn main() -> Result<(), divi::error::Error> {
    let prices = Prices::fetch(&TradeLeague::Standard).await?;
    let csv = r"#
        name,amount
        The Doctor, 2
        Rain of Chaos, 30
    #";
    let sample = DivinationCardsSample::create(SampleData::Csv(String::from(csv)), Some(prices))?;
    println!(
        "Total price of The Doctor cards: {}",
        sample.cards.get_card("The Doctor").sum.unwrap_or_default()
    );
    println!(
        "Total price of Rain of Chaos cards: {}",
        sample
            .cards
            .get_card("Rain of Chaos")
            .sum
            .unwrap_or_default()
    );
    Ok(())
}
