use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::{Precedence, ReturnOperator};

use crate::js_precedence::JsPrecedence;
use crate::traits::GetJsName;

#[wasm_bindgen(js_name = "ReturnOperator")]
pub(crate) struct JsReturnOperator;

impl JsReturnOperator {
    const JS_NAME: &'static str = "ReturnOperator";
    const OPERATOR: ReturnOperator = ReturnOperator;
}

#[wasm_bindgen(js_class = ReturnOperator)]
impl JsReturnOperator {
    #[allow(dead_code)]
    #[wasm_bindgen(getter = PRECEDENCE)]
    pub fn precedence() -> JsPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }

    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsReturnOperator, JsValue> {
        Err(js_sys::TypeError::new(&format!(
            "{} is not constructible",
            Self::JS_NAME
        ))
        .into())
    }
}

impl GetJsName for JsReturnOperator {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}
