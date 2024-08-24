use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::UnexpectedCharacter;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "UnexpectedCharacter")]
pub struct JsUnexpectedCharacter(UnexpectedCharacter<TokenOwnedString>);

impl JsUnexpectedCharacter {
    const JS_NAME: &'static str = "UnexpectedCharacter";
}

#[wasm_bindgen(js_class = UnexpectedCharacter)]
impl JsUnexpectedCharacter {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsUnexpectedCharacter, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        character: char,
        position: &JsSubstringPosition,
        string: String,
    ) -> Self {
        Self(UnexpectedCharacter {
            character,
            position: position.ref_to(),
            string: string.as_str().into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn character(&self) -> char {
        self.0.character
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

impl From<JsUnexpectedCharacter> for UnexpectedCharacter<TokenOwnedString> {
    fn from(value: JsUnexpectedCharacter) -> Self {
        value.0
    }
}

impl From<UnexpectedCharacter<TokenOwnedString>> for JsUnexpectedCharacter {
    fn from(value: UnexpectedCharacter<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsUnexpectedCharacter {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsUnexpectedCharacter {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for UnexpectedCharacter<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {})",
            JsUnexpectedCharacter::JS_NAME,
            self.character.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
