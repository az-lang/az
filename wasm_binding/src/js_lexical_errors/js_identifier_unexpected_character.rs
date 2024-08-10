use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::IdentifierUnexpectedCharacter;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "IdentifierUnexpectedCharacter")]
pub struct JsIdentifierUnexpectedCharacter(
    IdentifierUnexpectedCharacter<TokenOwnedString>,
);

impl JsIdentifierUnexpectedCharacter {
    const JS_NAME: &'static str = "IdentifierUnexpectedCharacter";
}

#[wasm_bindgen(js_class = IdentifierUnexpectedCharacter)]
impl JsIdentifierUnexpectedCharacter {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsIdentifierUnexpectedCharacter, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        character: char,
        expected: String,
        position: JsSubstringPosition,
        string: String,
    ) -> Self {
        Self(IdentifierUnexpectedCharacter {
            character,
            expected: expected.as_str().into(),
            position: position.into(),
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

impl From<JsIdentifierUnexpectedCharacter>
    for IdentifierUnexpectedCharacter<TokenOwnedString>
{
    fn from(value: JsIdentifierUnexpectedCharacter) -> Self {
        value.0
    }
}

impl From<IdentifierUnexpectedCharacter<TokenOwnedString>>
    for JsIdentifierUnexpectedCharacter
{
    fn from(value: IdentifierUnexpectedCharacter<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsIdentifierUnexpectedCharacter {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsIdentifierUnexpectedCharacter {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for IdentifierUnexpectedCharacter<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsIdentifierUnexpectedCharacter::JS_NAME,
            self.character.try_to_js_string()?,
            self.expected.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
