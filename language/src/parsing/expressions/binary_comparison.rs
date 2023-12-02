use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::operators::BinaryComparisonOperator;
use crate::tokenization::{SubstringPosition, Token, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryComparison<StringType> {
    pub left: Box<Expression<StringType>>,
    pub right: Box<Expression<StringType>>,
    pub operator: BinaryComparisonOperator,
    pub operator_position: SubstringPosition,
    pub operator_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for BinaryComparison<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self.left.tokenize();
        result.extend(self.operator_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: self.operator.into(),
            position: self.operator_position,
        });
        result.append(&mut self.right.tokenize());
        result
    }
}

macro_rules! impl_comparison_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<BinaryComparison<$source_string_type>>
            for BinaryComparison<$target_string_type>
        {
            fn from(value: BinaryComparison<$source_string_type>) -> Self {
                BinaryComparison {
                    left: Box::new((*value.left).into()),
                    right: Box::new((*value.right).into()),
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

impl_comparison_string_type_conversion!(&str, Arc<str>);
impl_comparison_string_type_conversion!(&str, Box<str>);
impl_comparison_string_type_conversion!(&str, Rc<str>);
impl_comparison_string_type_conversion!(&str, String);
impl_comparison_string_type_conversion!(Box<str>, Arc<str>);
impl_comparison_string_type_conversion!(Box<str>, Rc<str>);
impl_comparison_string_type_conversion!(Box<str>, String);
impl_comparison_string_type_conversion!(String, Arc<str>);
impl_comparison_string_type_conversion!(String, Box<str>);
impl_comparison_string_type_conversion!(String, Rc<str>);
