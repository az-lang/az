use std::rc::Rc;
use std::sync::Arc;

use crate::formatting::{
    format_script, reset_script_positions, CheckedFormatInSingleLine,
    FormatInMultipleLines, ResetPositions,
};
use crate::tokenization::{
    ByteCount, ByteSize, CharacterPosition, Token, TokenCollection, Tokenize,
    Utf8Count, Utf8Size,
};

use super::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, TERMINAL_FILLER_VEC_POSITION,
};
use super::filler_content::FillerContent;
use super::parser::Parser;
use super::parsing_error::ParsingError;
use super::positions_validation_step::PositionsValidationStep;
use super::statements::{
    validate_statement_vec_contents, validate_statement_vec_positions,
    Statement, StatementVecContentsValidationError,
    StatementVecPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Script<StringType> {
    pub statements: Vec<Statement<StringType>>,
    pub fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    Script<StringType>
where
    FillerContent<StringType>: ToString,
    Statement<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
{
    pub fn format(&mut self) {
        format_script(self)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Script<StringType> {
    pub fn reset_positions(&mut self)
    where
        FillerContent<StringType>: ToString,
        Statement<StringType>: ResetPositions,
    {
        reset_script_positions(self);
    }
}

impl<StringType: AsRef<str>> Script<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<ScriptContentsValidationError>> {
        let mut errors = vec![];
        if let Err(statement_errors) =
            validate_statement_vec_contents(&self.statements)
        {
            errors.extend(statement_errors.into_iter().map(|error| {
                ScriptContentsValidationError(
                    ScriptContentsValidationErrorKind::Statement(error),
                )
            }));
        }
        if let Err(filler_errors) = validate_filler_vec_contents::<
            TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.fillers)
        {
            errors.extend(filler_errors.into_iter().map(|error| {
                ScriptContentsValidationError(
                    ScriptContentsValidationErrorKind::Filler(error),
                )
            }));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Script<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        })
        .into_result()
    }

    fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<ScriptPositionsValidationError> {
        validate_statement_vec_positions(
            &self.statements,
            expected_start_character_position,
        )
        .map_error(|error| {
            ScriptPositionsValidationError(
                ScriptPositionsValidationErrorKind::Statement(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.fillers,
                expected_start_character_position,
            )
            .map_error(|error| {
                ScriptPositionsValidationError(
                    ScriptPositionsValidationErrorKind::Filler(error),
                )
            })
        })
    }
}

#[derive(Debug)]
struct ScriptContentsValidationError(ScriptContentsValidationErrorKind);

#[derive(Debug)]
struct ScriptPositionsValidationError(ScriptPositionsValidationErrorKind);

#[derive(Debug)]
enum ScriptContentsValidationErrorKind {
    Filler(FillerVecContentsValidationError),
    Statement(StatementVecContentsValidationError),
}

#[derive(Debug)]
enum ScriptPositionsValidationErrorKind {
    Filler(FillerVecPositionsValidationError),
    Statement(StatementVecPositionsValidationError),
}

impl std::fmt::Display for ScriptContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ScriptContentsValidationErrorKind::Filler(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ScriptContentsValidationErrorKind::Statement(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for ScriptPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ScriptPositionsValidationErrorKind::Filler(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ScriptPositionsValidationErrorKind::Statement(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for ScriptContentsValidationError {}

impl std::error::Error for ScriptPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Script<StringType>
where
    Statement<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .statements
            .into_iter()
            .flat_map(Tokenize::tokenize)
            .collect::<Vec<_>>();
        tokens.extend(self.fillers.into_iter().map(Into::into));
        TokenCollection::new(tokens)
    }
}

impl<
        StringType,
        TokenStringType: AsRef<str> + Into<StringType> + PartialEq,
    > TryFrom<TokenCollection<TokenStringType>> for Script<StringType>
{
    type Error = ParsingError<StringType, TokenStringType>;

    fn try_from(
        value: TokenCollection<TokenStringType>,
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
