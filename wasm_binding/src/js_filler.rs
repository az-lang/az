use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use crate::validation::validate_contents;
use az::parsing::Filler;

use super::js_filler_content::JsFillerContent;
use super::js_substring_position::JsSubstringPosition;
use super::traits::{
    FromRef, GetJsName, RefTo, TryFromRef, TryRefTo, TryToJsString, TryToJson,
};
use super::types::OwnedString;
use super::utils::to_invalid_js_value_message;

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Filler")]
pub struct JsFiller(Filler<OwnedString>);

impl JsFiller {
    const JS_NAME: &'static str = "Filler";
}

#[wasm_bindgen(js_class = Filler)]
impl JsFiller {
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> JsFillerContent {
        self.0.content.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsFiller, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        content: &JsFillerContent,
        position: &JsSubstringPosition,
    ) -> Result<JsFiller, JsValue> {
        Ok(Self(Filler {
            content: content.ref_to(),
            position: position.ref_to(),
        }))
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsFiller) -> bool {
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
}

impl From<Filler<OwnedString>> for JsFiller {
    fn from(value: Filler<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<JsFiller> for Filler<OwnedString> {
    fn from(value: JsFiller) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, Filler<OwnedString>> for JsFiller {
    fn from_ref(value: &'rust Filler<OwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, Filler<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Filler<OwnedString>) -> Self {
        JsFiller::from_ref(value).into()
    }
}

impl<'rust> FromRef<'rust, Vec<Filler<OwnedString>>> for Vec<JsFiller> {
    fn from_ref(value: &'rust Vec<Filler<OwnedString>>) -> Self {
        value.iter().map(RefTo::ref_to).collect()
    }
}

impl<'rust> FromRef<'rust, Vec<Filler<OwnedString>>> for js_sys::Array {
    fn from_ref(value: &'rust Vec<Filler<OwnedString>>) -> Self {
        value.iter().map(JsValue::from_ref).collect()
    }
}

impl<'rust> FromRef<'rust, Vec<Vec<Filler<OwnedString>>>>
    for Vec<js_sys::Array>
{
    fn from_ref(value: &'rust Vec<Vec<Filler<OwnedString>>>) -> Self {
        value.iter().map(RefTo::ref_to).collect()
    }
}

impl GetJsName for JsFiller {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<Filler<OwnedString>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| JsFiller::try_from_ref(&value).map(Into::into))
            .collect()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<Vec<Filler<OwnedString>>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| Vec::<Filler<OwnedString>>::try_from_ref(&value))
            .collect()
    }
}

impl<'js> TryFromRef<'js, JsValue> for Vec<Filler<OwnedString>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        value
            .dyn_ref::<js_sys::Array>()
            .ok_or_else(|| {
                JsValue::from(&to_invalid_js_value_message::<js_sys::Array>(
                    value,
                ))
            })?
            .try_ref_to()
    }
}

impl TryToJsString for JsFiller {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Filler<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsFiller::JS_NAME,
            self.content.try_to_js_string()?,
            self.position.try_to_js_string()?,
        ))
    }
}
