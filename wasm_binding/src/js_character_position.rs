use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::CharacterPosition;

use super::js_byte_count::JsByteCount;
use super::js_utf_8_count::JsUtf8Count;
use super::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "CharacterPosition")]
pub struct JsCharacterPosition(CharacterPosition);

impl JsCharacterPosition {
    const JS_NAME: &'static str = "CharacterPosition";
}

#[wasm_bindgen(js_class = CharacterPosition)]
impl JsCharacterPosition {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsCharacterPosition, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        byte: &JsByteCount,
        #[allow(non_snake_case)] utf8: &JsUtf8Count,
    ) -> Self {
        Self(CharacterPosition {
            byte: byte.ref_to(),
            utf_8: utf8.ref_to(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn byte(&self) -> JsByteCount {
        self.0.byte.ref_to()
    }

    #[wasm_bindgen(getter = utf8)]
    pub fn utf_8(&self) -> JsUtf8Count {
        self.0.utf_8.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsCharacterPosition) -> bool {
        self == other
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

impl<'rust> FromRef<'rust, CharacterPosition> for JsCharacterPosition {
    fn from_ref(value: &'rust CharacterPosition) -> Self {
        Self(*value)
    }
}

impl<'js> FromRef<'js, JsCharacterPosition> for CharacterPosition {
    fn from_ref(value: &'js JsCharacterPosition) -> Self {
        value.0
    }
}

impl GetJsName for JsCharacterPosition {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for CharacterPosition {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsCharacterPosition::JS_NAME,
            self.byte.try_to_js_string()?,
            self.utf_8.try_to_js_string()?
        ))
    }
}

impl TryToJsString for JsCharacterPosition {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}
