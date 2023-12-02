use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::Fillers;
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier<StringType> {
    pub string: StringType,
    pub position: SubstringPosition,
    pub fillers: Fillers<StringType>,
}

impl<StringType: Into<TokenStringType>, TokenStringType>
    Tokenize<TokenStringType> for Identifier<StringType>
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result =
            self.fillers.into_iter().map(Into::into).collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::Identifier(self.string.into()),
            position: self.position,
        });
        result
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
