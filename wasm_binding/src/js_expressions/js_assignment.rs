use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Assignment;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Assignment")]
pub struct JsAssignment(Assignment<OwnedString>);

impl JsAssignment {
    const JS_NAME: &'static str = "Assignment";
}

#[wasm_bindgen(js_class = Assignment)]
impl JsAssignment {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsAssignment, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        target: &JsValue,
        value: &JsValue,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsAssignment, JsValue> {
        Ok(JsAssignment(Assignment {
            target: Box::new(target.try_ref_to()?),
            value: Box::new(value.try_ref_to()?),
            operator_position: operatorPosition.ref_to(),
            operator_fillers: operatorFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter = operatorFillers)]
    pub fn operator_fillers(&self) -> Vec<JsFiller> {
        self.0.operator_fillers.ref_to()
    }

    #[wasm_bindgen(getter = operatorPosition)]
    pub fn operator_position(&self) -> JsSubstringPosition {
        self.0.operator_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn target(&self) -> JsValue {
        self.0.target.as_ref().ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.0.value.as_ref().ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsAssignment) -> bool {
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

impl From<JsAssignment> for Assignment<OwnedString> {
    fn from(value: JsAssignment) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, Assignment<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Assignment<OwnedString>) -> Self {
        JsAssignment(value.clone()).into()
    }
}

impl GetJsName for JsAssignment {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsAssignment {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Assignment<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsAssignment::JS_NAME,
            self.target.try_to_js_string()?,
            self.value.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?
        ))
    }
}
