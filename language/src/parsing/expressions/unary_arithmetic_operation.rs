use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::operators::UnaryArithmeticOperator;
use crate::tokenization::{SubstringPosition, Token, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct UnaryArithmeticOperation<StringType> {
    pub operand: Box<Expression<StringType>>,
    pub operator: UnaryArithmeticOperator,
    pub operator_position: SubstringPosition,
    pub operator_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .operator_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        result.push(Token {
            content: self.operator.into(),
            position: self.operator_position,
        });
        result.append(&mut self.operand.tokenize());
        result
    }
}

macro_rules! impl_unary_arithmetic_operation_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<UnaryArithmeticOperation<$source_string_type>>
            for UnaryArithmeticOperation<$target_string_type>
        {
            fn from(
                value: UnaryArithmeticOperation<$source_string_type>,
            ) -> Self {
                UnaryArithmeticOperation {
                    operand: Box::new((*value.operand).into()),
                    operator: value.operator,
                    operator_position: value.operator_position,
                    operator_fillers: value
                        .operator_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_unary_arithmetic_operation_string_type_conversion!(&str, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, Box<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, Rc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, String);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, Rc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, String);
impl_unary_arithmetic_operation_string_type_conversion!(String, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(String, Box<str>);
impl_unary_arithmetic_operation_string_type_conversion!(String, Rc<str>);
