use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Block;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Block")]
pub struct JsBlock(Block<OwnedString>);

impl JsBlock {
    const JS_NAME: &'static str = "Block";
}

#[wasm_bindgen(js_class = Block)]
impl JsBlock {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsBlock, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        statements: &js_sys::Array,
        expression: &JsValue,
        #[allow(non_snake_case)] openBracePosition: &JsSubstringPosition,
        #[allow(non_snake_case)] closeBracePosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openBraceFillers: &js_sys::Array,
        #[allow(non_snake_case)] closeBraceFillers: &js_sys::Array,
    ) -> Result<JsBlock, JsValue> {
        Ok(Self(Block {
            statements: statements.try_ref_to()?,
            expression: if expression.is_null() {
                None
            } else {
                Some(Box::new(expression.try_ref_to()?))
            },
            open_brace_position: openBracePosition.ref_to(),
            close_brace_position: closeBracePosition.ref_to(),
            open_brace_fillers: openBraceFillers.try_ref_to()?,
            close_brace_fillers: closeBraceFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter = closeBraceFillers)]
    pub fn close_brace_fillers(&self) -> Vec<JsFiller> {
        self.0.close_brace_fillers.ref_to()
    }

    #[wasm_bindgen(getter = closeBracePosition)]
    pub fn close_brace_position(&self) -> JsSubstringPosition {
        self.0.close_brace_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> JsValue {
        self.0
            .expression
            .as_ref()
            .map(AsRef::as_ref)
            .map(RefTo::ref_to)
            .unwrap_or_else(|| JsValue::NULL)
    }

    #[wasm_bindgen(getter = openBraceFillers)]
    pub fn open_brace_fillers(&self) -> Vec<JsFiller> {
        self.0.open_brace_fillers.ref_to()
    }

    #[wasm_bindgen(getter = openBracePosition)]
    pub fn open_brace_position(&self) -> JsSubstringPosition {
        self.0.open_brace_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn statements(&self) -> Vec<JsValue> {
        self.0.statements.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsBlock) -> bool {
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

impl From<JsBlock> for Block<OwnedString> {
    fn from(value: JsBlock) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, Block<OwnedString>> for JsBlock {
    fn from_ref(value: &'rust Block<OwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, Block<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Block<OwnedString>) -> Self {
        JsBlock::from_ref(value).into()
    }
}

impl<'js> FromRef<'js, JsBlock> for Block<OwnedString> {
    fn from_ref(value: &'js JsBlock) -> Self {
        value.0.clone()
    }
}

impl GetJsName for JsBlock {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsBlock {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Block<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {})",
            JsBlock::JS_NAME,
            self.statements.try_to_js_string()?,
            self.expression.try_to_js_string()?,
            self.open_brace_position.try_to_js_string()?,
            self.close_brace_position.try_to_js_string()?,
            self.open_brace_fillers.try_to_js_string()?,
            self.close_brace_fillers.try_to_js_string()?
        ))
    }
}
