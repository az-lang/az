use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::parsing::statements::{
    validate_statement_vec_contents, validate_statement_vec_positions,
    Statement, StatementVecContentsValidationError,
    StatementVecPositionsValidationError,
};
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::expression::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Block<StringType> {
    pub statements: Vec<Statement<StringType>>,
    pub expression: Option<Box<Expression<StringType>>>,
    pub open_brace_position: SubstringPosition,
    pub close_brace_position: SubstringPosition,
    pub open_brace_fillers: FillerVec<StringType>,
    pub close_brace_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Block<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<BlockContentsValidationError>> {
        let mut errors = vec![];
        if let Err(open_brace_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_brace_fillers)
        {
            errors.extend(open_brace_filler_errors.into_iter().map(|error| {
                BlockContentsValidationError(
                    BlockContentsValidationErrorKind::OpenBraceFiller(error),
                )
            }));
        }
        if let Err(statement_errors) =
            validate_statement_vec_contents(&self.statements)
        {
            errors.extend(statement_errors.into_iter().map(|error| {
                BlockContentsValidationError(
                    BlockContentsValidationErrorKind::Statement(Box::new(
                        error,
                    )),
                )
            }));
        }
        if let Some(expression) = &self.expression {
            if let Err(expression_errors) = expression.validate_contents_impl()
            {
                errors.extend(expression_errors.into_iter().map(|error| {
                    BlockContentsValidationError(
                        BlockContentsValidationErrorKind::Expression(
                            Box::new(error),
                        ),
                    )
                }));
            }
        }
        if let Err(close_brace_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.close_brace_fillers)
        {
            errors.extend(close_brace_filler_errors.into_iter().map(
                |error| {
                    BlockContentsValidationError(
                        BlockContentsValidationErrorKind::CloseBraceFiller(
                            error,
                        ),
                    )
                },
            ));
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Block<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<BlockPositionsValidationError> {
        validate_filler_vec_positions(
            &self.open_brace_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            BlockPositionsValidationError(
                BlockPositionsValidationErrorKind::OpenBraceFiller(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::OpenBrace.to_string(),
                &self.open_brace_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                BlockPositionsValidationError(
                    BlockPositionsValidationErrorKind::OpenBrace(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            validate_statement_vec_positions(
                &self.statements,
                expected_start_character_position,
            )
            .map_error(|error| {
                BlockPositionsValidationError(
                    BlockPositionsValidationErrorKind::Statement(Box::new(
                        error,
                    )),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            if let Some(expression) = &self.expression {
                expression
                    .validate_positions_impl(expected_start_character_position)
                    .map_error(|error| {
                        BlockPositionsValidationError(
                            BlockPositionsValidationErrorKind::Expression(
                                Box::new(error),
                            ),
                        )
                    })
            } else {
                PositionsValidationStep {
                    expected_end_character_position:
                        expected_start_character_position,
                    errors: vec![],
                }
            }
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.close_brace_fillers,
                expected_start_character_position,
            )
            .map_error(|error| {
                BlockPositionsValidationError(
                    BlockPositionsValidationErrorKind::CloseBraceFiller(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::CloseBrace.to_string(),
                &self.close_brace_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                BlockPositionsValidationError(
                    BlockPositionsValidationErrorKind::CloseBrace(error),
                )
            })
        })
    }
}

impl<StringType> Block<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.open_brace_fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.open_brace_position.start)
    }
}

#[derive(Debug)]
pub(super) struct BlockContentsValidationError(
    BlockContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct BlockPositionsValidationError(
    BlockPositionsValidationErrorKind,
);

#[derive(Debug)]
enum BlockContentsValidationErrorKind {
    CloseBraceFiller(FillerVecContentsValidationError),
    Expression(Box<ExpressionContentsValidationError>),
    OpenBraceFiller(FillerVecContentsValidationError),
    Statement(Box<StatementVecContentsValidationError>),
}

#[derive(Debug)]
enum BlockPositionsValidationErrorKind {
    CloseBrace(PositionsValidationError),
    CloseBraceFiller(FillerVecPositionsValidationError),
    Expression(Box<ExpressionPositionsValidationError>),
    OpenBrace(PositionsValidationError),
    OpenBraceFiller(FillerVecPositionsValidationError),
    Statement(Box<StatementVecPositionsValidationError>),
}

impl std::fmt::Display for BlockContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BlockContentsValidationErrorKind::CloseBraceFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockContentsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockContentsValidationErrorKind::OpenBraceFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockContentsValidationErrorKind::Statement(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for BlockPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BlockPositionsValidationErrorKind::CloseBrace(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockPositionsValidationErrorKind::CloseBraceFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockPositionsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockPositionsValidationErrorKind::OpenBrace(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockPositionsValidationErrorKind::OpenBraceFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BlockPositionsValidationErrorKind::Statement(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for BlockContentsValidationError {}

impl std::error::Error for BlockPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Block<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
    Statement<StringType>: Tokenize<TokenStringType>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .open_brace_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        tokens.push(Token {
            content: TokenContent::OpenBrace,
            position: self.open_brace_position,
        });
        tokens
            .extend(self.statements.into_iter().flat_map(Tokenize::tokenize));
        tokens.extend(
            self.expression
                .into_iter()
                .flat_map(|expression| expression.tokenize()),
        );
        tokens.extend(self.close_brace_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: TokenContent::CloseBrace,
            position: self.close_brace_position,
        });
        TokenCollection::new(tokens)
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
