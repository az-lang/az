use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;
use std::sync::Arc;

use crate::tokenization::constants::NEWLINE;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FillerContent<StringType> {
    CommentBlock(Vec<StringType>),
    CommentLine(StringType),
    Newline,
    Whitespace(StringType),
}

impl<StringType: Into<TokenStringType>, TokenStringType>
    From<FillerContent<StringType>> for TokenContent<TokenStringType>
{
    fn from(value: FillerContent<StringType>) -> Self {
        match value {
            FillerContent::CommentBlock(lines) => TokenContent::CommentBlock(
                lines.into_iter().map(Into::into).collect(),
            ),
            FillerContent::CommentLine(line) => {
                TokenContent::CommentLine(line.into())
            }
            FillerContent::Newline => TokenContent::Newline,
            FillerContent::Whitespace(string) => {
                TokenContent::Whitespace(string.into())
            }
        }
    }
}

impl<StringType: AsRef<str>> Display for FillerContent<StringType> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillerContent::CommentBlock(lines) => {
                for line in lines {
                    formatter.write_str(line.as_ref())?;
                }
                Ok(())
            }
            FillerContent::CommentLine(line) => {
                formatter.write_str(line.as_ref())
            }
            FillerContent::Newline => formatter.write_char(NEWLINE),
            FillerContent::Whitespace(string) => {
                formatter.write_str(string.as_ref())
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
                    FillerContent::CommentBlock(strings) => {
                        Self::CommentBlock(
                            strings.into_iter().map(Into::into).collect(),
                        )
                    }
                    FillerContent::CommentLine(string) => {
                        Self::CommentLine(string.into())
                    }
                    FillerContent::Newline => Self::Newline,
                    FillerContent::Whitespace(string) => {
                        Self::Whitespace(string.into())
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
