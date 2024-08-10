use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::AnnotatedIdentifier;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryFromRef, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::js_identifier::JsIdentifier;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "AnnotatedIdentifier")]
pub struct JsAnnotatedIdentifier(AnnotatedIdentifier<OwnedString>);

impl JsAnnotatedIdentifier {
    const JS_NAME: &'static str = "AnnotatedIdentifier";
}

#[wasm_bindgen(js_class = AnnotatedIdentifier)]
impl JsAnnotatedIdentifier {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsAnnotatedIdentifier, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        identifier: &JsIdentifier,
        annotation: &JsValue,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsAnnotatedIdentifier, JsValue> {
        Ok(JsAnnotatedIdentifier(AnnotatedIdentifier {
            identifier: identifier.ref_to(),
            annotation: Box::new(annotation.try_ref_to()?),
            operator_position: operatorPosition.ref_to(),
            operator_fillers: operatorFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn annotation(&self) -> JsValue {
        self.0.annotation.as_ref().ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn identifier(&self) -> JsIdentifier {
        self.0.identifier.ref_to()
    }

    #[wasm_bindgen(getter = operatorFillers)]
    pub fn operator_fillers(&self) -> Vec<JsFiller> {
        self.0.operator_fillers.ref_to()
    }

    #[wasm_bindgen(getter = operatorPosition)]
    pub fn operator_position(&self) -> JsSubstringPosition {
        self.0.operator_position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsAnnotatedIdentifier) -> bool {
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

impl From<JsAnnotatedIdentifier> for AnnotatedIdentifier<OwnedString> {
    fn from(value: JsAnnotatedIdentifier) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, AnnotatedIdentifier<OwnedString>>
    for JsAnnotatedIdentifier
{
    fn from_ref(value: &'rust AnnotatedIdentifier<OwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, AnnotatedIdentifier<OwnedString>> for JsValue {
    fn from_ref(value: &'rust AnnotatedIdentifier<OwnedString>) -> Self {
        JsAnnotatedIdentifier::from_ref(value).into()
    }
}

impl<'rust> FromRef<'rust, Vec<AnnotatedIdentifier<OwnedString>>>
    for Vec<JsAnnotatedIdentifier>
{
    fn from_ref(value: &'rust Vec<AnnotatedIdentifier<OwnedString>>) -> Self {
        value.iter().map(RefTo::ref_to).collect()
    }
}

impl GetJsName for JsAnnotatedIdentifier {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array>
    for Vec<AnnotatedIdentifier<OwnedString>>
{
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| {
                JsAnnotatedIdentifier::try_from_ref(&value).map(Into::into)
            })
            .collect()
    }
}

impl TryToJsString for JsAnnotatedIdentifier {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for AnnotatedIdentifier<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsAnnotatedIdentifier::JS_NAME,
            self.identifier.try_to_js_string()?,
            self.annotation.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?
        ))
    }
}
