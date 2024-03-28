use std::rc::Rc;
use std::sync::Arc;

use crate::formatting::{reset_script_positions, ResetPositions};
use crate::parsing::FillerContent;
use crate::tokenization::{ByteSize, Token, Tokenize, Utf8Size};

use super::filler::{Filler, Fillers};
use super::parser::Parser;
use super::parsing_error::ParsingError;
use super::statement::Statement;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Script<StringType> {
    pub statements: Vec<Statement<StringType>>,
    pub fillers: Fillers<StringType>,
}

impl<StringType: ByteSize + Utf8Size> Script<StringType> {
    pub fn reset_positions(&mut self)
    where
        FillerContent<StringType>: ToString,
        Statement<StringType>: ResetPositions,
    {
        reset_script_positions(self);
    }
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Script<StringType>
where
    Statement<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .statements
            .into_iter()
            .flat_map(Tokenize::tokenize)
            .collect::<Vec<_>>();
        result.extend(self.fillers.into_iter().map(Into::into));
        result
    }
}

impl<
        StringType,
        TokenStringType: AsRef<str> + Into<StringType> + PartialEq,
    > TryFrom<Vec<Token<TokenStringType>>> for Script<StringType>
{
    type Error = ParsingError<StringType, TokenStringType>;

    fn try_from(
        value: Vec<Token<TokenStringType>>,
    ) -> Result<Self, Self::Error> {
        let mut parser = Parser::new(value.into_iter());
        let mut statements = vec![];
        while let Some((token, fillers)) = parser.next() {
            let (next_statement, next_parser) =
                parser.parse_statement(token, fillers)?;
            parser = next_parser;
            statements.push(next_statement);
        }
        Ok(Script {
            statements,
            fillers: parser.into_fillers(),
        })
    }
}

macro_rules! impl_script_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Script<$source_string_type>>
            for Script<$target_string_type>
        {
            fn from(value: Script<$source_string_type>) -> Self {
                Script {
                    statements: value
                        .statements
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    fillers: value
                        .fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_script_string_type_conversion!(&str, Arc<str>);
impl_script_string_type_conversion!(&str, Box<str>);
impl_script_string_type_conversion!(&str, Rc<str>);
impl_script_string_type_conversion!(&str, String);
impl_script_string_type_conversion!(Box<str>, Arc<str>);
impl_script_string_type_conversion!(Box<str>, Rc<str>);
impl_script_string_type_conversion!(Box<str>, String);
impl_script_string_type_conversion!(String, Arc<str>);
impl_script_string_type_conversion!(String, Box<str>);
impl_script_string_type_conversion!(String, Rc<str>);
