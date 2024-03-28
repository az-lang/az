use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::tokenization::{
    NumericLiteralType, SubstringPosition, Token, TokenContent, Tokenize,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteral<StringType> {
    pub value: StringType,
    pub type_: NumericLiteralType,
    pub position: SubstringPosition,
    pub fillers: Fillers<StringType>,
}

impl<StringType: Into<TokenStringType>, TokenStringType>
    Tokenize<TokenStringType> for NumericLiteral<StringType>
where
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result =
            self.fillers.into_iter().map(Into::into).collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::NumericLiteral {
                value: self.value.into(),
                type_: self.type_,
            },
            position: self.position,
        });
        result
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
