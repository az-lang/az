use pyo3::PyErr;

use az::parsing::ParsingError;

use crate::types::{OwnedString, TokenOwnedString};

use super::py_mismatched_open_brace::PyMismatchedOpenBrace;
use super::py_mismatched_open_parenthesis::PyMismatchedOpenParenthesis;
use super::py_missing_semicolon::PyMissingSemicolon;
use super::py_out_of_tokens::PyOutOfTokens;
use super::py_unexpected_expression::PyUnexpectedExpression;
use super::py_unexpected_token::PyUnexpectedToken;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedParsingErrorWrapper(
    ParsingError<OwnedString, OwnedString>,
);

impl From<OwnedParsingErrorWrapper> for PyErr {
    fn from(value: OwnedParsingErrorWrapper) -> Self {
        match value.0 {
            ParsingError::MismatchedOpenBrace(value) => {
                PyMismatchedOpenBrace::from(value).into()
            }
            ParsingError::MismatchedOpenParenthesis(value) => {
                PyMismatchedOpenParenthesis::from(value).into()
            }
            ParsingError::MissingSemicolon(value) => {
                PyMissingSemicolon::from(value).into()
            }
            ParsingError::OutOfTokens(value) => {
                PyOutOfTokens::from(value).into()
            }
            ParsingError::UnexpectedExpression(value) => {
                PyUnexpectedExpression::from(value).into()
            }
            ParsingError::UnexpectedToken(value) => {
                PyUnexpectedToken::from(value).into()
            }
        }
    }
}

impl From<ParsingError<OwnedString, TokenOwnedString>>
    for OwnedParsingErrorWrapper
{
    fn from(value: ParsingError<OwnedString, TokenOwnedString>) -> Self {
        Self(value)
    }
}
