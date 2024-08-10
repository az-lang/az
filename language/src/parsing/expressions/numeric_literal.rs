use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    validate_floating_point_literal_string, validate_integer_literal_string,
    ByteSize, CharacterPosition, NumericLiteralType,
    NumericLiteralValueValidationError, PositionsValidationError,
    SubstringPosition, Token, TokenCollection, TokenContent, Tokenize,
    Utf8Size,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteral<StringType> {
    pub value: StringType,
    pub type_: NumericLiteralType,
    pub position: SubstringPosition,
    pub fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> NumericLiteral<StringType> {
    pub fn validate_contents(&self) -> Result<(), Vec<impl std::error::Error>>
    where
        StringType: AsRef<str>,
    {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<NumericLiteralContentsValidationError>> {
        let mut errors = vec![];
        if let Err(value_errors) = match self.type_ {
            NumericLiteralType::F32 | NumericLiteralType::F64 => {
                validate_floating_point_literal_string(
                    &self.value,
                    self.position.start,
                )
            }
            NumericLiteralType::I8
            | NumericLiteralType::I16
            | NumericLiteralType::I32
            | NumericLiteralType::I64
            | NumericLiteralType::ISize
            | NumericLiteralType::U8
            | NumericLiteralType::U16
            | NumericLiteralType::U32
            | NumericLiteralType::U64
            | NumericLiteralType::USize => validate_integer_literal_string(
                &self.value,
                self.position.start,
            ),
        } {
            errors.extend(value_errors.into_iter().map(|error| {
                NumericLiteralContentsValidationError(
                    NumericLiteralContentsValidationErrorKind::Value(error),
                )
            }));
        };
        if let Err(filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.fillers)
        {
            errors.extend(filler_errors.into_iter().map(|error| {
                NumericLiteralContentsValidationError(
                    NumericLiteralContentsValidationErrorKind::Filler(error),
                )
            }))
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> NumericLiteral<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<NumericLiteralPositionsValidationError> {
        validate_filler_vec_positions(
            &self.fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            NumericLiteralPositionsValidationError(
                NumericLiteralPositionsValidationErrorKind::Filler(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<&str>::NumericLiteral {
                    value: self.value.as_ref(),
                    type_: self.type_,
                }
                .to_string(),
                &self.position,
                expected_start_character_position,
            )
            .map_error(|error| {
                NumericLiteralPositionsValidationError(
                    NumericLiteralPositionsValidationErrorKind::Value(error),
                )
            })
        })
    }
}

impl<StringType> NumericLiteral<StringType> {
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
pub(super) struct NumericLiteralContentsValidationError(
    NumericLiteralContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct NumericLiteralPositionsValidationError(
    NumericLiteralPositionsValidationErrorKind,
);

#[derive(Debug)]
enum NumericLiteralContentsValidationErrorKind {
    Filler(FillerVecContentsValidationError),
    Value(NumericLiteralValueValidationError),
}

#[derive(Debug)]
enum NumericLiteralPositionsValidationErrorKind {
    Filler(FillerVecPositionsValidationError),
    Value(PositionsValidationError),
}

impl std::fmt::Display for NumericLiteralContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            NumericLiteralContentsValidationErrorKind::Value(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            NumericLiteralContentsValidationErrorKind::Filler(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for NumericLiteralPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            NumericLiteralPositionsValidationErrorKind::Value(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            NumericLiteralPositionsValidationErrorKind::Filler(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for NumericLiteralContentsValidationError {}

impl std::error::Error for NumericLiteralPositionsValidationError {}

impl<StringType: Into<TokenStringType>, TokenStringType>
    Tokenize<TokenStringType> for NumericLiteral<StringType>
where
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut result =
            self.fillers.into_iter().map(Into::into).collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::NumericLiteral {
                value: self.value.into(),
                type_: self.type_,
            },
            position: self.position,
        });
        TokenCollection::new(result)
    }
}

macro_rules! impl_numeric_literal_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<NumericLiteral<$source_string_type>>
            for NumericLiteral<$target_string_type>
        {
            fn from(value: NumericLiteral<$source_string_type>) -> Self {
                NumericLiteral {
                    value: value.value.into(),
                    type_: value.type_,
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

impl_numeric_literal_string_type_conversion!(&str, Arc<str>);
impl_numeric_literal_string_type_conversion!(&str, Box<str>);
impl_numeric_literal_string_type_conversion!(&str, Rc<str>);
impl_numeric_literal_string_type_conversion!(&str, String);
impl_numeric_literal_string_type_conversion!(Box<str>, Arc<str>);
impl_numeric_literal_string_type_conversion!(Box<str>, Rc<str>);
impl_numeric_literal_string_type_conversion!(Box<str>, String);
impl_numeric_literal_string_type_conversion!(String, Arc<str>);
impl_numeric_literal_string_type_conversion!(String, Box<str>);
impl_numeric_literal_string_type_conversion!(String, Rc<str>);
