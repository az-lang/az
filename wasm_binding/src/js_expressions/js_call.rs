use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Call;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Call")]
pub struct JsCall(Call<OwnedString>);

impl JsCall {
    const JS_NAME: &'static str = "Call";
}

#[wasm_bindgen(js_class = Call)]
impl JsCall {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsCall, JsValue> {
        value.try_ref_to()
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        callable: &JsValue,
        arguments_: &js_sys::Array,
        #[allow(non_snake_case)] openParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] commaPositions: &js_sys::Array,
        #[allow(non_snake_case)]
        closeParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] commaFillers: &js_sys::Array,
        #[allow(non_snake_case)] closeParenthesisFillers: &js_sys::Array,
    ) -> Result<JsCall, JsValue> {
        Ok(Self(Call {
            callable: Box::new(callable.try_ref_to()?),
            arguments: arguments_.try_ref_to()?,
            open_parenthesis_position: openParenthesisPosition.ref_to(),
            comma_positions: commaPositions.try_ref_to()?,
            close_parenthesis_position: closeParenthesisPosition.ref_to(),
            open_parenthesis_fillers: openParenthesisFillers.try_ref_to()?,
            comma_fillers: commaFillers.try_ref_to()?,
            close_parenthesis_fillers: closeParenthesisFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter = arguments_)]
    pub fn arguments(&self) -> Vec<JsValue> {
        self.0.arguments.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn callable(&self) -> JsValue {
        self.0.callable.as_ref().ref_to()
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

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsCall) -> bool {
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

impl From<JsCall> for Call<OwnedString> {
    fn from(value: JsCall) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, Call<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Call<OwnedString>) -> Self {
        JsCall(value.clone()).into()
    }
}

impl GetJsName for JsCall {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsCall {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Call<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {}, {}, {})",
            JsCall::JS_NAME,
            self.callable.try_to_js_string()?,
            self.arguments.try_to_js_string()?,
            self.open_parenthesis_position.try_to_js_string()?,
            self.comma_positions.try_to_js_string()?,
            self.close_parenthesis_position.try_to_js_string()?,
            self.open_parenthesis_fillers.try_to_js_string()?,
            self.comma_fillers.try_to_js_string()?,
            self.close_parenthesis_fillers.try_to_js_string()?
        ))
    }
}
