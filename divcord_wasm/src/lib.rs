mod utils;

use divcord::{Record, Spreadsheet};
use poe_data::PoeData;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn parsed_records(
    spreadsheet: String,
    poe_data: String,
    toast: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let spreadsheet: Spreadsheet = serde_json::from_str(&spreadsheet).unwrap();
    let poe_data: PoeData = serde_json::from_str(&poe_data).unwrap();

    let mut records: Vec<Record> = vec![];
    for record in divcord::records_iter(&spreadsheet, &poe_data) {
        match record {
            Ok(record) => records.push(record),
            Err(err) => {
                toast
                    .call1(
                        &JsValue::null(),
                        &JsValue::from_str(err.to_string().as_str()),
                    )
                    .unwrap();
            }
        }
    }

    Ok(serde_wasm_bindgen::to_value(&records).unwrap())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn find_cards_by_source_types(types: JsValue, records: JsValue, poe_data: JsValue) -> JsValue {
    set_panic_hook();
    let poe_data: PoeData = serde_wasm_bindgen::from_value(poe_data).unwrap();

    let types: Vec<String> = serde_wasm_bindgen::from_value(types).unwrap();
    let records: Vec<Record> = serde_wasm_bindgen::from_value(records).unwrap();

    let cards = divcord::cards::find_cards_by_source_types(&types, &records, &poe_data);

    serde_wasm_bindgen::to_value(&cards).unwrap()
}
