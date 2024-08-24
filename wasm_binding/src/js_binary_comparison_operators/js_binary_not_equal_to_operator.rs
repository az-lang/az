use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::js_associativity::JsAssociativity;
use crate::js_precedence::JsPrecedence;
use crate::traits::GetJsName;

#[wasm_bindgen(js_name = "BinaryNotEqualToOperator")]
pub(crate) struct JsBinaryNotEqualToOperator;

impl JsBinaryNotEqualToOperator {
    const JS_NAME: &'static str = "BinaryNotEqualToOperator";
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::NotEqualTo;
}

#[wasm_bindgen(js_class = BinaryNotEqualToOperator)]
impl JsBinaryNotEqualToOperator {
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
    pub fn new() -> Result<JsBinaryNotEqualToOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}

impl GetJsName for JsBinaryNotEqualToOperator {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}
