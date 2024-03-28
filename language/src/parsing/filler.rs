use std::rc::Rc;
use std::sync::Arc;

use crate::tokenization::{SubstringPosition, Token, TokenContent};

use super::filler_content::FillerContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Filler<StringType> {
    pub content: FillerContent<StringType>,
    pub position: SubstringPosition,
}

pub(super) type Fillers<StringType> = Vec<Filler<StringType>>;

impl<StringType, TokenStringType> From<Filler<StringType>>
    for Token<TokenStringType>
where
    FillerContent<StringType>: Into<TokenContent<TokenStringType>>,
{
    fn from(value: Filler<StringType>) -> Self {
        Token {
            content: value.content.into(),
            position: value.position,
        }
    }
}

macro_rules! impl_filler_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Filler<$source_string_type>>
            for Filler<$target_string_type>
        {
            fn from(value: Filler<$source_string_type>) -> Self {
                Filler {
                    content: value.content.into(),
                    position: value.position,
                }
            }
        }
    };
}

impl_filler_string_type_conversion!(&str, Arc<str>);
impl_filler_string_type_conversion!(&str, Box<str>);
impl_filler_string_type_conversion!(&str, Rc<str>);
impl_filler_string_type_conversion!(&str, String);
impl_filler_string_type_conversion!(Box<str>, Arc<str>);
impl_filler_string_type_conversion!(Box<str>, Rc<str>);
impl_filler_string_type_conversion!(Box<str>, String);
impl_filler_string_type_conversion!(String, Arc<str>);
impl_filler_string_type_conversion!(String, Box<str>);
impl_filler_string_type_conversion!(String, Rc<str>);
