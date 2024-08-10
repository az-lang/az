use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::FunctionType;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "FunctionType")]
pub struct JsFunctionType(FunctionType<OwnedString>);

impl JsFunctionType {
    const JS_NAME: &'static str = "FunctionType";
}

#[wasm_bindgen(js_class = FunctionType)]
impl JsFunctionType {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsFunctionType, JsValue> {
        value.try_ref_to()
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        parameters: &js_sys::Array,
        #[allow(non_snake_case)] returnType: &JsValue,
        #[allow(non_snake_case)] openParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] commaPositions: &js_sys::Array,
        #[allow(non_snake_case)]
        closeParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] commaFillers: &js_sys::Array,
        #[allow(non_snake_case)] closeParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsFunctionType, JsValue> {
        Ok(Self(FunctionType {
            parameters: parameters.try_ref_to()?,
            return_type: Box::new(returnType.try_ref_to()?),
            open_parenthesis_position: openParenthesisPosition.ref_to(),
            comma_positions: commaPositions.try_ref_to()?,
            close_parenthesis_position: closeParenthesisPosition.ref_to(),
            operator_position: operatorPosition.ref_to(),
            open_parenthesis_fillers: openParenthesisFillers.try_ref_to()?,
            comma_fillers: commaFillers.try_ref_to()?,
            close_parenthesis_fillers: closeParenthesisFillers.try_ref_to()?,
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

    #[wasm_bindgen(getter = closeParenthesisFillers)]
    pub fn close_parenthesis_fillers(&self) -> Vec<JsFiller> {
        self.0.close_parenthesis_fillers.ref_to()
    }

    #[wasm_bindgen(getter = closeParenthesisPosition)]
    pub fn close_parenthesis_position(&self) -> JsSubstringPosition {
        self.0.close_parenthesis_position.ref_to()
    }

    #[wasm_bindgen(getter = commaFillers)]
    pub fn comma_fillers(&self) -> Vec<js_sys::Array> {
        self.0.comma_fillers.ref_to()
    }

    #[wasm_bindgen(getter = commaPositions)]
    pub fn comma_positions(&self) -> Vec<JsSubstringPosition> {
        self.0.comma_positions.ref_to()
    }

    #[wasm_bindgen(getter = openParenthesisFillers)]
    pub fn open_parenthesis_fillers(&self) -> Vec<JsFiller> {
        self.0.open_parenthesis_fillers.ref_to()
    }

    #[wasm_bindgen(getter = openParenthesisPosition)]
    pub fn open_parenthesis_position(&self) -> JsSubstringPosition {
        self.0.open_parenthesis_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn parameters(&self) -> Vec<JsValue> {
        self.0.parameters.ref_to()
    }

    #[wasm_bindgen(getter = returnType)]
    pub fn return_type(&self) -> JsValue {
        self.0.return_type.as_ref().ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsFunctionType) -> bool {
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

impl From<JsFunctionType> for FunctionType<OwnedString> {
    fn from(value: JsFunctionType) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, FunctionType<OwnedString>> for JsValue {
    fn from_ref(value: &'rust FunctionType<OwnedString>) -> Self {
        JsFunctionType(value.clone()).into()
    }
}

impl GetJsName for JsFunctionType {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsFunctionType {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for FunctionType<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            JsFunctionType::JS_NAME,
            self.parameters.try_to_js_string()?,
            self.return_type.try_to_js_string()?,
            self.open_parenthesis_position.try_to_js_string()?,
            self.comma_positions.try_to_js_string()?,
            self.close_parenthesis_position.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.open_parenthesis_fillers.try_to_js_string()?,
            self.comma_fillers.try_to_js_string()?,
            self.close_parenthesis_fillers.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?,
        ))
    }
}
