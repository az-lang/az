use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::statement::Statement;
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Block<StringType> {
    pub statements: Vec<Statement<StringType>>,
    pub expression: Option<Box<Expression<StringType>>>,
    pub open_brace_position: SubstringPosition,
    pub close_brace_position: SubstringPosition,
    pub open_brace_fillers: Fillers<StringType>,
    pub close_brace_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Block<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .open_brace_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::OpenBrace,
            position: self.open_brace_position,
        });
        result.append(
            &mut self
                .statements
                .into_iter()
                .flat_map(Tokenize::tokenize)
                .collect(),
        );
        result.append(
            &mut self
                .expression
                .into_iter()
                .flat_map(|expression| (*expression).tokenize())
                .collect(),
        );
        result.extend(self.close_brace_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: TokenContent::CloseBrace,
            position: self.close_brace_position,
        });
        result
    }
}

macro_rules! impl_block_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Block<$source_string_type>> for Block<$target_string_type> {
            fn from(value: Block<$source_string_type>) -> Self {
                Block {
                    statements: value
                        .statements
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    expression: value
                        .expression
                        .map(|expression| Box::new((*expression).into())),
                    open_brace_position: value.open_brace_position,
                    close_brace_position: value.close_brace_position,
                    open_brace_fillers: value
                        .open_brace_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    close_brace_fillers: value
                        .close_brace_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_block_string_type_conversion!(&str, Arc<str>);
impl_block_string_type_conversion!(&str, Box<str>);
impl_block_string_type_conversion!(&str, Rc<str>);
impl_block_string_type_conversion!(&str, String);
impl_block_string_type_conversion!(Box<str>, Arc<str>);
impl_block_string_type_conversion!(Box<str>, Rc<str>);
impl_block_string_type_conversion!(Box<str>, String);
impl_block_string_type_conversion!(String, Arc<str>);
impl_block_string_type_conversion!(String, Box<str>);
impl_block_string_type_conversion!(String, Rc<str>);
