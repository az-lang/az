use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::MismatchedOpenParenthesis;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "MismatchedOpenParenthesis")]
pub struct JsMismatchedOpenParenthesis(MismatchedOpenParenthesis);

impl JsMismatchedOpenParenthesis {
    const JS_NAME: &'static str = "MismatchedOpenParenthesis";
}

#[wasm_bindgen(js_class = MismatchedOpenParenthesis)]
impl JsMismatchedOpenParenthesis {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsMismatchedOpenParenthesis, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(position: &JsSubstringPosition) -> Self {
        Self(MismatchedOpenParenthesis {
            position: position.ref_to(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
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

impl From<JsMismatchedOpenParenthesis> for MismatchedOpenParenthesis {
    fn from(value: JsMismatchedOpenParenthesis) -> Self {
        value.0
    }
}

impl From<MismatchedOpenParenthesis> for JsMismatchedOpenParenthesis {
    fn from(value: MismatchedOpenParenthesis) -> Self {
        Self(value)
    }
}

impl GetJsName for JsMismatchedOpenParenthesis {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsMismatchedOpenParenthesis {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for MismatchedOpenParenthesis {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsMismatchedOpenParenthesis::JS_NAME,
            self.position.try_to_js_string()?
        ))
    }
}
