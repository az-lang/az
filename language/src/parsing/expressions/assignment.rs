use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::operators::BinaryAssignmentOperator;
use crate::tokenization::{SubstringPosition, Token, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment<StringType> {
    pub target: Box<Expression<StringType>>,
    pub value: Box<Expression<StringType>>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Assignment<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self.target.tokenize();
        result.extend(self.operator_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: BinaryAssignmentOperator.into(),
            position: self.operator_position,
        });
        result.append(&mut self.value.tokenize());
        result
    }
}

macro_rules! impl_assignment_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Assignment<$source_string_type>>
            for Assignment<$target_string_type>
        {
            fn from(value: Assignment<$source_string_type>) -> Self {
                Assignment {
                    target: Box::new((*value.target).into()),
                    value: Box::new((*value.value).into()),
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

impl_assignment_string_type_conversion!(&str, Arc<str>);
impl_assignment_string_type_conversion!(&str, Box<str>);
impl_assignment_string_type_conversion!(&str, Rc<str>);
impl_assignment_string_type_conversion!(&str, String);
impl_assignment_string_type_conversion!(Box<str>, Arc<str>);
impl_assignment_string_type_conversion!(Box<str>, Rc<str>);
impl_assignment_string_type_conversion!(Box<str>, String);
impl_assignment_string_type_conversion!(String, Arc<str>);
impl_assignment_string_type_conversion!(String, Box<str>);
impl_assignment_string_type_conversion!(String, Rc<str>);
