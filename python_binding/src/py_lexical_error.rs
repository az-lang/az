use az::tokenization;

use super::macros::define_derived_exception;
use super::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use super::py_substring_position::PySubstringPosition;

#[pyo3::pyclass(module = "az.tokenization", name = "LexicalError",
extends = pyo3::exceptions::PyException, frozen, subclass)]
pub(super) struct PyLexicalError {}

impl PyLexicalError {
    fn new() -> pyo3::PyClassInitializer<Self> {
        pyo3::PyClassInitializer::from(Self {})
    }
}

define_derived_exception!(
    PyCommentBlockIncomplete,
    "az.tokenization",
    "CommentBlockIncomplete",
    PyLexicalError,
    position: PySubstringPosition,
    strings: Vec<String>,
);

define_derived_exception!(
    PyIdentifierIncomplete,
    "az.tokenization",
    "IdentifierIncomplete",
    PyLexicalError,
    position: PySubstringPosition,
    string: String,
);

define_derived_exception!(
    PyIdentifierUnexpectedCharacter,
    "az.tokenization",
    "IdentifierUnexpectedCharacter",
    PyLexicalError,
    character: char,
    expected: String,
    position: PySubstringPosition,
    string: String,
);

define_derived_exception!(
    PyNumericLiteralValueUnexpectedCharacter,
    "az.tokenization",
    "NumericLiteralValueUnexpectedCharacter",
    PyLexicalError,
    character: char,
    expected: String,
    kind: PyNumericLiteralValueKind,
    position: PySubstringPosition,
    string: String,
);

define_derived_exception!(
    PyNumericLiteralValueTypeSuffixConflict,
    "az.tokenization",
    "NumericLiteralValueTypeSuffixConflict",
    PyLexicalError,
    position: PySubstringPosition,
    string: String,
    type_suffix: String,
    value: String,
    value_kind: PyNumericLiteralValueKind,
);

define_derived_exception!(
    PyNumericLiteralTypeSuffixIncomplete,
    "az.tokenization",
    "NumericLiteralTypeSuffixIncomplete",
    PyLexicalError,
    position: PySubstringPosition,
    string: String,
    value: String,
    value_kind: PyNumericLiteralValueKind,
);

define_derived_exception!(
    PyNumericLiteralTypeSuffixUnexpectedCharacter,
    "az.tokenization",
    "NumericLiteralTypeSuffixUnexpectedCharacter",
    PyLexicalError,
    character: char,
    expected: String,
    position: PySubstringPosition,
    string: String,
    value: String,
    value_kind: PyNumericLiteralValueKind,
);

define_derived_exception!(
    PyNumericLiteralTypeSuffixUnknown,
    "az.tokenization",
    "NumericLiteralTypeSuffixUnknown",
    PyLexicalError,
    position: PySubstringPosition,
    string: String,
    type_suffix: String,
    value: String,
    value_kind: PyNumericLiteralValueKind,
);

define_derived_exception!(
    PyNumericLiteralValueIncomplete,
    "az.tokenization",
    "NumericLiteralValueIncomplete",
    PyLexicalError,
    kind: PyNumericLiteralValueKind,
    position: PySubstringPosition,
    string: String,
);

define_derived_exception!(
    PyUnexpectedCharacter,
    "az.tokenization",
    "UnexpectedCharacter",
    PyLexicalError,
    character: char,
    position: PySubstringPosition,
    string: String,
);

pub(super) enum OwnedLexicalError {
    CommentBlockIncomplete(PyCommentBlockIncomplete),
    IdentifierIncomplete(PyIdentifierIncomplete),
    IdentifierUnexpectedCharacter(PyIdentifierUnexpectedCharacter),
    NumericLiteralValueTypeSuffixConflict(
        PyNumericLiteralValueTypeSuffixConflict,
    ),
    NumericLiteralTypeSuffixIncomplete(PyNumericLiteralTypeSuffixIncomplete),
    NumericLiteralTypeSuffixUnexpectedCharacter(
        PyNumericLiteralTypeSuffixUnexpectedCharacter,
    ),
    NumericLiteralTypeSuffixUnknown(PyNumericLiteralTypeSuffixUnknown),
    NumericLiteralValueIncomplete(PyNumericLiteralValueIncomplete),
    NumericLiteralValueUnexpectedCharacter(
        PyNumericLiteralValueUnexpectedCharacter,
    ),
    UnexpectedCharacter(PyUnexpectedCharacter),
}

impl From<OwnedLexicalError> for pyo3::PyErr {
    fn from(value: OwnedLexicalError) -> Self {
        match value {
            OwnedLexicalError::CommentBlockIncomplete(value) => value.into(),
            OwnedLexicalError::IdentifierIncomplete(value) => value.into(),
            OwnedLexicalError::IdentifierUnexpectedCharacter(value) => {
                value.into()
            }
            OwnedLexicalError::NumericLiteralValueTypeSuffixConflict(
                value,
            ) => value.into(),
            OwnedLexicalError::NumericLiteralTypeSuffixIncomplete(value) => {
                value.into()
            }
            OwnedLexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                value,
            ) => value.into(),
            OwnedLexicalError::NumericLiteralTypeSuffixUnknown(value) => {
                value.into()
            }
            OwnedLexicalError::NumericLiteralValueIncomplete(value) => {
                value.into()
            }
            OwnedLexicalError::NumericLiteralValueUnexpectedCharacter(
                value,
            ) => value.into(),
            OwnedLexicalError::UnexpectedCharacter(value) => value.into(),
        }
    }
}

impl From<tokenization::LexicalError<'_>> for OwnedLexicalError {
    fn from(value: tokenization::LexicalError<'_>) -> Self {
        match value {
            tokenization::LexicalError::CommentBlockIncomplete { position, strings } => {
                OwnedLexicalError::CommentBlockIncomplete(PyCommentBlockIncomplete {
                    position: position.into(),
                    strings: strings.iter().map(|string| string.to_string()).collect(),
                })
            }
            tokenization::LexicalError::IdentifierIncomplete { position, string } => {
                OwnedLexicalError::IdentifierIncomplete(PyIdentifierIncomplete {
                    position: position.into(),
                    string: string.to_string(),
                })
            }
            tokenization::LexicalError::IdentifierUnexpectedCharacter {
                character,
                expected,
                position,
                string,
            } => {
                OwnedLexicalError::IdentifierUnexpectedCharacter(PyIdentifierUnexpectedCharacter {
                    character,
                    expected: expected.to_string(),
                    position: position.into(),
                    string: string.to_string(),
                })
            }
            tokenization::LexicalError::NumericLiteralValueTypeSuffixConflict {
                position,
                type_suffix,
                string,
                value,
                value_kind,
            } => OwnedLexicalError::NumericLiteralValueTypeSuffixConflict(
                PyNumericLiteralValueTypeSuffixConflict {
                    position: position.into(),
                    string: string.to_string(),
                    type_suffix: type_suffix.to_string(),
                    value: value.to_string(),
                    value_kind: value_kind.into(),
                },
            ),
            tokenization::LexicalError::NumericLiteralTypeSuffixIncomplete {
                position,
                string,
                value,
                value_kind,
            } => OwnedLexicalError::NumericLiteralTypeSuffixIncomplete(
                PyNumericLiteralTypeSuffixIncomplete {
                    position: position.into(),
                    string: string.to_string(),
                    value: value.to_string(),
                    value_kind: value_kind.into(),
                },
            ),
            tokenization::LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter {
                character,
                expected,
                position,
                string,
                value,
                value_kind,
            } => OwnedLexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                PyNumericLiteralTypeSuffixUnexpectedCharacter {
                    character,
                    expected: expected.to_string(),
                    position: position.into(),
                    string: string.to_string(),
                    value: value.to_string(),
                    value_kind: value_kind.into(),
                },
            ),
            tokenization::LexicalError::NumericLiteralTypeSuffixUnknown {
                position,
                type_suffix,
                string,
                value,
                value_kind,
            } => OwnedLexicalError::NumericLiteralTypeSuffixUnknown(
                PyNumericLiteralTypeSuffixUnknown {
                    position: position.into(),
                    string: string.to_string(),
                    type_suffix: type_suffix.to_string(),
                    value: value.to_string(),
                    value_kind: value_kind.into(),
                },
            ),
            tokenization::LexicalError::NumericLiteralValueIncomplete {
                string,
                position,
                kind,
            } => {
                OwnedLexicalError::NumericLiteralValueIncomplete(PyNumericLiteralValueIncomplete {
                    kind: kind.into(),
                    string: string.to_string(),
                    position: position.into(),
                })
            }
            tokenization::LexicalError::NumericLiteralValueUnexpectedCharacter {
                character,
                expected,
                kind,
                position,
                string,
            } => OwnedLexicalError::NumericLiteralValueUnexpectedCharacter(
                PyNumericLiteralValueUnexpectedCharacter {
                    character,
                    expected: expected.to_string(),
                    kind: kind.into(),
                    position: position.into(),
                    string: string.to_string(),
                },
            ),
            tokenization::LexicalError::UnexpectedCharacter {
                character,
                position,
                string,
            } => OwnedLexicalError::UnexpectedCharacter(PyUnexpectedCharacter {
                character,
                position: position.into(),
                string: string.to_string(),
            }),
        }
    }
}
