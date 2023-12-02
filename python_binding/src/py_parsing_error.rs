use crate::py_expression::OwnedExpression;
use az::parsing::ParsingError;

use super::macros::define_derived_exception;
use super::py_positioned_token::PyPositionedToken;

#[pyo3::pyclass(module = "az.parsing", name = "ParsingError",
extends = pyo3::exceptions::PyException, frozen, subclass)]
pub(super) struct PyParsingError {}

impl PyParsingError {
    fn new() -> pyo3::PyClassInitializer<Self> {
        pyo3::PyClassInitializer::from(Self {})
    }
}

define_derived_exception!(
    PyMismatchedOpenBrace, "az.parsing", "MismatchedOpenBrace", PyParsingError,
    token: PyPositionedToken,
);

define_derived_exception!(
    PyMismatchedOpenParentheses, "az.parsing", "MismatchedOpenParentheses", PyParsingError,
    token: PyPositionedToken,
);

define_derived_exception!(
    PyMissingSemicolon, "az.parsing", "MissingSemicolon", PyParsingError, token: PyPositionedToken,
);

define_derived_exception!(
    PyOutOfTokens,
    "az.parsing",
    "OutOfTokens",
    PyParsingError
);

define_derived_exception!(
    PyUnexpectedExpression, "az.parsing", "UnexpectedExpression", PyParsingError, expression: OwnedExpression,
);

define_derived_exception!(
    PyUnexpectedToken, "az.parsing", "UnexpectedToken", PyParsingError, token: PyPositionedToken,
);

pub(super) enum OwnedParsingError {
    MismatchedOpenBrace(PyMismatchedOpenBrace),
    MismatchedOpenParentheses(PyMismatchedOpenParentheses),
    MissingSemicolon(PyMissingSemicolon),
    OutOfTokens(PyOutOfTokens),
    UnexpectedExpression(PyUnexpectedExpression),
    UnexpectedToken(PyUnexpectedToken),
}

impl From<OwnedParsingError> for pyo3::PyErr {
    fn from(value: OwnedParsingError) -> Self {
        match value {
            OwnedParsingError::MismatchedOpenBrace(wrapped) => wrapped.into(),
            OwnedParsingError::MismatchedOpenParentheses(wrapped) => {
                wrapped.into()
            }
            OwnedParsingError::MissingSemicolon(wrapped) => wrapped.into(),
            OwnedParsingError::OutOfTokens(wrapped) => wrapped.into(),
            OwnedParsingError::UnexpectedExpression(wrapped) => wrapped.into(),
            OwnedParsingError::UnexpectedToken(wrapped) => wrapped.into(),
        }
    }
}

impl From<ParsingError<'_>> for OwnedParsingError {
    fn from(value: ParsingError<'_>) -> Self {
        match value {
            ParsingError::MismatchedOpenBrace(token) => {
                OwnedParsingError::MismatchedOpenBrace(PyMismatchedOpenBrace {
                    token: token.into(),
                })
            }
            ParsingError::MismatchedOpenParentheses(token) => {
                OwnedParsingError::MismatchedOpenParentheses(
                    PyMismatchedOpenParentheses {
                        token: token.into(),
                    },
                )
            }
            ParsingError::MissingSemicolon(token) => {
                OwnedParsingError::MissingSemicolon(PyMissingSemicolon {
                    token: token.into(),
                })
            }
            ParsingError::OutOfTokens => {
                OwnedParsingError::OutOfTokens(PyOutOfTokens {})
            }
            ParsingError::UnexpectedExpression(expression) => {
                OwnedParsingError::UnexpectedExpression(
                    PyUnexpectedExpression {
                        expression: OwnedExpression::from(expression),
                    },
                )
            }
            ParsingError::UnexpectedToken(token) => {
                OwnedParsingError::UnexpectedToken(PyUnexpectedToken {
                    token: token.into(),
                })
            }
        }
    }
}
