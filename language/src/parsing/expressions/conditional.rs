use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::keywords::{
    CONDITIONAL_ALTERNATIVE_OPENER, CONDITIONAL_ANTECEDENT_OPENER,
};
use crate::tokenization::{SubstringPosition, Token, TokenContent, Tokenize};

use super::block::Block;
use super::expression::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Conditional<StringType> {
    pub antecedent: Box<Expression<StringType>>,
    pub consequent: Block<StringType>,
    pub alternative: Option<Box<Expression<StringType>>>,
    pub opener_position: SubstringPosition,
    pub alternative_opener_position: Option<SubstringPosition>,
    pub opener_fillers: Fillers<StringType>,
    pub alternative_opener_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for Conditional<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self
            .opener_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        result.push(Token {
            content: TokenContent::Identifier(TokenStringType::from(
                CONDITIONAL_ANTECEDENT_OPENER,
            )),
            position: self.opener_position,
        });
        result.append(&mut self.antecedent.tokenize());
        result.append(&mut self.consequent.tokenize());
        result.extend(
            self.alternative_opener_fillers.into_iter().map(Into::into),
        );
        if let Some(alternative_opener_position) =
            self.alternative_opener_position
        {
            result.push(Token {
                content: TokenContent::Identifier(TokenStringType::from(
                    CONDITIONAL_ALTERNATIVE_OPENER,
                )),
                position: alternative_opener_position,
            });
        }
        if let Some(alternative) = self.alternative {
            result.append(&mut alternative.tokenize());
        }
        result
    }
}

macro_rules! impl_conditional_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Conditional<$source_string_type>>
            for Conditional<$target_string_type>
        {
            fn from(value: Conditional<$source_string_type>) -> Self {
                Conditional {
                    antecedent: Box::new((*value.antecedent).into()),
                    consequent: value.consequent.into(),
                    alternative: value.alternative.map(|alternative_value| {
                        Box::new((*alternative_value).into())
                    }),
                    opener_position: value.opener_position,
                    alternative_opener_position: value
                        .alternative_opener_position,
                    opener_fillers: value
                        .opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    alternative_opener_fillers: value
                        .alternative_opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_conditional_string_type_conversion!(&str, Arc<str>);
impl_conditional_string_type_conversion!(&str, Box<str>);
impl_conditional_string_type_conversion!(&str, Rc<str>);
impl_conditional_string_type_conversion!(&str, String);
impl_conditional_string_type_conversion!(Box<str>, Arc<str>);
impl_conditional_string_type_conversion!(Box<str>, Rc<str>);
impl_conditional_string_type_conversion!(Box<str>, String);
impl_conditional_string_type_conversion!(String, Arc<str>);
impl_conditional_string_type_conversion!(String, Box<str>);
impl_conditional_string_type_conversion!(String, Rc<str>);
