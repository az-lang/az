use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::MismatchedOpenBrace;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "MismatchedOpenBrace")]
pub struct JsMismatchedOpenBrace(MismatchedOpenBrace);

impl JsMismatchedOpenBrace {
    const JS_NAME: &'static str = "MismatchedOpenBrace";
}

#[wasm_bindgen(js_class = MismatchedOpenBrace)]
impl JsMismatchedOpenBrace {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsMismatchedOpenBrace, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(position: &JsSubstringPosition) -> Self {
        Self(MismatchedOpenBrace {
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

impl From<JsMismatchedOpenBrace> for MismatchedOpenBrace {
    fn from(value: JsMismatchedOpenBrace) -> Self {
        value.0
    }
}

impl From<MismatchedOpenBrace> for JsMismatchedOpenBrace {
    fn from(value: MismatchedOpenBrace) -> Self {
        Self(value)
    }
}

impl GetJsName for JsMismatchedOpenBrace {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsMismatchedOpenBrace {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for MismatchedOpenBrace {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsMismatchedOpenBrace::JS_NAME,
            self.position.try_to_js_string()?
        ))
    }
}
