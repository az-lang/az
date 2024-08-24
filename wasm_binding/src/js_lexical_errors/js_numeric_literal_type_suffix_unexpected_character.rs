use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralTypeSuffixUnexpectedCharacter;

use crate::js_numeric_literal_value_kind::JsNumericLiteralValueKind;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteralTypeSuffixUnexpectedCharacter")]
pub struct JsNumericLiteralTypeSuffixUnexpectedCharacter(
    NumericLiteralTypeSuffixUnexpectedCharacter<TokenOwnedString>,
);

impl JsNumericLiteralTypeSuffixUnexpectedCharacter {
    const JS_NAME: &'static str =
        "NumericLiteralTypeSuffixUnexpectedCharacter";
}

#[wasm_bindgen(js_class = NumericLiteralTypeSuffixUnexpectedCharacter)]
impl JsNumericLiteralTypeSuffixUnexpectedCharacter {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsNumericLiteralTypeSuffixUnexpectedCharacter, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        character: char,
        expected: String,
        position: JsSubstringPosition,
        string: String,
        value: String,
        value_kind: JsNumericLiteralValueKind,
    ) -> Self {
        Self(NumericLiteralTypeSuffixUnexpectedCharacter {
            character,
            expected: expected.as_str().into(),
            position: position.into(),
            string: string.as_str().into(),
            value: value.as_str().into(),
            value_kind: value_kind.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn character(&self) -> char {
        self.0.character
    }

    #[wasm_bindgen(getter)]
    pub fn expected(&self) -> String {
        self.0.expected.as_ref().into()
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

impl From<JsNumericLiteralTypeSuffixUnexpectedCharacter>
    for NumericLiteralTypeSuffixUnexpectedCharacter<TokenOwnedString>
{
    fn from(value: JsNumericLiteralTypeSuffixUnexpectedCharacter) -> Self {
        value.0
    }
}

impl From<NumericLiteralTypeSuffixUnexpectedCharacter<TokenOwnedString>>
    for JsNumericLiteralTypeSuffixUnexpectedCharacter
{
    fn from(
        value: NumericLiteralTypeSuffixUnexpectedCharacter<TokenOwnedString>,
    ) -> Self {
        Self(value)
    }
}

impl GetJsName for JsNumericLiteralTypeSuffixUnexpectedCharacter {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteralTypeSuffixUnexpectedCharacter {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString
    for NumericLiteralTypeSuffixUnexpectedCharacter<TokenOwnedString>
{
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {})",
            JsNumericLiteralTypeSuffixUnexpectedCharacter::JS_NAME,
            self.character.try_to_js_string()?,
            self.expected.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
            self.value.try_to_js_string()?,
            self.value_kind.try_to_js_string()?,
        ))
    }
}
