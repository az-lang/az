use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple<StringType> {
    pub elements: Vec<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub commas_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub open_parenthesis_fillers: Fillers<StringType>,
    pub commas_fillers: Vec<Fillers<StringType>>,
    pub close_parenthesis_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Tuple<StringType>
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
        if self.elements.len() > self.commas_positions.len() {
            let mut elements = self.elements;
            let last_element = unsafe { elements.pop().unwrap_unchecked() };
            elements
                .into_iter()
                .zip(
                    self.commas_fillers.into_iter().zip(self.commas_positions),
                )
                .for_each(|(element, (comma_fillers, comma_position))| {
                    result.append(&mut element.tokenize());
                    result.extend(comma_fillers.into_iter().map(Into::into));
                    result.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
            result.append(&mut last_element.tokenize());
        } else {
            self.elements
                .into_iter()
                .zip(
                    self.commas_fillers.into_iter().zip(self.commas_positions),
                )
                .for_each(|(element, (comma_fillers, comma_position))| {
                    result.append(&mut element.tokenize());
                    result.extend(comma_fillers.into_iter().map(Into::into));
                    result.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
        }
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

macro_rules! impl_tuple_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Tuple<$source_string_type>> for Tuple<$target_string_type> {
            fn from(value: Tuple<$source_string_type>) -> Self {
                Tuple {
                    elements: value
                        .elements
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    open_parenthesis_position: value.open_parenthesis_position,
                    commas_positions: value.commas_positions,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    commas_fillers: value
                        .commas_fillers
                        .into_iter()
                        .map(|fillers| {
                            fillers.into_iter().map(Into::into).collect()
                        })
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

impl_tuple_string_type_conversion!(&str, Arc<str>);
impl_tuple_string_type_conversion!(&str, Box<str>);
impl_tuple_string_type_conversion!(&str, Rc<str>);
impl_tuple_string_type_conversion!(&str, String);
impl_tuple_string_type_conversion!(Box<str>, Arc<str>);
impl_tuple_string_type_conversion!(Box<str>, Rc<str>);
impl_tuple_string_type_conversion!(Box<str>, String);
impl_tuple_string_type_conversion!(String, Arc<str>);
impl_tuple_string_type_conversion!(String, Box<str>);
impl_tuple_string_type_conversion!(String, Rc<str>);
