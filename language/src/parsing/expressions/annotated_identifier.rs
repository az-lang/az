use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{Filler, Fillers};
use crate::parsing::operators::BinaryAnnotationOperator;
use crate::tokenization::{SubstringPosition, Token, Tokenize};

use super::expression::Expression;
use super::identifier::Identifier;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedIdentifier<StringType> {
    pub identifier: Identifier<StringType>,
    pub annotation: Box<Expression<StringType>>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: Fillers<StringType>,
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for AnnotatedIdentifier<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Identifier<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> Vec<Token<TokenStringType>> {
        let mut result = self.identifier.tokenize();
        result.extend(self.operator_fillers.into_iter().map(Into::into));
        result.push(Token {
            content: BinaryAnnotationOperator.into(),
            position: self.operator_position,
        });
        result.extend(self.annotation.tokenize());
        result
    }
}

macro_rules! impl_annotated_identifier_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<AnnotatedIdentifier<$source_string_type>>
            for AnnotatedIdentifier<$target_string_type>
        {
            fn from(value: AnnotatedIdentifier<$source_string_type>) -> Self {
                AnnotatedIdentifier {
                    identifier: value.identifier.into(),
                    annotation: Box::new((*value.annotation).into()),
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

impl_annotated_identifier_string_type_conversion!(&str, Arc<str>);
impl_annotated_identifier_string_type_conversion!(&str, Box<str>);
impl_annotated_identifier_string_type_conversion!(&str, Rc<str>);
impl_annotated_identifier_string_type_conversion!(&str, String);
impl_annotated_identifier_string_type_conversion!(Box<str>, Arc<str>);
impl_annotated_identifier_string_type_conversion!(Box<str>, Rc<str>);
impl_annotated_identifier_string_type_conversion!(Box<str>, String);
impl_annotated_identifier_string_type_conversion!(String, Arc<str>);
impl_annotated_identifier_string_type_conversion!(String, Box<str>);
impl_annotated_identifier_string_type_conversion!(String, Rc<str>);
