use wasm_bindgen::prelude::wasm_bindgen;

use az::parsing::is_keyword;

#[wasm_bindgen(js_name = "isKeyword")]
pub fn js_is_keyword(value: &str) -> bool {
    is_keyword(value)
}
