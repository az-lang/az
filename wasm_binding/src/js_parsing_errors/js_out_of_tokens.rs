use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::OutOfTokens;

use crate::traits::{GetJsName, TryRefTo, TryToJsString, TryToJson};

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "OutOfTokens")]
pub struct JsOutOfTokens(OutOfTokens);

impl JsOutOfTokens {
    const JS_NAME: &'static str = "OutOfTokens";
}

#[wasm_bindgen(js_class = OutOfTokens)]
impl JsOutOfTokens {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsOutOfTokens, JsValue> {
        value.try_ref_to()
    }

    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(OutOfTokens)
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.try_to_js_string()
    }
}

impl From<JsOutOfTokens> for OutOfTokens {
    fn from(value: JsOutOfTokens) -> Self {
        value.0
    }
}

impl From<OutOfTokens> for JsOutOfTokens {
    fn from(value: OutOfTokens) -> Self {
        Self(value)
    }
}

impl GetJsName for JsOutOfTokens {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsOutOfTokens {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for OutOfTokens {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!("new {}()", JsOutOfTokens::JS_NAME))
    }
}
