use serde_json::Value;
use wasm_bindgen::prelude::*;

pub use divi::{
    card_record::CardRecord,
    cards::FixedCardName,
    consts::{CARDS, LEGACY_CARDS},
    error::Error,
    sample::{Input, NameAmount, Sample},
};

fn is_card(name: &str) -> bool {
    CARDS.contains(&name)
}

fn extract_cards_from_tab(tab: &Value) -> Vec<NameAmount> {
    let items = tab.get("items").and_then(|v| v.as_array());

    match items {
        Some(items) => items
            .iter()
            .filter_map(|item| {
                let name = item.get("baseType")?.as_str()?.to_string();
                let amount = item.get("stackSize")?.as_u64()? as u32;

                if is_card(&name) {
                    Some(NameAmount { name, amount })
                } else {
                    None
                }
            })
            .collect(),
        None => vec![],
    }
}

#[wasm_bindgen]
pub fn create_sample_from_csv(csv: &str) -> Result<JsValue, JsValue> {
    let input = Input::Csv(csv.to_string());
    let sample = Sample::create(input, None).map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&sample).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn create_sample_from_name_amount(pairs: JsValue) -> Result<JsValue, JsValue> {
    let pairs: Vec<NameAmount> =
        serde_wasm_bindgen::from_value(pairs).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let input = Input::NameAmountPairs(pairs);
    let sample = Sample::create(input, None).map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&sample).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn create_sample_from_tab(tab: JsValue) -> Result<JsValue, JsValue> {
    let tab: Value =
        serde_wasm_bindgen::from_value(tab).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let cards = extract_cards_from_tab(&tab);
    let input = Input::NameAmountPairs(cards);
    let sample = Sample::create(input, None).map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&sample).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn _prices_from_league(_league: &str) -> Result<JsValue, JsValue> {
    // This would need async handling in WASM
    // For now, return error indicating it's async
    Err(JsValue::from_str(
        "Use prices_fetch for async price loading",
    ))
}
