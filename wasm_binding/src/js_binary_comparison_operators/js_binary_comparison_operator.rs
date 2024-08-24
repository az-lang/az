use wasm_bindgen::JsValue;

use az::parsing::BinaryComparisonOperator;

use crate::traits::{FromRef, GetJsName, RefTo, TryFromRef, TryToJsString};
use crate::utils::js_value_to_js_string;

use super::js_binary_equal_to_operator::JsBinaryEqualToOperator;
use super::js_binary_greater_than_operator::JsBinaryGreaterThanOperator;
use super::js_binary_greater_than_or_equal_to_operator::JsBinaryGreaterThanOrEqualToOperator;
use super::js_binary_less_than_operator::JsBinaryLessThanOperator;
use super::js_binary_less_than_or_equal_to_operator::JsBinaryLessThanOrEqualToOperator;
use super::js_binary_not_equal_to_operator::JsBinaryNotEqualToOperator;

impl<'rust> FromRef<'rust, BinaryComparisonOperator> for JsValue {
    fn from_ref(value: &'rust BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo => js_sys::Object::constructor(
                &JsValue::from(JsBinaryEqualToOperator).into(),
            )
            .into(),
            BinaryComparisonOperator::GreaterThan => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinaryGreaterThanOperator).into(),
                )
                .into()
            }
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinaryGreaterThanOrEqualToOperator)
                        .into(),
                )
                .into()
            }
            BinaryComparisonOperator::LessThan => js_sys::Object::constructor(
                &JsValue::from(JsBinaryLessThanOperator).into(),
            )
            .into(),
            BinaryComparisonOperator::LessThanOrEqualTo => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinaryLessThanOrEqualToOperator).into(),
                )
                .into()
            }
            BinaryComparisonOperator::NotEqualTo => {
                js_sys::Object::constructor(
                    &JsValue::from(JsBinaryNotEqualToOperator).into(),
                )
                .into()
            }
        }
    }
}

impl<'js> TryFromRef<'js, JsValue> for BinaryComparisonOperator {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        const OPERATORS: [BinaryComparisonOperator; 6] = [
            BinaryComparisonOperator::EqualTo,
            BinaryComparisonOperator::GreaterThan,
            BinaryComparisonOperator::GreaterThanOrEqualTo,
            BinaryComparisonOperator::LessThan,
            BinaryComparisonOperator::LessThanOrEqualTo,
            BinaryComparisonOperator::NotEqualTo,
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
                "{} is not valid binary comparison operator.",
                js_value_to_js_string(value)
            ))))
        }
    }
}

impl TryToJsString for BinaryComparisonOperator {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(match self {
            BinaryComparisonOperator::EqualTo => {
                JsBinaryEqualToOperator::get_js_name()
            }
            BinaryComparisonOperator::GreaterThan => {
                JsBinaryGreaterThanOperator::get_js_name()
            }
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                JsBinaryGreaterThanOrEqualToOperator::get_js_name()
            }
            BinaryComparisonOperator::LessThan => {
                JsBinaryLessThanOperator::get_js_name()
            }
            BinaryComparisonOperator::LessThanOrEqualTo => {
                JsBinaryLessThanOrEqualToOperator::get_js_name()
            }
            BinaryComparisonOperator::NotEqualTo => {
                JsBinaryNotEqualToOperator::get_js_name()
            }
        })
    }
}
