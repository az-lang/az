use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Identifier;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Identifier")]
pub struct JsIdentifier(Identifier<OwnedString>);

impl JsIdentifier {
    const JS_NAME: &'static str = "Identifier";
}

#[wasm_bindgen(js_class = Identifier)]
impl JsIdentifier {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsIdentifier, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        string: String,
        position: &JsSubstringPosition,
        fillers: &js_sys::Array,
    ) -> Result<JsIdentifier, JsValue> {
        Ok(Self(Identifier {
            string: string.into(),
            position: position.ref_to(),
            fillers: fillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn fillers(&self) -> Vec<JsFiller> {
        self.0.fillers.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsIdentifier) -> bool {
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

    #[wasm_bindgen(js_name = "validateContents")]
    pub fn validate_contents(&self) -> Result<(), JsValue> {
        validate_contents(&self.0)
    }

    #[wasm_bindgen(js_name = "validatePositions")]
    pub fn validate_positions(&self) -> Result<(), JsValue> {
        validate_positions(&self.0)
    }
}

impl From<JsIdentifier> for Identifier<OwnedString> {
    fn from(value: JsIdentifier) -> Self {
        value.0
    }
}

impl<'js> FromRef<'js, JsIdentifier> for Identifier<OwnedString> {
    fn from_ref(value: &'js JsIdentifier) -> Self {
        value.0.clone()
    }
}

impl<'rust> FromRef<'rust, Identifier<OwnedString>> for JsIdentifier {
    fn from_ref(value: &'rust Identifier<OwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, Identifier<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Identifier<OwnedString>) -> Self {
        JsIdentifier::from_ref(value).into()
    }
}

impl GetJsName for JsIdentifier {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsIdentifier {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Identifier<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {})",
            JsIdentifier::JS_NAME,
            self.string.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.fillers.try_to_js_string()?
        ))
    }
}
