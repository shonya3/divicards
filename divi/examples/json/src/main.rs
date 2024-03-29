use divi::{
    prices::Prices,
    sample::{CardNameAmount, DivinationCardsSample, SampleData},
};
fn main() -> Result<(), divi::error::Error> {
    let s1 = DivinationCardsSample::create(
        SampleData::CardNameAmountList(vec![
            CardNameAmount::new(String::from("Rain of Chaos"), 25),
            CardNameAmount::new(String::from("The Doctor"), 1),
        ]),
        Some(Prices::default()),
    )?;

    let json = r#"[
        {"name": "Rain of Chaos", "amount": 30},
        {"name": "Not really a card", "amount": 1}
    ]"#;
    let vec: Vec<CardNameAmount> = serde_json::from_str(json)?;
    let s2 = DivinationCardsSample::create(SampleData::CardNameAmountList(vec), None)?;

    let merged = DivinationCardsSample::merge(Some(Prices::default()), &[s1, s2.clone()]);

    assert_eq!(s2.not_cards, vec![String::from("Not really a card")]);
    assert_eq!(merged.cards.get_card("Rain of Chaos").amount, 55);

    Ok(())
}
