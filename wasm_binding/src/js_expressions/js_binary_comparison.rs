use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::BinaryComparison;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "BinaryComparison")]
pub struct JsBinaryComparison(BinaryComparison<OwnedString>);

impl JsBinaryComparison {
    const JS_NAME: &'static str = "BinaryComparison";
}

#[wasm_bindgen(js_class = BinaryComparison)]
impl JsBinaryComparison {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsBinaryComparison, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        left: &JsValue,
        right: &JsValue,
        operator: &JsValue,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsBinaryComparison, JsValue> {
        Ok(Self(BinaryComparison {
            left: Box::new(left.try_ref_to()?),
            right: Box::new(right.try_ref_to()?),
            operator: operator.try_ref_to()?,
            operator_position: operatorPosition.ref_to(),
            operator_fillers: operatorFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn left(&self) -> JsValue {
        self.0.left.as_ref().ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn right(&self) -> JsValue {
        self.0.right.as_ref().ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> JsValue {
        self.0.operator.ref_to()
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
    pub fn equal_to(&self, other: &JsBinaryComparison) -> bool {
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

impl From<JsBinaryComparison> for BinaryComparison<OwnedString> {
    fn from(value: JsBinaryComparison) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, BinaryComparison<OwnedString>> for JsValue {
    fn from_ref(value: &'rust BinaryComparison<OwnedString>) -> Self {
        JsBinaryComparison(value.clone()).into()
    }
}

impl GetJsName for JsBinaryComparison {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsBinaryComparison {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for BinaryComparison<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {})",
            JsBinaryComparison::JS_NAME,
            self.left.try_to_js_string()?,
            self.right.try_to_js_string()?,
            self.operator.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?,
        ))
    }
}
