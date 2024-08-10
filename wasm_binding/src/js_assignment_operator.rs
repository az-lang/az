use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{AssignmentOperator, Associativity, Precedence};

use super::js_associativity::JsAssociativity;
use super::js_precedence::JsPrecedence;

#[wasm_bindgen(js_name = "AssignmentOperator")]
pub struct JsAssignmentOperator(AssignmentOperator);

impl JsAssignmentOperator {
    const JS_NAME: &'static str = "AssignmentOperator";
    const OPERATOR: AssignmentOperator = AssignmentOperator;
}

#[wasm_bindgen(js_class = AssignmentOperator)]
impl JsAssignmentOperator {
    #[allow(dead_code)]
    #[wasm_bindgen(getter = ASSOCIATIVITY)]
    pub fn associativity() -> JsAssociativity {
        Associativity::from(Self::OPERATOR).into()
    }

    #[allow(dead_code)]
    #[wasm_bindgen(getter = PRECEDENCE)]
    pub fn precedence() -> JsPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }

    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsAssignmentOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}
