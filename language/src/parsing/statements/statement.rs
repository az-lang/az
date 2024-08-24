use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, TokenCollection, Tokenize, Utf8Size,
};

use super::expression_statement::{
    ExpressionStatement, ExpressionStatementContentsValidationError,
    ExpressionStatementPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Statement<StringType> {
    Expression(ExpressionStatement<StringType>),
}

impl<StringType: AsRef<str>> Statement<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<StatementContentsValidationError>> {
        match self {
            Statement::Expression(statement) => {
                statement.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            StatementContentsValidationError(
                                StatementContentsValidationErrorKind::Expression(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Statement<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<StatementPositionsValidationError> {
        match self {
            Statement::Expression(value) => value
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    StatementPositionsValidationError(
                        StatementPositionsValidationErrorKind::Expression(
                            error,
                        ),
                    )
                }),
        }
    }
}

impl<StringType> Statement<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        match self {
            Statement::Expression(value) => {
                value.to_first_start_character_position()
            }
        }
    }
}

pub(crate) fn validate_statement_vec_contents<StringType: AsRef<str>>(
    statement_vec: &[Statement<StringType>],
) -> Result<(), Vec<StatementVecContentsValidationError>> {
    let mut errors = vec![];
    for (index, statement) in statement_vec.iter().enumerate() {
        if let Err(statement_errors) = statement.validate_contents_impl() {
            errors.extend(statement_errors.into_iter().map(|error| {
                StatementVecContentsValidationError { error, index }
            }));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_statement_vec_positions<
    StringType: AsRef<str> + ByteSize + Utf8Size,
>(
    statement_vec: &[Statement<StringType>],
    expected_start_character_position: CharacterPosition,
) -> PositionsValidationStep<StatementVecPositionsValidationError> {
    match statement_vec.split_first() {
        Some((first, rest)) => {
            let mut result = first
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| StatementVecPositionsValidationError {
                    error,
                    index: 0usize,
                });
            for (offset, statement) in rest.iter().enumerate() {
                result =
                    result.merge_with(|expected_start_character_position| {
                        statement
                            .validate_positions_impl(
                                expected_start_character_position,
                            )
                            .map_error(|error| {
                                StatementVecPositionsValidationError {
                                    error,
                                    index: 1usize + offset,
                                }
                            })
                    });
            }
            result
        }
        None => PositionsValidationStep {
            expected_end_character_position: expected_start_character_position,
            errors: vec![],
        },
    }
}

#[derive(Debug)]
pub(crate) struct StatementVecContentsValidationError {
    error: StatementContentsValidationError,
    index: usize,
}

#[derive(Debug)]
pub(crate) struct StatementVecPositionsValidationError {
    error: StatementPositionsValidationError,
    index: usize,
}

#[derive(Debug)]
struct StatementContentsValidationError(StatementContentsValidationErrorKind);

#[derive(Debug)]
struct StatementPositionsValidationError(
    StatementPositionsValidationErrorKind,
);

#[derive(Debug)]
enum StatementContentsValidationErrorKind {
    Expression(ExpressionStatementContentsValidationError),
}

#[derive(Debug)]
enum StatementPositionsValidationErrorKind {
    Expression(ExpressionStatementPositionsValidationError),
}

impl std::fmt::Display for StatementContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            StatementContentsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for StatementPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            StatementPositionsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for StatementVecContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid statement contents with index {}: {}",
            self.index, self.error
        )
    }
}

impl std::fmt::Display for StatementVecPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid statement positions with index {}: {}",
            self.index, self.error
        )
    }
}

impl std::error::Error for StatementContentsValidationError {}

impl std::error::Error for StatementPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Statement<StringType>
where
    ExpressionStatement<StringType>: Tokenize<TokenStringType>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        match self {
            Statement::Expression(expression) => expression.tokenize(),
        }
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
