use wasm_bindgen::JsValue;

use az::parsing::BinaryArithmeticOperator;

use crate::traits::{FromRef, GetJsName, RefTo, TryFromRef, TryToJsString};
use crate::utils::js_value_to_js_string;

use super::js_binary_addition_operator::JsBinaryAdditionOperator;
use super::js_binary_division_operator::JsBinaryDivisionOperator;
use super::js_binary_multiplication_operator::JsBinaryMultiplicationOperator;
use super::js_binary_subtraction_operator::JsBinarySubtractionOperator;

impl<'rust> FromRef<'rust, BinaryArithmeticOperator> for JsValue {
    fn from_ref(value: &'rust BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition => js_sys::Object::constructor(
                &JsValue::from(JsBinaryAdditionOperator).into(),
            )
            .into(),
            BinaryArithmeticOperator::Division => js_sys::Object::constructor(
                &JsValue::from(JsBinaryDivisionOperator).into(),
            )
            .into(),
            BinaryArithmeticOperator::Multiplication => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinaryMultiplicationOperator).into(),
                )
                .into()
            }
            BinaryArithmeticOperator::Subtraction => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinarySubtractionOperator).into(),
                )
                .into()
            }
        }
    }
}

impl<'js> TryFromRef<'js, JsValue> for BinaryArithmeticOperator {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        const OPERATORS: [BinaryArithmeticOperator; 4] = [
            BinaryArithmeticOperator::Addition,
            BinaryArithmeticOperator::Division,
            BinaryArithmeticOperator::Multiplication,
            BinaryArithmeticOperator::Subtraction,
        ];
        if let Some(operator) = OPERATORS
            .iter()
            .find(|candidate| {
                js_sys::Object::is(value, &(*candidate).ref_to())
            })
            .cloned()
        {
            Ok(operator)
        } else {
            Err(JsValue::from(js_sys::TypeError::new(&format!(
                "{} is not valid binary arithmetic operator.",
                js_value_to_js_string(value)
            ))))
        }
    }
}

impl TryToJsString for BinaryArithmeticOperator {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(match self {
            BinaryArithmeticOperator::Addition => {
                JsBinaryAdditionOperator::get_js_name()
            }
            BinaryArithmeticOperator::Division => {
                JsBinaryDivisionOperator::get_js_name()
            }
            BinaryArithmeticOperator::Multiplication => {
                JsBinaryMultiplicationOperator::get_js_name()
            }
            BinaryArithmeticOperator::Subtraction => {
                JsBinarySubtractionOperator::get_js_name()
            }
        })
    }
}
