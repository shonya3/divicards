use divi::{
    prices::Prices,
    sample::{Input, NameAmount, Sample},
};
fn main() -> Result<(), divi::error::Error> {
    let s1 = Sample::create(
        Input::NameAmountPairs(vec![
            NameAmount::new(String::from("Rain of Chaos"), 25),
            NameAmount::new(String::from("The Doctor"), 1),
        ]),
        Some(Prices::default()),
    )?;

    let json = r#"[
        {"name": "Rain of Chaos", "amount": 30},
        {"name": "Not really a card", "amount": 1}
    ]"#;
    let vec: Vec<NameAmount> = serde_json::from_str(json)?;
    let s2 = Sample::create(Input::NameAmountPairs(vec), None)?;

    let merged = Sample::merge(Some(Prices::default()), &[s1, s2.clone()])?;

    assert_eq!(s2.not_cards, vec![String::from("Not really a card")]);
    assert_eq!(merged.cards.get("Rain of Chaos").unwrap().amount, 55);

    Ok(())
}
