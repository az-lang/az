use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::AnnotationOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::binary_operand::{
    validate_maybe_nested_right_binary_operand, BinaryOperandMetadata,
    BinaryRightOperandValidationError, RIGHT_BINARY_OPERAND_POSITION,
};
use super::expression::{Expression, ExpressionPositionsValidationError};
use super::identifier::{
    Identifier, IdentifierContentsValidationError,
    IdentifierPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedIdentifier<StringType> {
    pub identifier: Identifier<StringType>,
    pub annotation: Box<Expression<StringType>>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> AnnotatedIdentifier<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<AnnotatedIdentifierContentsValidationError>> {
        let mut errors = vec![];
        if let Err(identifier_errors) = self
            .identifier
            .validate_contents_impl(self.identifier.position.start)
        {
            errors.extend(identifier_errors.into_iter().map(|error| {
                AnnotatedIdentifierContentsValidationError(
                    AnnotatedIdentifierContentsValidationErrorKind::Identifier(
                        error,
                    ),
                )
            }));
        }
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(operator_filler_errors.into_iter().map(|error| {
                AnnotatedIdentifierContentsValidationError(
                    AnnotatedIdentifierContentsValidationErrorKind::OperatorFiller(
                        error,
                    ),
                )
            }));
        }
        errors.extend(
            validate_maybe_nested_right_binary_operand(
                self.annotation.as_ref(),
                AnnotationOperator,
            )
            .into_iter()
            .map(|error| {
                AnnotatedIdentifierContentsValidationError(
                    AnnotatedIdentifierContentsValidationErrorKind::Annotation(
                        error,
                    ),
                )
            }),
        );
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    AnnotatedIdentifier<StringType>
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
    ) -> PositionsValidationStep<AnnotatedIdentifierPositionsValidationError>
    {
        self
            .identifier
            .validate_positions_impl(expected_start_character_position)
            .map_error(
                |error| {
                    AnnotatedIdentifierPositionsValidationError(
                        AnnotatedIdentifierPositionsValidationErrorKind::Identifier(
                            error,
                        ),
                    )
                },
            )
            .merge_with(|expected_start_character_position| {
                    validate_filler_vec_positions(
                        &self.operator_fillers,
                        expected_start_character_position,
                    )
                        .map_error(|error| { AnnotatedIdentifierPositionsValidationError(
                            AnnotatedIdentifierPositionsValidationErrorKind::OperatorFiller(
                                error,
                            ),
                        )})
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::from(AnnotationOperator)
                        .to_string(),
                    &self.operator_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    AnnotatedIdentifierPositionsValidationError(
                        AnnotatedIdentifierPositionsValidationErrorKind::Operator(
                            error,
                        ),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                self.annotation.validate_positions_impl(
                    expected_start_character_position,
                ).map_error(|error| { AnnotatedIdentifierPositionsValidationError(
                    AnnotatedIdentifierPositionsValidationErrorKind::Annotation(
                        Box::new(error),
                    ),
                )})
            })
    }
}

impl<StringType> AnnotatedIdentifier<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.identifier.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct AnnotatedIdentifierContentsValidationError(
    AnnotatedIdentifierContentsValidationErrorKind,
);

#[derive(Debug)]
enum AnnotatedIdentifierContentsValidationErrorKind {
    Annotation(BinaryRightOperandValidationError<AnnotationOperator>),
    Identifier(IdentifierContentsValidationError),
    OperatorFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
pub(super) struct AnnotatedIdentifierPositionsValidationError(
    AnnotatedIdentifierPositionsValidationErrorKind,
);

#[derive(Debug)]
enum AnnotatedIdentifierPositionsValidationErrorKind {
    Annotation(Box<ExpressionPositionsValidationError>),
    Identifier(IdentifierPositionsValidationError),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
}

impl BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>
    for AnnotationOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "annotation"
    }
}

impl std::fmt::Display for AnnotatedIdentifierContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            AnnotatedIdentifierContentsValidationErrorKind::Annotation(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            AnnotatedIdentifierContentsValidationErrorKind::Identifier(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            AnnotatedIdentifierContentsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::fmt::Display for AnnotatedIdentifierPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            AnnotatedIdentifierPositionsValidationErrorKind::Annotation(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            AnnotatedIdentifierPositionsValidationErrorKind::Identifier(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            AnnotatedIdentifierPositionsValidationErrorKind::Operator(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            AnnotatedIdentifierPositionsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for AnnotatedIdentifierContentsValidationError {}

impl std::error::Error for AnnotatedIdentifierPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for AnnotatedIdentifier<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Identifier<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.identifier.tokenize().into_iter().collect::<Vec<_>>();
        tokens.extend(self.operator_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: AnnotationOperator.into(),
            position: self.operator_position,
        });
        tokens.extend(self.annotation.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_annotated_identifier_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<AnnotatedIdentifier<$source_string_type>>
            for AnnotatedIdentifier<$target_string_type>
        {
            fn from(value: AnnotatedIdentifier<$source_string_type>) -> Self {
                AnnotatedIdentifier {
                    identifier: value.identifier.into(),
                    annotation: Box::new((*value.annotation).into()),
                    operator_position: value.operator_position,
                    operator_fillers: value
                        .operator_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_annotated_identifier_string_type_conversion!(&str, Arc<str>);
impl_annotated_identifier_string_type_conversion!(&str, Box<str>);
impl_annotated_identifier_string_type_conversion!(&str, Rc<str>);
impl_annotated_identifier_string_type_conversion!(&str, String);
impl_annotated_identifier_string_type_conversion!(Box<str>, Arc<str>);
impl_annotated_identifier_string_type_conversion!(Box<str>, Rc<str>);
impl_annotated_identifier_string_type_conversion!(Box<str>, String);
impl_annotated_identifier_string_type_conversion!(String, Arc<str>);
impl_annotated_identifier_string_type_conversion!(String, Box<str>);
impl_annotated_identifier_string_type_conversion!(String, Rc<str>);
