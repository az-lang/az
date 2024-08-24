use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::MemberAccessOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::binary_operand::{
    validate_maybe_nested_left_binary_operand,
    BinaryLeftOperandValidationError, BinaryOperandMetadata,
    LEFT_BINARY_OPERAND_POSITION,
};
use super::expression::{Expression, ExpressionPositionsValidationError};
use super::identifier::{
    Identifier, IdentifierContentsValidationError,
    IdentifierPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccess<StringType> {
    pub object: Box<Expression<StringType>>,
    pub member: Identifier<StringType>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> MemberAccess<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<MemberAccessContentsValidationError>> {
        let mut errors = vec![];
        errors.extend(
            validate_maybe_nested_left_binary_operand(
                self.object.as_ref(),
                MemberAccessOperator,
            )
            .into_iter()
            .map(|error| {
                MemberAccessContentsValidationError(
                    MemberAccessContentsValidationErrorKind::Object(error),
                )
            }),
        );
        if let Err(member_errors) = self
            .member
            .validate_contents_impl(self.member.position.start)
        {
            errors.extend(member_errors.into_iter().map(|error| {
                MemberAccessContentsValidationError(
                    MemberAccessContentsValidationErrorKind::Member(error),
                )
            }));
        }
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(operator_filler_errors.into_iter().map(|error| {
                MemberAccessContentsValidationError(
                    MemberAccessContentsValidationErrorKind::OperatorFiller(
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> MemberAccess<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<MemberAccessPositionsValidationError> {
        self
            .object
            .validate_positions_impl(expected_start_character_position)
            .map_error(
                |error| {
                    MemberAccessPositionsValidationError(
                        MemberAccessPositionsValidationErrorKind::Object(
                            Box::new(error),
                        ),
                    )
                },
            )
            .merge_with(|expected_start_character_position| {
                    validate_filler_vec_positions(
                        &self.operator_fillers,
                        expected_start_character_position,
                    )
                        .map_error(|error| { MemberAccessPositionsValidationError(
                            MemberAccessPositionsValidationErrorKind::OperatorFiller(
                                error,
                            ),
                        )})
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::from(MemberAccessOperator).to_string(),
                    &self.operator_position,
                    expected_start_character_position
                ).map_error(
                    |error| {
                        MemberAccessPositionsValidationError(
                            MemberAccessPositionsValidationErrorKind::Operator(
                                error,
                            ),
                        )
                    }
                )
            })
            .merge_with(|expected_start_character_position| {
                self.member.validate_positions_impl(
                    expected_start_character_position,
                ).map_error(|error| { MemberAccessPositionsValidationError(
                    MemberAccessPositionsValidationErrorKind::Member(error),
                )})
            })
    }
}

impl<StringType> MemberAccess<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.object.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct MemberAccessContentsValidationError(
    MemberAccessContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct MemberAccessPositionsValidationError(
    MemberAccessPositionsValidationErrorKind,
);

#[derive(Debug)]
pub(super) enum MemberAccessContentsValidationErrorKind {
    Member(IdentifierContentsValidationError),
    Object(BinaryLeftOperandValidationError<MemberAccessOperator>),
    OperatorFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
pub(super) enum MemberAccessPositionsValidationErrorKind {
    Member(IdentifierPositionsValidationError),
    Object(Box<ExpressionPositionsValidationError>),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
}

impl BinaryOperandMetadata<LEFT_BINARY_OPERAND_POSITION>
    for MemberAccessOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "object"
    }
}

impl std::fmt::Display for MemberAccessContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            MemberAccessContentsValidationErrorKind::Member(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            MemberAccessContentsValidationErrorKind::Object(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            MemberAccessContentsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for MemberAccessPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            MemberAccessPositionsValidationErrorKind::Member(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            MemberAccessPositionsValidationErrorKind::Object(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            MemberAccessPositionsValidationErrorKind::Operator(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            MemberAccessPositionsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for MemberAccessContentsValidationError {}

impl std::error::Error for MemberAccessPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for MemberAccess<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
    Identifier<StringType>: Tokenize<TokenStringType>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.object.tokenize().into_iter().collect::<Vec<_>>();
        tokens.extend(self.operator_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: MemberAccessOperator.into(),
            position: self.operator_position,
        });
        tokens.extend(self.member.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_member_access_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<MemberAccess<$source_string_type>>
            for MemberAccess<$target_string_type>
        {
            fn from(value: MemberAccess<$source_string_type>) -> Self {
                MemberAccess {
                    object: Box::new((*value.object).into()),
                    member: value.member.into(),
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

impl_member_access_string_type_conversion!(&str, Arc<str>);
impl_member_access_string_type_conversion!(&str, Box<str>);
impl_member_access_string_type_conversion!(&str, Rc<str>);
impl_member_access_string_type_conversion!(&str, String);
impl_member_access_string_type_conversion!(Box<str>, Arc<str>);
impl_member_access_string_type_conversion!(Box<str>, Rc<str>);
impl_member_access_string_type_conversion!(Box<str>, String);
impl_member_access_string_type_conversion!(String, Arc<str>);
impl_member_access_string_type_conversion!(String, Box<str>);
impl_member_access_string_type_conversion!(String, Rc<str>);
