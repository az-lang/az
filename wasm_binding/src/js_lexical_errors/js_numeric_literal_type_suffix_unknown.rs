use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralTypeSuffixUnknown;

use crate::js_numeric_literal_value_kind::JsNumericLiteralValueKind;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteralTypeSuffixUnknown")]
pub struct JsNumericLiteralTypeSuffixUnknown(
    NumericLiteralTypeSuffixUnknown<TokenOwnedString>,
);

impl JsNumericLiteralTypeSuffixUnknown {
    const JS_NAME: &'static str = "NumericLiteralTypeSuffixUnknown";
}

#[wasm_bindgen(js_class = NumericLiteralTypeSuffixUnknown)]
impl JsNumericLiteralTypeSuffixUnknown {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsNumericLiteralTypeSuffixUnknown, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        position: JsSubstringPosition,
        string: String,
        type_suffix: String,
        value: String,
        value_kind: JsNumericLiteralValueKind,
    ) -> Self {
        Self(NumericLiteralTypeSuffixUnknown {
            position: position.into(),
            string: string.as_str().into(),
            type_suffix: type_suffix.as_str().into(),
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
    pub fn type_suffix(&self) -> String {
        self.0.type_suffix.as_ref().into()
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

impl From<JsNumericLiteralTypeSuffixUnknown>
    for NumericLiteralTypeSuffixUnknown<TokenOwnedString>
{
    fn from(value: JsNumericLiteralTypeSuffixUnknown) -> Self {
        value.0
    }
}

impl From<NumericLiteralTypeSuffixUnknown<TokenOwnedString>>
    for JsNumericLiteralTypeSuffixUnknown
{
    fn from(value: NumericLiteralTypeSuffixUnknown<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsNumericLiteralTypeSuffixUnknown {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteralTypeSuffixUnknown {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for NumericLiteralTypeSuffixUnknown<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {})",
            JsNumericLiteralTypeSuffixUnknown::JS_NAME,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
            self.type_suffix.try_to_js_string()?,
            self.value.try_to_js_string()?,
            self.value_kind.try_to_js_string()?,
        ))
    }
}
