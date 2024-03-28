use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::keywords::FUNCTION_OPENER;
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::block::Block;
use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition<StringType> {
    pub parameters: Vec<Expression<StringType>>,
    pub return_type: Box<Expression<StringType>>,
    pub body: Block<StringType>,
    pub opener_position: SubstringPosition,
    pub open_parenthesis_position: SubstringPosition,
    pub commas_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub arrow_position: SubstringPosition,
    pub opener_fillers: Fillers<StringType>,
    pub open_parenthesis_fillers: Fillers<StringType>,
    pub commas_fillers: Vec<Fillers<StringType>>,
    pub close_parenthesis_fillers: Fillers<StringType>,
    pub arrow_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for FunctionDefinition<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .opener_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::Identifier(TokenStringType::from(
                FUNCTION_OPENER,
            )),
            position: self.opener_position,
        });
        result
            .extend(self.open_parenthesis_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: TokenContent::OpenParenthesis,
            position: self.open_parenthesis_position,
        });
        if self.parameters.len() > self.commas_positions.len() {
            let mut parameters = self.parameters;
            let last_parameter =
                unsafe { parameters.pop().unwrap_unchecked() };
            parameters
                .into_iter()
                .zip(
                    self.commas_fillers.into_iter().zip(self.commas_positions),
                )
                .for_each(|(parameter, (comma_fillers, comma_position))| {
                    result.append(&mut parameter.tokenize());
                    result.extend(comma_fillers.into_iter().map(Into::into));
                    result.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
            result.append(&mut last_parameter.tokenize());
        } else {
            self.parameters
                .into_iter()
                .zip(
                    self.commas_fillers.into_iter().zip(self.commas_positions),
                )
                .for_each(|(parameter, (comma_fillers, comma_position))| {
                    result.append(&mut parameter.tokenize());
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
        result.extend(self.arrow_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: TokenContent::Arrow,
            position: self.arrow_position,
        });
        result.append(&mut self.return_type.tokenize());
        result.append(&mut self.body.tokenize());
        result
    }
}

macro_rules! impl_function_definition_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<FunctionDefinition<$source_string_type>>
            for FunctionDefinition<$target_string_type>
        {
            fn from(value: FunctionDefinition<$source_string_type>) -> Self {
                FunctionDefinition {
                    parameters: value
                        .parameters
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    return_type: Box::new((*value.return_type).into()),
                    body: value.body.into(),
                    opener_position: value.opener_position,
                    open_parenthesis_position: value.open_parenthesis_position,
                    commas_positions: value.commas_positions,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    arrow_position: value.arrow_position,
                    opener_fillers: value
                        .opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    commas_fillers: value
                        .commas_fillers
                        .into_iter()
                        .map(|comma_fillers| {
                            comma_fillers.into_iter().map(Into::into).collect()
                        })
                        .collect(),
                    close_parenthesis_fillers: value
                        .close_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    arrow_fillers: value
                        .arrow_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_function_definition_string_type_conversion!(&str, Arc<str>);
impl_function_definition_string_type_conversion!(&str, Box<str>);
impl_function_definition_string_type_conversion!(&str, Rc<str>);
impl_function_definition_string_type_conversion!(&str, String);
impl_function_definition_string_type_conversion!(Box<str>, Arc<str>);
impl_function_definition_string_type_conversion!(Box<str>, Rc<str>);
impl_function_definition_string_type_conversion!(Box<str>, String);
impl_function_definition_string_type_conversion!(String, Arc<str>);
impl_function_definition_string_type_conversion!(String, Box<str>);
impl_function_definition_string_type_conversion!(String, Rc<str>);
