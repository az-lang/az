use std::error::Error;
use std::fmt::Display;

use wasm_bindgen::JsValue;

use az::parsing::ParsingError;

use crate::traits::TryToJsString;
use crate::types::{OwnedString, TokenOwnedString};

#[derive(Debug)]
pub(crate) struct OwnedParsingErrorWrapper(
    ParsingError<OwnedString, TokenOwnedString>,
);

impl Display for OwnedParsingErrorWrapper {
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

impl Error for OwnedParsingErrorWrapper {}

impl From<ParsingError<OwnedString, TokenOwnedString>>
    for OwnedParsingErrorWrapper
{
    fn from(value: ParsingError<OwnedString, TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl TryToJsString for OwnedParsingErrorWrapper {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for ParsingError<OwnedString, TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            ParsingError::MismatchedOpenBrace(value) => {
                value.try_to_js_string()
            }
            ParsingError::MismatchedOpenParenthesis(value) => {
                value.try_to_js_string()
            }
            ParsingError::MissingSemicolon(value) => value.try_to_js_string(),
            ParsingError::OutOfTokens(value) => value.try_to_js_string(),
            ParsingError::UnexpectedExpression(value) => {
                value.try_to_js_string()
            }
            ParsingError::UnexpectedToken(value) => value.try_to_js_string(),
        }
    }
}
