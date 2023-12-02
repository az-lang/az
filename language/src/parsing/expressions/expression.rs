use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::Filler;
use crate::tokenization::{Token, Tokenize};

use super::annotated_identifier::AnnotatedIdentifier;
use super::assignment::Assignment;
use super::binary_arithmetic_operation::BinaryArithmeticOperation;
use super::binary_comparison::BinaryComparison;
use super::block::Block;
use super::call::Call;
use super::conditional::Conditional;
use super::function_definition::FunctionDefinition;
use super::grouping::Grouping;
use super::identifier::Identifier;
use super::member_access::MemberAccess;
use super::numeric_literal::NumericLiteral;
use super::tuple::Tuple;
use super::unary_arithmetic_operation::UnaryArithmeticOperation;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Expression<StringType> {
    AnnotatedIdentifier(AnnotatedIdentifier<StringType>),
    Assignment(Assignment<StringType>),
    BinaryArithmeticOperation(BinaryArithmeticOperation<StringType>),
    BinaryComparison(BinaryComparison<StringType>),
    Block(Block<StringType>),
    Call(Call<StringType>),
    Conditional(Conditional<StringType>),
    FunctionDefinition(FunctionDefinition<StringType>),
    Grouping(Grouping<StringType>),
    Identifier(Identifier<StringType>),
    MemberAccess(MemberAccess<StringType>),
    NumericLiteral(NumericLiteral<StringType>),
    Tuple(Tuple<StringType>),
    UnaryArithmeticOperation(UnaryArithmeticOperation<StringType>),
}

impl<
        StringType: Into<TokenStringType>,
        TokenStringType: From<&'static str>,
    > Tokenize<TokenStringType> for Expression<StringType>
where
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        match self {
            Expression::AnnotatedIdentifier(value) => value.tokenize(),
            Expression::Assignment(value) => value.tokenize(),
            Expression::BinaryArithmeticOperation(value) => value.tokenize(),
            Expression::Block(value) => value.tokenize(),
            Expression::Call(value) => value.tokenize(),
            Expression::BinaryComparison(value) => value.tokenize(),
            Expression::Conditional(value) => value.tokenize(),
            Expression::FunctionDefinition(value) => value.tokenize(),
            Expression::Grouping(value) => value.tokenize(),
            Expression::Identifier(value) => value.tokenize(),
            Expression::MemberAccess(value) => value.tokenize(),
            Expression::NumericLiteral(value) => value.tokenize(),
            Expression::Tuple(value) => value.tokenize(),
            Expression::UnaryArithmeticOperation(value) => value.tokenize(),
        }
    }
}

macro_rules! impl_expression_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Expression<$source_string_type>>
            for Expression<$target_string_type>
        {
            fn from(value: Expression<$source_string_type>) -> Self {
                match value {
                    Expression::AnnotatedIdentifier(value) => {
                        Expression::AnnotatedIdentifier(value.into())
                    }
                    Expression::Assignment(value) => {
                        Expression::Assignment(value.into())
                    }
                    Expression::BinaryArithmeticOperation(value) => {
                        Expression::BinaryArithmeticOperation(value.into())
                    }
                    Expression::Block(value) => {
                        Expression::Block(value.into())
                    }
                    Expression::Call(value) => Expression::Call(value.into()),
                    Expression::BinaryComparison(value) => {
                        Expression::BinaryComparison(value.into())
                    }
                    Expression::Conditional(value) => {
                        Expression::Conditional(value.into())
                    }
                    Expression::FunctionDefinition(value) => {
                        Expression::FunctionDefinition(value.into())
                    }
                    Expression::Grouping(value) => {
                        Expression::Grouping(value.into())
                    }
                    Expression::Identifier(value) => {
                        Expression::Identifier(value.into())
                    }
                    Expression::MemberAccess(value) => {
                        Expression::MemberAccess(value.into())
                    }
                    Expression::NumericLiteral(value) => {
                        Expression::NumericLiteral(value.into())
                    }
                    Expression::Tuple(value) => {
                        Expression::Tuple(value.into())
                    }
                    Expression::UnaryArithmeticOperation(value) => {
                        Expression::UnaryArithmeticOperation(value.into())
                    }
                }
            }
        }
    };
}

impl_expression_string_type_conversion!(&str, Arc<str>);
impl_expression_string_type_conversion!(&str, Box<str>);
impl_expression_string_type_conversion!(&str, Rc<str>);
impl_expression_string_type_conversion!(&str, String);
impl_expression_string_type_conversion!(Box<str>, Arc<str>);
impl_expression_string_type_conversion!(Box<str>, Rc<str>);
impl_expression_string_type_conversion!(Box<str>, String);
impl_expression_string_type_conversion!(String, Arc<str>);
impl_expression_string_type_conversion!(String, Box<str>);
impl_expression_string_type_conversion!(String, Rc<str>);
