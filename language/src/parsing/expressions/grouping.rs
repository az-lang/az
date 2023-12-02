use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Grouping<StringType> {
    pub expression: Box<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub close_parenthesis_position: SubstringPosition,
    pub open_parenthesis_fillers: Fillers<StringType>,
    pub close_parenthesis_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Grouping<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .open_parenthesis_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::OpenParenthesis,
            position: self.open_parenthesis_position,
        });
        result.append(&mut self.expression.tokenize());
        result.extend(
            self.close_parenthesis_fillers.into_iter().map(Into::into),
        );
        result.push(Token {
            content: TokenContent::CloseParenthesis,
            position: self.close_parenthesis_position,
        });
        result
    }
}

macro_rules! impl_grouping_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Grouping<$source_string_type>>
            for Grouping<$target_string_type>
        {
            fn from(value: Grouping<$source_string_type>) -> Self {
                Grouping {
                    expression: Box::new((*value.expression).into()),
                    open_parenthesis_position: value.open_parenthesis_position,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    close_parenthesis_fillers: value
                        .close_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_grouping_string_type_conversion!(&str, Arc<str>);
impl_grouping_string_type_conversion!(&str, Box<str>);
impl_grouping_string_type_conversion!(&str, Rc<str>);
impl_grouping_string_type_conversion!(&str, String);
impl_grouping_string_type_conversion!(Box<str>, Arc<str>);
impl_grouping_string_type_conversion!(Box<str>, Rc<str>);
impl_grouping_string_type_conversion!(Box<str>, String);
impl_grouping_string_type_conversion!(String, Arc<str>);
impl_grouping_string_type_conversion!(String, Box<str>);
impl_grouping_string_type_conversion!(String, Rc<str>);
