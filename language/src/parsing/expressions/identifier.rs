use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, FillerVec,
    FillerVecContentsValidationError, FillerVecPositionsValidationError,
    NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    validate_identifier_string, ByteSize, CharacterPosition,
    IdentifierStringValidationError, PositionsValidationError,
    SubstringPosition, Token, TokenCollection, TokenContent, Tokenize,
    Utf8Size,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier<StringType> {
    pub string: StringType,
    pub position: SubstringPosition,
    pub fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Identifier<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl(self.position.start)
    }

    pub(super) fn validate_contents_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<IdentifierContentsValidationError>> {
        let mut errors = vec![];
        if let Err(filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.fillers)
        {
            errors.extend(filler_errors.into_iter().map(|error| {
                IdentifierContentsValidationError(
                    IdentifierContentsValidationErrorKind::FillerVec(error),
                )
            }));
        }
        if let Err(string_errors) = validate_identifier_string(
            &self.string,
            expected_start_character_position,
        ) {
            errors.extend(string_errors.into_iter().map(|error| {
                IdentifierContentsValidationError(
                    IdentifierContentsValidationErrorKind::String(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Identifier<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<IdentifierPositionsValidationError> {
        validate_filler_vec_positions(
            &self.fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            IdentifierPositionsValidationError(
                IdentifierPositionsValidationErrorKind::FillerVec(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &self.string,
                &self.position,
                expected_start_character_position,
            )
            .map_error(|error| {
                IdentifierPositionsValidationError(
                    IdentifierPositionsValidationErrorKind::String(error),
                )
            })
        })
    }
}

impl<StringType> Identifier<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.position.start)
    }
}

#[derive(Debug)]
pub(super) struct IdentifierContentsValidationError(
    IdentifierContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct IdentifierPositionsValidationError(
    IdentifierPositionsValidationErrorKind,
);

#[derive(Debug)]
enum IdentifierContentsValidationErrorKind {
    FillerVec(FillerVecContentsValidationError),
    String(IdentifierStringValidationError),
}

#[derive(Debug)]
enum IdentifierPositionsValidationErrorKind {
    FillerVec(FillerVecPositionsValidationError),
    String(PositionsValidationError),
}

impl std::fmt::Display for IdentifierContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            IdentifierContentsValidationErrorKind::FillerVec(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            IdentifierContentsValidationErrorKind::String(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for IdentifierPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            IdentifierPositionsValidationErrorKind::FillerVec(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            IdentifierPositionsValidationErrorKind::String(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for IdentifierContentsValidationError {}
impl std::error::Error for IdentifierPositionsValidationError {}

impl<StringType: AsRef<str> + Into<TokenStringType>, TokenStringType>
    Tokenize<TokenStringType> for Identifier<StringType>
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.fillers.into_iter().map(Into::into).collect::<Vec<_>>();
        tokens.push(Token {
            content: TokenContent::Identifier(self.string.into()),
            position: self.position,
        });
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_identifier_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Identifier<$source_string_type>>
            for Identifier<$target_string_type>
        {
            fn from(value: Identifier<$source_string_type>) -> Self {
                Identifier {
                    string: value.string.into(),
                    position: value.position,
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

impl_identifier_string_type_conversion!(&str, Arc<str>);
impl_identifier_string_type_conversion!(&str, Box<str>);
impl_identifier_string_type_conversion!(&str, Rc<str>);
impl_identifier_string_type_conversion!(&str, String);
impl_identifier_string_type_conversion!(Box<str>, Arc<str>);
impl_identifier_string_type_conversion!(Box<str>, Rc<str>);
impl_identifier_string_type_conversion!(Box<str>, String);
impl_identifier_string_type_conversion!(String, Arc<str>);
impl_identifier_string_type_conversion!(String, Box<str>);
impl_identifier_string_type_conversion!(String, Rc<str>);
