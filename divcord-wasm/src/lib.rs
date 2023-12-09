mod utils;

use divcord::table::{table_record::SourcefulDivcordTableRecord, DivcordTable};
use poe_data::PoeData;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn parsed_records(
    divcord_table: JsValue,
    poe_data: JsValue,
    toast: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let table: DivcordTable = serde_wasm_bindgen::from_value(divcord_table).unwrap();
    let poe_data: PoeData = serde_wasm_bindgen::from_value(poe_data).unwrap();

    let mut records: Vec<SourcefulDivcordTableRecord> = vec![];
    for record in table.sourceful_records_iter(poe_data) {
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
