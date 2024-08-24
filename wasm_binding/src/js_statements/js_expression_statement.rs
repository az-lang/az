use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::ExpressionStatement;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;

use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "ExpressionStatement")]
pub struct JsExpressionStatement(ExpressionStatement<OwnedString>);

impl JsExpressionStatement {
    const JS_NAME: &'static str = "ExpressionStatement";
}

#[wasm_bindgen(js_class = ExpressionStatement)]
impl JsExpressionStatement {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: JsValue,
    ) -> Result<JsExpressionStatement, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        expression: &JsValue,
        #[allow(non_snake_case)] semicolonPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] semicolonFillers: &js_sys::Array,
    ) -> Result<JsExpressionStatement, JsValue> {
        Ok(JsExpressionStatement(ExpressionStatement {
            expression: expression.try_ref_to()?,
            semicolon_position: semicolonPosition.ref_to(),
            semicolon_fillers: semicolonFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> JsValue {
        self.0.expression.ref_to()
    }

    #[wasm_bindgen(getter = semicolonFillers)]
    pub fn semicolon_fillers(&self) -> Vec<JsFiller> {
        self.0.semicolon_fillers.ref_to()
    }

    #[wasm_bindgen(getter = semicolonPosition)]
    pub fn semicolon_position(&self) -> JsSubstringPosition {
        self.0.semicolon_position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsExpressionStatement) -> bool {
        self == other
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.0.try_to_js_string()
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

impl From<JsExpressionStatement> for ExpressionStatement<OwnedString> {
    fn from(value: JsExpressionStatement) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, ExpressionStatement<OwnedString>> for JsValue {
    fn from_ref(value: &'rust ExpressionStatement<OwnedString>) -> Self {
        JsExpressionStatement(value.clone()).into()
    }
}

impl GetJsName for JsExpressionStatement {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsExpressionStatement {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for ExpressionStatement<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {})",
            JsExpressionStatement::JS_NAME,
            self.expression.try_to_js_string()?,
            self.semicolon_position.try_to_js_string()?,
            self.semicolon_fillers.try_to_js_string()?
        ))
    }
}
