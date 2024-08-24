use wasm_bindgen::JsValue;

use az::parsing::UnaryArithmeticOperator;

use crate::traits::{FromRef, GetJsName, RefTo, TryFromRef, TryToJsString};
use crate::utils::js_value_to_js_string;

use super::js_unary_negation_operator::JsUnaryNegationOperator;

impl<'rust> FromRef<'rust, UnaryArithmeticOperator> for JsValue {
    fn from_ref(value: &'rust UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => js_sys::Object::constructor(
                &JsValue::from(JsUnaryNegationOperator).into(),
            )
            .into(),
        }
    }
}

impl<'js> TryFromRef<'js, JsValue> for UnaryArithmeticOperator {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        if js_sys::Object::is(
            value,
            &UnaryArithmeticOperator::Negation.ref_to(),
        ) {
            Ok(UnaryArithmeticOperator::Negation)
        } else {
            Err(JsValue::from(js_sys::TypeError::new(&format!(
                "{} is not valid unary arithmetic operator.",
                js_value_to_js_string(value)
            ))))
        }
    }
}

impl TryToJsString for UnaryArithmeticOperator {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(match self {
            UnaryArithmeticOperator::Negation => {
                JsUnaryNegationOperator::get_js_name()
            }
        })
    }
}
