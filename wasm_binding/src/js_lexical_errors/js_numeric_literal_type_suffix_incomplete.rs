use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralTypeSuffixIncomplete;

use crate::js_numeric_literal_value_kind::JsNumericLiteralValueKind;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteralTypeSuffixIncomplete")]
pub struct JsNumericLiteralTypeSuffixIncomplete(
    NumericLiteralTypeSuffixIncomplete<TokenOwnedString>,
);

impl JsNumericLiteralTypeSuffixIncomplete {
    const JS_NAME: &'static str = "NumericLiteralTypeSuffixIncomplete";
}

#[wasm_bindgen(js_class = NumericLiteralTypeSuffixIncomplete)]
impl JsNumericLiteralTypeSuffixIncomplete {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsNumericLiteralTypeSuffixIncomplete, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        position: JsSubstringPosition,
        string: String,
        value: String,
        value_kind: JsNumericLiteralValueKind,
    ) -> Self {
        Self(NumericLiteralTypeSuffixIncomplete {
            position: position.into(),
            string: string.as_str().into(),
            value: value.as_str().into(),
            value_kind: value_kind.into(),
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

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.0.value.as_ref().into()
    }

    #[wasm_bindgen(getter)]
    pub fn value_kind(&self) -> JsNumericLiteralValueKind {
        (&self.0.value_kind).into()
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

impl From<JsNumericLiteralTypeSuffixIncomplete>
    for NumericLiteralTypeSuffixIncomplete<TokenOwnedString>
{
    fn from(value: JsNumericLiteralTypeSuffixIncomplete) -> Self {
        value.0
    }
}

impl From<NumericLiteralTypeSuffixIncomplete<TokenOwnedString>>
    for JsNumericLiteralTypeSuffixIncomplete
{
    fn from(
        value: NumericLiteralTypeSuffixIncomplete<TokenOwnedString>,
    ) -> Self {
        Self(value)
    }
}

impl GetJsName for JsNumericLiteralTypeSuffixIncomplete {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteralTypeSuffixIncomplete {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for NumericLiteralTypeSuffixIncomplete<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsNumericLiteralTypeSuffixIncomplete::JS_NAME,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
            self.value.try_to_js_string()?,
            self.value_kind.try_to_js_string()?,
        ))
    }
}
