use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::operators::MemberAccessOperator;
use crate::tokenization::{SubstringPosition, Token, Tokenize};

use super::expression::Expression;
use super::identifier::Identifier;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccess<StringType> {
    pub object: Box<Expression<StringType>>,
    pub member: Identifier<StringType>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for MemberAccess<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
    Identifier<StringType>: Tokenize<TokenStringType>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self.object.tokenize();
        result.extend(self.operator_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: MemberAccessOperator.into(),
            position: self.operator_position,
        });
        result.append(&mut self.member.tokenize());
        result
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
