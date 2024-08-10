use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::BinaryArithmeticOperation;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "BinaryArithmeticOperation")]
pub struct JsBinaryArithmeticOperation(BinaryArithmeticOperation<OwnedString>);

impl JsBinaryArithmeticOperation {
    const JS_NAME: &'static str = "BinaryArithmeticOperation";
}

#[wasm_bindgen(js_class = BinaryArithmeticOperation)]
impl JsBinaryArithmeticOperation {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsBinaryArithmeticOperation, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        left: &JsValue,
        right: &JsValue,
        operator: &JsValue,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsBinaryArithmeticOperation, JsValue> {
        Ok(Self(BinaryArithmeticOperation {
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
    pub fn equal_to(&self, other: &JsBinaryArithmeticOperation) -> bool {
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

impl<'rust> FromRef<'rust, BinaryArithmeticOperation<OwnedString>>
    for JsValue
{
    fn from_ref(value: &'rust BinaryArithmeticOperation<OwnedString>) -> Self {
        JsBinaryArithmeticOperation(value.clone()).into()
    }
}

impl From<JsBinaryArithmeticOperation>
    for BinaryArithmeticOperation<OwnedString>
{
    fn from(value: JsBinaryArithmeticOperation) -> Self {
        value.0
    }
}

impl GetJsName for JsBinaryArithmeticOperation {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsBinaryArithmeticOperation {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for BinaryArithmeticOperation<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {})",
            JsBinaryArithmeticOperation::JS_NAME,
            self.left.try_to_js_string()?,
            self.right.try_to_js_string()?,
            self.operator.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?,
        ))
    }
}
