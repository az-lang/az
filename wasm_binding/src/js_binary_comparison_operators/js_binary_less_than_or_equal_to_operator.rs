use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::js_associativity::JsAssociativity;
use crate::js_precedence::JsPrecedence;
use crate::traits::GetJsName;

#[wasm_bindgen(js_name = "BinaryLessThanOrEqualToOperator")]
pub(crate) struct JsBinaryLessThanOrEqualToOperator;

impl JsBinaryLessThanOrEqualToOperator {
    const JS_NAME: &'static str = "BinaryLessThanOrEqualToOperator";
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::LessThanOrEqualTo;
}

#[wasm_bindgen(js_class = BinaryLessThanOrEqualToOperator)]
impl JsBinaryLessThanOrEqualToOperator {
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
    pub fn new() -> Result<JsBinaryLessThanOrEqualToOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}

impl GetJsName for JsBinaryLessThanOrEqualToOperator {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}
