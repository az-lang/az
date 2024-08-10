use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::IdentifierIncomplete;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "IdentifierIncomplete")]
pub struct JsIdentifierIncomplete(IdentifierIncomplete<TokenOwnedString>);

impl JsIdentifierIncomplete {
    const JS_NAME: &'static str = "IdentifierIncomplete";
}

#[wasm_bindgen(js_class = IdentifierIncomplete)]
impl JsIdentifierIncomplete {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsIdentifierIncomplete, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(position: JsSubstringPosition, string: String) -> Self {
        Self(IdentifierIncomplete {
            position: position.into(),
            string: string.as_str().into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn string(&self) -> String {
        self.0.string.as_ref().into()
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

impl From<JsIdentifierIncomplete> for IdentifierIncomplete<TokenOwnedString> {
    fn from(value: JsIdentifierIncomplete) -> Self {
        value.0
    }
}

impl From<IdentifierIncomplete<TokenOwnedString>> for JsIdentifierIncomplete {
    fn from(value: IdentifierIncomplete<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsIdentifierIncomplete {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsIdentifierIncomplete {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for IdentifierIncomplete<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsIdentifierIncomplete::JS_NAME,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
