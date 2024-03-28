use std::rc::Rc;
use std::sync::Arc;

use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::expressions::Expression;
use super::filler::{Filler, Fillers};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Statement<StringType> {
    Expression(ExpressionStatement<StringType>),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement<StringType> {
    pub expression: Expression<StringType>,
    pub semicolon_position: SubstringPosition,
    pub semicolon_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Statement<StringType>
where
    ExpressionStatement<StringType>: Tokenize<TokenStringType>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        match self {
            Statement::Expression(expression) => expression.tokenize(),
        }
    }
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for ExpressionStatement<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self.expression.tokenize();
        result.extend(self.semicolon_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: TokenContent::Semicolon,
            position: self.semicolon_position,
        });
        result
    }
}

macro_rules! impl_statement_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Statement<$source_string_type>>
            for Statement<$target_string_type>
        {
            fn from(value: Statement<$source_string_type>) -> Self {
                match value {
                    Statement::Expression(expression) => {
                        Statement::Expression(expression.into())
                    }
                }
            }
        }
    };
}

impl_statement_string_type_conversion!(&str, Arc<str>);
impl_statement_string_type_conversion!(&str, Box<str>);
impl_statement_string_type_conversion!(&str, Rc<str>);
impl_statement_string_type_conversion!(&str, String);
impl_statement_string_type_conversion!(Box<str>, Arc<str>);
impl_statement_string_type_conversion!(Box<str>, Rc<str>);
impl_statement_string_type_conversion!(Box<str>, String);
impl_statement_string_type_conversion!(String, Arc<str>);
impl_statement_string_type_conversion!(String, Box<str>);
impl_statement_string_type_conversion!(String, Rc<str>);

macro_rules! impl_expression_statement_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<ExpressionStatement<$source_string_type>>
            for ExpressionStatement<$target_string_type>
        {
            fn from(value: ExpressionStatement<$source_string_type>) -> Self {
                ExpressionStatement {
                    expression: value.expression.into(),
                    semicolon_position: value.semicolon_position,
                    semicolon_fillers: value
                        .semicolon_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_expression_statement_string_type_conversion!(&str, Arc<str>);
impl_expression_statement_string_type_conversion!(&str, Box<str>);
impl_expression_statement_string_type_conversion!(&str, Rc<str>);
impl_expression_statement_string_type_conversion!(&str, String);
impl_expression_statement_string_type_conversion!(Box<str>, Arc<str>);
impl_expression_statement_string_type_conversion!(Box<str>, Rc<str>);
impl_expression_statement_string_type_conversion!(Box<str>, String);
impl_expression_statement_string_type_conversion!(String, Arc<str>);
impl_expression_statement_string_type_conversion!(String, Box<str>);
impl_expression_statement_string_type_conversion!(String, Rc<str>);
