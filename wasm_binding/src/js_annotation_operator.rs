use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{AnnotationOperator, Associativity, Precedence};

use crate::js_associativity::JsAssociativity;
use crate::js_precedence::JsPrecedence;

#[wasm_bindgen(js_name = "AnnotationOperator")]
pub struct JsAnnotationOperator(AnnotationOperator);

impl JsAnnotationOperator {
    const JS_NAME: &'static str = "AnnotationOperator";
    const OPERATOR: AnnotationOperator = AnnotationOperator;
}

#[wasm_bindgen(js_class = AnnotationOperator)]
impl JsAnnotationOperator {
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
    pub fn new() -> Result<JsAnnotationOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}
