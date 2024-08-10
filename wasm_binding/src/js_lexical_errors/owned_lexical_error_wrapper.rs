use std::error::Error;

use wasm_bindgen::JsValue;

use az::tokenization::LexicalError;

use crate::traits::TryToJsString;
use crate::types::TokenOwnedString;

#[derive(Debug)]
pub(crate) struct OwnedLexicalErrorWrapper(LexicalError<TokenOwnedString>);

impl std::fmt::Display for OwnedLexicalErrorWrapper {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self.try_to_js_string() {
            Ok(value) => write!(formatter, "{}", value),
            Err(error) => write!(
                formatter,
                "Error while serializing {:?}: {:?}.",
                self, error
            ),
        }
    }
}

impl Error for OwnedLexicalErrorWrapper {}

impl From<OwnedLexicalErrorWrapper> for LexicalError<TokenOwnedString> {
    fn from(value: OwnedLexicalErrorWrapper) -> Self {
        value.0
    }
}

impl From<LexicalError<TokenOwnedString>> for OwnedLexicalErrorWrapper {
    fn from(value: LexicalError<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl TryToJsString for LexicalError<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            LexicalError::CommentBlockIncomplete(value) => {
                value.try_to_js_string()
            }
            LexicalError::IdentifierIncomplete(value) => {
                value.try_to_js_string()
            }
            LexicalError::IdentifierUnexpectedCharacter(value) => {
                value.try_to_js_string()
            }
            LexicalError::NumericLiteralTypeSuffixIncomplete(value) => {
                value.try_to_js_string()
            }
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                value,
            ) => value.try_to_js_string(),
            LexicalError::NumericLiteralTypeSuffixUnknown(value) => {
                value.try_to_js_string()
            }
            LexicalError::NumericLiteralValueIncomplete(value) => {
                value.try_to_js_string()
            }
            LexicalError::NumericLiteralValueTypeSuffixConflict(value) => {
                value.try_to_js_string()
            }
            LexicalError::NumericLiteralValueUnexpectedCharacter(value) => {
                value.try_to_js_string()
            }
            LexicalError::UnexpectedCharacter(value) => {
                value.try_to_js_string()
            }
        }
    }
}

impl TryToJsString for OwnedLexicalErrorWrapper {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}
