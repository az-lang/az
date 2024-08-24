use wasm_bindgen::JsValue;

use az::parsing::Expression;

use crate::js_expressions::js_unidirectional_conditional::JsUnidirectionalConditional;
use crate::js_expressions::js_while_loop::JsWhileLoop;
use crate::traits::{
    FromRef, GetJsName, RefTo, ToJsTypeName, TryFromRef, TryToJsString,
};
use crate::types::OwnedString;

use super::js_annotated_identifier::JsAnnotatedIdentifier;
use super::js_assignment::JsAssignment;
use super::js_bidirectional_conditional::JsBidirectionalConditional;
use super::js_binary_arithmetic_operation::JsBinaryArithmeticOperation;
use super::js_binary_comparison::JsBinaryComparison;
use super::js_block::JsBlock;
use super::js_call::JsCall;
use super::js_function_definition::JsFunctionDefinition;
use super::js_function_type::JsFunctionType;
use super::js_grouping::JsGrouping;
use super::js_identifier::JsIdentifier;
use super::js_member_access::JsMemberAccess;
use super::js_numeric_literal::JsNumericLiteral;
use super::js_return::JsReturn;
use super::js_tuple::JsTuple;
use super::js_unary_arithmetic_operation::JsUnaryArithmeticOperation;

impl<'rust> FromRef<'rust, Expression<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Expression<OwnedString>) -> Self {
        match value {
            Expression::AnnotatedIdentifier(value) => value.ref_to(),
            Expression::Assignment(value) => value.ref_to(),
            Expression::BidirectionalConditional(value) => value.ref_to(),
            Expression::BinaryArithmeticOperation(value) => value.ref_to(),
            Expression::BinaryComparison(value) => value.ref_to(),
            Expression::Block(value) => value.ref_to(),
            Expression::Call(value) => value.ref_to(),
            Expression::FunctionDefinition(value) => value.ref_to(),
            Expression::FunctionType(value) => value.ref_to(),
            Expression::Grouping(value) => value.ref_to(),
            Expression::Identifier(value) => value.ref_to(),
            Expression::MemberAccess(value) => value.ref_to(),
            Expression::NumericLiteral(value) => value.ref_to(),
            Expression::Return(value) => value.ref_to(),
            Expression::Tuple(value) => value.ref_to(),
            Expression::UnaryArithmeticOperation(value) => value.ref_to(),
            Expression::UnidirectionalConditional(value) => value.ref_to(),
            Expression::WhileLoop(value) => value.ref_to(),
        }
    }
}

impl<'rust> FromRef<'rust, Vec<Expression<OwnedString>>> for Vec<JsValue> {
    fn from_ref(value: &'rust Vec<Expression<OwnedString>>) -> Self {
        value.iter().map(FromRef::from_ref).collect()
    }
}

impl<'js> TryFromRef<'js, JsValue> for Expression<OwnedString> {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        let js_type_name = value.to_js_type_name();
        if js_type_name == JsAnnotatedIdentifier::get_js_name() {
            JsAnnotatedIdentifier::try_from_ref(value)
                .map(|value| Expression::AnnotatedIdentifier(value.into()))
        } else if js_type_name == JsAssignment::get_js_name() {
            JsAssignment::try_from_ref(value)
                .map(|value| Expression::Assignment(value.into()))
        } else if js_type_name == JsBidirectionalConditional::get_js_name() {
            JsBidirectionalConditional::try_from_ref(value).map(|value| {
                Expression::BidirectionalConditional(value.into())
            })
        } else if js_type_name == JsBinaryArithmeticOperation::get_js_name() {
            JsBinaryArithmeticOperation::try_from_ref(value).map(|value| {
                Expression::BinaryArithmeticOperation(value.into())
            })
        } else if js_type_name == JsBinaryComparison::get_js_name() {
            JsBinaryComparison::try_from_ref(value)
                .map(|value| Expression::BinaryComparison(value.into()))
        } else if js_type_name == JsBlock::get_js_name() {
            JsBlock::try_from_ref(value)
                .map(|value| Expression::Block(value.into()))
        } else if js_type_name == JsCall::get_js_name() {
            JsCall::try_from_ref(value)
                .map(|value| Expression::Call(value.into()))
        } else if js_type_name == JsFunctionDefinition::get_js_name() {
            JsFunctionDefinition::try_from_ref(value)
                .map(|value| Expression::FunctionDefinition(value.into()))
        } else if js_type_name == JsFunctionType::get_js_name() {
            JsFunctionType::try_from_ref(value)
                .map(|value| Expression::FunctionType(value.into()))
        } else if js_type_name == JsGrouping::get_js_name() {
            JsGrouping::try_from_ref(value)
                .map(|value| Expression::Grouping(value.into()))
        } else if js_type_name == JsIdentifier::get_js_name() {
            JsIdentifier::try_from_ref(value)
                .map(|value| Expression::Identifier(value.into()))
        } else if js_type_name == JsMemberAccess::get_js_name() {
            JsMemberAccess::try_from_ref(value)
                .map(|value| Expression::MemberAccess(value.into()))
        } else if js_type_name == JsNumericLiteral::get_js_name() {
            JsNumericLiteral::try_from_ref(value)
                .map(|value| Expression::NumericLiteral(value.into()))
        } else if js_type_name == JsReturn::get_js_name() {
            JsReturn::try_from_ref(value)
                .map(|value| Expression::Return(value.into()))
        } else if js_type_name == JsTuple::get_js_name() {
            JsTuple::try_from_ref(value)
                .map(|value| Expression::Tuple(value.into()))
        } else if js_type_name == JsUnaryArithmeticOperation::get_js_name() {
            JsUnaryArithmeticOperation::try_from_ref(value).map(|value| {
                Expression::UnaryArithmeticOperation(value.into())
            })
        } else if js_type_name == JsUnidirectionalConditional::get_js_name() {
            JsUnidirectionalConditional::try_from_ref(value).map(|value| {
                Expression::UnidirectionalConditional(value.into())
            })
        } else if js_type_name == JsWhileLoop::get_js_name() {
            JsWhileLoop::try_from_ref(value)
                .map(|value| Expression::WhileLoop(value.into()))
        } else {
            Err(js_sys::TypeError::new(&format!(
                "can't convert {} to Expression",
                js_type_name
            ))
            .into())
        }
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<Expression<OwnedString>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| Expression::try_from_ref(&value))
            .collect()
    }
}

impl TryToJsString for Expression<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            Expression::AnnotatedIdentifier(value) => value.try_to_js_string(),
            Expression::Assignment(value) => value.try_to_js_string(),
            Expression::BidirectionalConditional(value) => {
                value.try_to_js_string()
            }
            Expression::BinaryArithmeticOperation(value) => {
                value.try_to_js_string()
            }
            Expression::BinaryComparison(value) => value.try_to_js_string(),
            Expression::Block(value) => value.try_to_js_string(),
            Expression::Call(value) => value.try_to_js_string(),
            Expression::FunctionDefinition(value) => value.try_to_js_string(),
            Expression::FunctionType(value) => value.try_to_js_string(),
            Expression::Grouping(value) => value.try_to_js_string(),
            Expression::Identifier(value) => value.try_to_js_string(),
            Expression::MemberAccess(value) => value.try_to_js_string(),
            Expression::NumericLiteral(value) => value.try_to_js_string(),
            Expression::Return(value) => value.try_to_js_string(),
            Expression::Tuple(value) => value.try_to_js_string(),
            Expression::UnaryArithmeticOperation(value) => {
                value.try_to_js_string()
            }
            Expression::UnidirectionalConditional(value) => {
                value.try_to_js_string()
            }
            Expression::WhileLoop(value) => value.try_to_js_string(),
        }
    }
}
