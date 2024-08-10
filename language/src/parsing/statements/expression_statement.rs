use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::expressions::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};
use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement<StringType> {
    pub expression: Expression<StringType>,
    pub semicolon_position: SubstringPosition,
    pub semicolon_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> ExpressionStatement<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<ExpressionStatementContentsValidationError>> {
        let mut errors = vec![];
        if let Err(expression_errors) =
            self.expression.validate_contents_impl()
        {
            errors.extend(expression_errors.into_iter().map(|error| {
                ExpressionStatementContentsValidationError(
                    ExpressionStatementContentsValidationErrorKind::Expression(
                        error,
                    ),
                )
            }));
        }
        if let Err(semicolon_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.semicolon_fillers)
        {
            errors.extend(semicolon_filler_errors.into_iter().map(|error| {
                ExpressionStatementContentsValidationError(
                    ExpressionStatementContentsValidationErrorKind::SemicolonFiller(
                        error,
                    ),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    ExpressionStatement<StringType>
{
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<ExpressionStatementPositionsValidationError>
    {
        self
            .expression
            .validate_positions_impl(expected_start_character_position)
            .map_error(
                |error| {
                    ExpressionStatementPositionsValidationError(
                        ExpressionStatementPositionsValidationErrorKind::Expression(
                            error,
                        ),
                    )
                },
            )
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.semicolon_fillers,
                    expected_start_character_position,
                )
                    .map_error(|error| { ExpressionStatementPositionsValidationError(
                        ExpressionStatementPositionsValidationErrorKind::SemicolonFiller(
                            error,
                        ),
                    )})
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::Semicolon.to_string(),
                    &self.semicolon_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionStatementPositionsValidationError(
                        ExpressionStatementPositionsValidationErrorKind::Semicolon(
                            error,
                        ),
                    )
                })
            })
    }
}

impl<StringType> ExpressionStatement<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.expression.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct ExpressionStatementContentsValidationError(
    ExpressionStatementContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct ExpressionStatementPositionsValidationError(
    ExpressionStatementPositionsValidationErrorKind,
);

#[derive(Debug)]
enum ExpressionStatementContentsValidationErrorKind {
    Expression(ExpressionContentsValidationError),
    SemicolonFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum ExpressionStatementPositionsValidationErrorKind {
    Expression(ExpressionPositionsValidationError),
    Semicolon(PositionsValidationError),
    SemicolonFiller(FillerVecPositionsValidationError),
}

impl std::fmt::Display for ExpressionStatementContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ExpressionStatementContentsValidationErrorKind::Expression(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionStatementContentsValidationErrorKind::SemicolonFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::fmt::Display for ExpressionStatementPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ExpressionStatementPositionsValidationErrorKind::Expression(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionStatementPositionsValidationErrorKind::Semicolon(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionStatementPositionsValidationErrorKind::SemicolonFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for ExpressionStatementContentsValidationError {}

impl std::error::Error for ExpressionStatementPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for ExpressionStatement<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.expression.tokenize().into_iter().collect::<Vec<_>>();
        tokens.extend(self.semicolon_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: TokenContent::Semicolon,
            position: self.semicolon_position,
        });
        TokenCollection::new(tokens)
    }
}

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
