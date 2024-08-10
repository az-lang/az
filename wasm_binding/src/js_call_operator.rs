use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Associativity, CallOperator, Precedence};

use super::js_associativity::JsAssociativity;
use super::js_precedence::JsPrecedence;

#[wasm_bindgen(js_name = "CallOperator")]
pub struct JsCallOperator(CallOperator);

impl JsCallOperator {
    const JS_NAME: &'static str = "CallOperator";
    const OPERATOR: CallOperator = CallOperator;
}

#[wasm_bindgen(js_class = CallOperator)]
impl JsCallOperator {
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
    pub fn new() -> Result<JsCallOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}
