mod utils;

use divcord::{ParseRecordError, Record, Spreadsheet};
use poe_data::PoeData;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn parsed_records(
    spreadsheet: String,
    poe_data: String,
    on_error: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let on_err = |s: &str| {
        on_error
            .call1(&JsValue::null(), &JsValue::from_str(s))
            .unwrap()
    };

    let spreadsheet: Spreadsheet = serde_json::from_str(&spreadsheet).unwrap();
    let poe_data: PoeData = serde_json::from_str(&poe_data).unwrap();

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
                on_err(err.to_string().as_str());
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
pub fn find_cards_by_source_types(types: JsValue, records: JsValue, poe_data: String) -> JsValue {
    set_panic_hook();
    let poe_data: PoeData = serde_json::from_str(&poe_data).unwrap();

    let types: Vec<String> = serde_wasm_bindgen::from_value(types).unwrap();
    let records: Vec<Record> = serde_wasm_bindgen::from_value(records).unwrap();

    let cards = divcord::cards::cards_by_source_types(&types, &records, &poe_data);

    log(&format!("{}", cards.len()));

    serde_wasm_bindgen::to_value(&cards).unwrap()
}

#[wasm_bindgen]
pub fn slug(s: String) -> String {
    slug::slugify(s)
}

#[wasm_bindgen]
pub async fn fetch_spreadsheet() -> Result<JsValue, JsValue> {
    let api_key = env!("GOOGLE_API_KEY");

    match divcord::spreadsheet::fetch_spreadsheet(api_key).await {
        Ok(spreadsheet) => Ok(serde_wasm_bindgen::to_value(&spreadsheet)?),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
