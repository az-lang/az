use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;
use std::sync::Arc;

use crate::tokenization::constants::NEWLINE;
use crate::tokenization::{
    validate_comment_block_lines, validate_comment_line_string,
    validate_whitespace_string, ByteCount, ByteSize, CharacterPosition,
    CommentBlockLinesValidationError, CommentLineStringValidationError,
    TokenContent, Utf8Count, Utf8Size, WhitespaceStringValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FillerContent<StringType> {
    CommentBlock(StringType),
    CommentLine(StringType),
    Newline,
    Whitespace(StringType),
}

impl<StringType: AsRef<str>> FillerContent<StringType> {
    pub fn validate(&self) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_impl(CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        })
    }

    pub(super) fn validate_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<FillerContentValidationError>> {
        match self {
            FillerContent::Newline => Ok(()),
            FillerContent::CommentBlock(value) => {
                validate_comment_block_lines(
                    value
                        .as_ref()
                        .split_inclusive(NEWLINE)
                        .collect::<Vec<_>>()
                        .as_slice(),
                    expected_start_character_position,
                )
                .map_err(|errors| {
                    errors
                        .into_iter()
                        .map(FillerContentValidationError::CommentBlock)
                        .collect()
                })
            }
            FillerContent::CommentLine(value) => {
                validate_comment_line_string(value).map_err(|errors| {
                    errors
                        .into_iter()
                        .map(FillerContentValidationError::CommentLine)
                        .collect()
                })
            }
            FillerContent::Whitespace(value) => validate_whitespace_string(
                value,
                expected_start_character_position,
            )
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(FillerContentValidationError::Whitespace)
                    .collect()
            }),
        }
    }
}

impl<StringType: ByteSize + Utf8Size> FillerContent<StringType> {
    pub(crate) fn to_character_count(&self) -> CharacterPosition {
        match self {
            Self::CommentBlock(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
            Self::CommentLine(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
            Self::Newline => CharacterPosition {
                byte: NEWLINE.byte_size(),
                utf_8: NEWLINE.utf_8_size(),
            },
            Self::Whitespace(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
        }
    }
}

#[derive(Debug)]
pub(super) enum FillerContentValidationError {
    CommentBlock(CommentBlockLinesValidationError),
    CommentLine(CommentLineStringValidationError),
    Whitespace(WhitespaceStringValidationError),
}

impl Display for FillerContentValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillerContentValidationError::CommentBlock(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FillerContentValidationError::CommentLine(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FillerContentValidationError::Whitespace(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for FillerContentValidationError {}

impl<StringType: Into<TokenStringType>, TokenStringType>
    From<FillerContent<StringType>> for TokenContent<TokenStringType>
{
    fn from(value: FillerContent<StringType>) -> Self {
        match value {
            FillerContent::CommentBlock(value) => {
                TokenContent::CommentBlock(value.into())
            }
            FillerContent::CommentLine(value) => {
                TokenContent::CommentLine(value.into())
            }
            FillerContent::Newline => TokenContent::Newline,
            FillerContent::Whitespace(value) => {
                TokenContent::Whitespace(value.into())
            }
        }
    }
}

impl<StringType: AsRef<str>> Display for FillerContent<StringType> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillerContent::CommentBlock(value) => {
                formatter.write_str(value.as_ref())
            }
            FillerContent::CommentLine(value) => {
                formatter.write_str(value.as_ref())
            }
            FillerContent::Newline => formatter.write_char(NEWLINE),
            FillerContent::Whitespace(value) => {
                formatter.write_str(value.as_ref())
            }
        }
    }
}

macro_rules! impl_filler_content_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<FillerContent<$source_string_type>>
            for FillerContent<$target_string_type>
        {
            fn from(value: FillerContent<$source_string_type>) -> Self {
                match value {
                    FillerContent::CommentBlock(value) => {
                        Self::CommentBlock(value.into())
                    }
                    FillerContent::CommentLine(value) => {
                        Self::CommentLine(value.into())
                    }
                    FillerContent::Newline => Self::Newline,
                    FillerContent::Whitespace(value) => {
                        Self::Whitespace(value.into())
                    }
                }
            }
        }
    };
}

impl_filler_content_string_type_conversion!(&str, Arc<str>);
impl_filler_content_string_type_conversion!(&str, Box<str>);
impl_filler_content_string_type_conversion!(&str, Rc<str>);
impl_filler_content_string_type_conversion!(&str, String);
impl_filler_content_string_type_conversion!(Box<str>, Arc<str>);
impl_filler_content_string_type_conversion!(Box<str>, Rc<str>);
impl_filler_content_string_type_conversion!(Box<str>, String);
impl_filler_content_string_type_conversion!(String, Arc<str>);
impl_filler_content_string_type_conversion!(String, Box<str>);
impl_filler_content_string_type_conversion!(String, Rc<str>);
