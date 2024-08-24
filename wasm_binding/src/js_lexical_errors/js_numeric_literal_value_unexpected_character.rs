use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralValueUnexpectedCharacter;

use crate::js_numeric_literal_value_kind::JsNumericLiteralValueKind;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteralValueUnexpectedCharacter")]
pub struct JsNumericLiteralValueUnexpectedCharacter(
    NumericLiteralValueUnexpectedCharacter<TokenOwnedString>,
);

impl JsNumericLiteralValueUnexpectedCharacter {
    const JS_NAME: &'static str = "NumericLiteralValueUnexpectedCharacter";
}

#[wasm_bindgen(js_class = NumericLiteralValueUnexpectedCharacter)]
impl JsNumericLiteralValueUnexpectedCharacter {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsNumericLiteralValueUnexpectedCharacter, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        character: char,
        expected: String,
        kind: JsNumericLiteralValueKind,
        position: &JsSubstringPosition,
        string: String,
    ) -> Self {
        Self(NumericLiteralValueUnexpectedCharacter {
            character,
            expected: expected.as_str().into(),
            kind: kind.into(),
            position: position.ref_to(),
            string: string.as_str().into(),
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

impl From<JsNumericLiteralValueUnexpectedCharacter>
    for NumericLiteralValueUnexpectedCharacter<TokenOwnedString>
{
    fn from(value: JsNumericLiteralValueUnexpectedCharacter) -> Self {
        value.0
    }
}

impl From<NumericLiteralValueUnexpectedCharacter<TokenOwnedString>>
    for JsNumericLiteralValueUnexpectedCharacter
{
    fn from(
        value: NumericLiteralValueUnexpectedCharacter<TokenOwnedString>,
    ) -> Self {
        Self(value)
    }
}

impl GetJsName for JsNumericLiteralValueUnexpectedCharacter {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteralValueUnexpectedCharacter {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString
    for NumericLiteralValueUnexpectedCharacter<TokenOwnedString>
{
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {})",
            JsNumericLiteralValueUnexpectedCharacter::JS_NAME,
            self.character.try_to_js_string()?,
            self.expected.try_to_js_string()?,
            self.kind.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
