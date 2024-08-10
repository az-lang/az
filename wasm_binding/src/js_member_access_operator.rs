use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Associativity, MemberAccessOperator, Precedence};

use super::js_associativity::JsAssociativity;
use super::js_precedence::JsPrecedence;

#[wasm_bindgen(js_name = "MemberAccessOperator")]
pub struct JsMemberAccessOperator(MemberAccessOperator);

impl JsMemberAccessOperator {
    const JS_NAME: &'static str = "MemberAccessOperator";
    const OPERATOR: MemberAccessOperator = MemberAccessOperator;
}

#[wasm_bindgen(js_class = MemberAccessOperator)]
impl JsMemberAccessOperator {
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
    pub fn new() -> Result<JsMemberAccessOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}
