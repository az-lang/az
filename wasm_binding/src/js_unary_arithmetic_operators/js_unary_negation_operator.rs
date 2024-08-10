use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Precedence, UnaryArithmeticOperator};

use crate::js_precedence::JsPrecedence;
use crate::traits::GetJsName;

#[wasm_bindgen(js_name = "UnaryNegationOperator")]
pub(crate) struct JsUnaryNegationOperator;

impl JsUnaryNegationOperator {
    const JS_NAME: &'static str = "UnaryNegationOperator";
    const OPERATOR: UnaryArithmeticOperator =
        UnaryArithmeticOperator::Negation;
}

#[wasm_bindgen(js_class = UnaryNegationOperator)]
impl JsUnaryNegationOperator {
    #[allow(dead_code)]
    #[wasm_bindgen(getter = PRECEDENCE)]
    pub fn precedence() -> JsPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }

    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsUnaryNegationOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}

impl GetJsName for JsUnaryNegationOperator {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}
