use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Associativity, BinaryArithmeticOperator, Precedence};

use crate::js_associativity::JsAssociativity;
use crate::js_precedence::JsPrecedence;
use crate::traits::GetJsName;

#[wasm_bindgen(js_name = "BinaryDivisionOperator")]
pub(crate) struct JsBinaryDivisionOperator;

impl JsBinaryDivisionOperator {
    const JS_NAME: &'static str = "BinaryDivisionOperator";
    const OPERATOR: BinaryArithmeticOperator =
        BinaryArithmeticOperator::Division;
}

#[wasm_bindgen(js_class = BinaryDivisionOperator)]
impl JsBinaryDivisionOperator {
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
    pub fn new() -> Result<JsBinaryDivisionOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}

impl GetJsName for JsBinaryDivisionOperator {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}
