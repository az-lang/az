use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::FunctionDefinition;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::js_annotated_identifier::JsAnnotatedIdentifier;
use super::js_block::JsBlock;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "FunctionDefinition")]
pub struct JsFunctionDefinition(FunctionDefinition<OwnedString>);

impl JsFunctionDefinition {
    const JS_NAME: &'static str = "FunctionDefinition";
}

#[wasm_bindgen(js_class = FunctionDefinition)]
impl JsFunctionDefinition {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsFunctionDefinition, JsValue> {
        value.try_ref_to()
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        parameters: &js_sys::Array,
        #[allow(non_snake_case)] returnType: &JsValue,
        body: &JsBlock,
        #[allow(non_snake_case)] openerPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] commaPositions: &js_sys::Array,
        #[allow(non_snake_case)]
        closeParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] arrowPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openerFillers: &js_sys::Array,
        #[allow(non_snake_case)] openParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] commaFillers: &js_sys::Array,
        #[allow(non_snake_case)] closeParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] arrowFillers: &js_sys::Array,
    ) -> Result<JsFunctionDefinition, JsValue> {
        Ok(Self(FunctionDefinition {
            parameters: parameters.try_ref_to()?,
            return_type: Box::new(returnType.try_ref_to()?),
            body: body.ref_to(),
            opener_position: openerPosition.ref_to(),
            open_parenthesis_position: openParenthesisPosition.ref_to(),
            comma_positions: commaPositions.try_ref_to()?,
            close_parenthesis_position: closeParenthesisPosition.ref_to(),
            arrow_position: arrowPosition.ref_to(),
            opener_fillers: openerFillers.try_ref_to()?,
            open_parenthesis_fillers: openParenthesisFillers.try_ref_to()?,
            comma_fillers: commaFillers.try_ref_to()?,
            close_parenthesis_fillers: closeParenthesisFillers.try_ref_to()?,
            arrow_fillers: arrowFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter = arrowFillers)]
    pub fn arrow_fillers(&self) -> Vec<JsFiller> {
        self.0.arrow_fillers.ref_to()
    }

    #[wasm_bindgen(getter = arrowPosition)]
    pub fn arrow_position(&self) -> JsSubstringPosition {
        self.0.arrow_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> JsBlock {
        self.0.body.ref_to()
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

    #[wasm_bindgen(getter = openerFillers)]
    pub fn opener_fillers(&self) -> Vec<JsFiller> {
        self.0.opener_fillers.ref_to()
    }

    #[wasm_bindgen(getter = openerPosition)]
    pub fn opener_position(&self) -> JsSubstringPosition {
        self.0.opener_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn parameters(&self) -> Vec<JsAnnotatedIdentifier> {
        self.0.parameters.ref_to()
    }

    #[wasm_bindgen(getter = returnType)]
    pub fn return_type(&self) -> JsValue {
        self.0.return_type.as_ref().ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsFunctionDefinition) -> bool {
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

impl From<JsFunctionDefinition> for FunctionDefinition<OwnedString> {
    fn from(value: JsFunctionDefinition) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, FunctionDefinition<OwnedString>> for JsValue {
    fn from_ref(value: &'rust FunctionDefinition<OwnedString>) -> Self {
        JsFunctionDefinition(value.clone()).into()
    }
}

impl GetJsName for JsFunctionDefinition {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsFunctionDefinition {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for FunctionDefinition<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            JsFunctionDefinition::JS_NAME,
            self.parameters.try_to_js_string()?,
            self.return_type.try_to_js_string()?,
            self.body.try_to_js_string()?,
            self.opener_position.try_to_js_string()?,
            self.open_parenthesis_position.try_to_js_string()?,
            self.comma_positions.try_to_js_string()?,
            self.close_parenthesis_position.try_to_js_string()?,
            self.arrow_position.try_to_js_string()?,
            self.opener_fillers.try_to_js_string()?,
            self.open_parenthesis_fillers.try_to_js_string()?,
            self.comma_fillers.try_to_js_string()?,
            self.close_parenthesis_fillers.try_to_js_string()?,
            self.arrow_fillers.try_to_js_string()?,
        ))
    }
}
