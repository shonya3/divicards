mod utils;

use divcord::{ParseRecordError, Record};
use poe_data::PoeData;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// Fetch spreadsheet and parse.
#[wasm_bindgen]
pub async fn fetch_divcord_records(
    poe_data: JsValue,
    on_error: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    set_panic_hook();
    let api_key = env!("GOOGLE_API_KEY");

    let poe_data: PoeData =
        serde_wasm_bindgen::from_value(poe_data).map_err(|err| JsValue::from(err.to_string()))?;
    let spreadsheet = divcord::spreadsheet::fetch_spreadsheet(api_key)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let on_err = |s: &str| {
        on_error
            .call1(&JsValue::null(), &JsValue::from_str(s))
            .unwrap()
    };

    let mut records: Vec<Record> = vec![];
    for record in divcord::records_iter(&spreadsheet, &poe_data) {
        match record {
            Ok(record_result) => {
                records.push(record_result.record);
                if !record_result.errors.is_empty() {
                    let errors_string =
                        ParseRecordError::ParseDropSources(record_result.errors).to_string();
                    on_err(&errors_string);
                }
            }
            Err(err) => {
                on_err(&err.to_string());
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

// #[wasm_bindgen]
// pub fn find_cards_by_source_types(types: JsValue, records: JsValue, poe_data: String) -> JsValue {
//     set_panic_hook();
//     let poe_data: PoeData = serde_json::from_str(&poe_data).unwrap();

//     let types: Vec<String> = serde_wasm_bindgen::from_value(types).unwrap();
//     let records: Vec<Record> = serde_wasm_bindgen::from_value(records).unwrap();

//     let cards = divcord::cards::cards_by_source_types(&types, &records, &poe_data);

//     log(&format!("{}", cards.len()));

//     serde_wasm_bindgen::to_value(&cards).unwrap()
// }

#[wasm_bindgen]
pub fn slug(s: String) -> String {
    slug::slugify(s)
}
