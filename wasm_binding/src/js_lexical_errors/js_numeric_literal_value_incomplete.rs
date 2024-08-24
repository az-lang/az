use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralValueIncomplete;

use crate::js_numeric_literal_value_kind::JsNumericLiteralValueKind;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteralValueIncomplete")]
pub struct JsNumericLiteralValueIncomplete(
    NumericLiteralValueIncomplete<TokenOwnedString>,
);

impl JsNumericLiteralValueIncomplete {
    const JS_NAME: &'static str = "NumericLiteralValueIncomplete";
}

#[wasm_bindgen(js_class = NumericLiteralValueIncomplete)]
impl JsNumericLiteralValueIncomplete {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsNumericLiteralValueIncomplete, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: JsNumericLiteralValueKind,
        position: JsSubstringPosition,
        string: String,
    ) -> Self {
        Self(NumericLiteralValueIncomplete {
            kind: kind.into(),
            position: position.into(),
            string: string.as_str().into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JsNumericLiteralValueKind {
        (&self.0.kind).into()
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

impl From<JsNumericLiteralValueIncomplete>
    for NumericLiteralValueIncomplete<TokenOwnedString>
{
    fn from(value: JsNumericLiteralValueIncomplete) -> Self {
        value.0
    }
}

impl From<NumericLiteralValueIncomplete<TokenOwnedString>>
    for JsNumericLiteralValueIncomplete
{
    fn from(value: NumericLiteralValueIncomplete<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsNumericLiteralValueIncomplete {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteralValueIncomplete {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for NumericLiteralValueIncomplete<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {})",
            JsNumericLiteralValueIncomplete::JS_NAME,
            self.kind.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
