use pyo3::PyErr;

use az::tokenization::LexicalError;

use crate::types::TokenOwnedStr;

use super::py_comment_block_incomplete::PyCommentBlockIncomplete;
use super::py_identifier_incomplete::PyIdentifierIncomplete;
use super::py_identifier_unexpected_character::PyIdentifierUnexpectedCharacter;
use super::py_numeric_literal_type_suffix_incomplete::PyNumericLiteralTypeSuffixIncomplete;
use super::py_numeric_literal_type_suffix_unexpected_character::PyNumericLiteralTypeSuffixUnexpectedCharacter;
use super::py_numeric_literal_type_suffix_unknown::PyNumericLiteralTypeSuffixUnknown;
use super::py_numeric_literal_value_incomplete::PyNumericLiteralValueIncomplete;
use super::py_numeric_literal_value_type_suffix_conflict::PyNumericLiteralValueTypeSuffixConflict;
use super::py_numeric_literal_value_unexpected_character::PyNumericLiteralValueUnexpectedCharacter;
use super::py_unexpected_character::PyUnexpectedCharacter;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedLexicalErrorWrapper(LexicalError<TokenOwnedStr>);

impl From<OwnedLexicalErrorWrapper> for PyErr {
    fn from(value: OwnedLexicalErrorWrapper) -> Self {
        match value.0 {
            LexicalError::CommentBlockIncomplete(value) => {
                PyCommentBlockIncomplete::from(value).into()
            }
            LexicalError::IdentifierIncomplete(value) => {
                PyIdentifierIncomplete::from(value).into()
            }
            LexicalError::IdentifierUnexpectedCharacter(value) => {
                PyIdentifierUnexpectedCharacter::from(value).into()
            }
            LexicalError::NumericLiteralTypeSuffixIncomplete(value) => {
                PyNumericLiteralTypeSuffixIncomplete::from(value).into()
            }
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                value,
            ) => PyNumericLiteralTypeSuffixUnexpectedCharacter::from(value)
                .into(),
            LexicalError::NumericLiteralTypeSuffixUnknown(value) => {
                PyNumericLiteralTypeSuffixUnknown::from(value).into()
            }
            LexicalError::NumericLiteralValueIncomplete(value) => {
                PyNumericLiteralValueIncomplete::from(value).into()
            }
            LexicalError::NumericLiteralValueTypeSuffixConflict(value) => {
                PyNumericLiteralValueTypeSuffixConflict::from(value).into()
            }
            LexicalError::NumericLiteralValueUnexpectedCharacter(value) => {
                PyNumericLiteralValueUnexpectedCharacter::from(value).into()
            }
            LexicalError::UnexpectedCharacter(value) => {
                PyUnexpectedCharacter::from(value).into()
            }
        }
    }
}

impl From<LexicalError<TokenOwnedStr>> for OwnedLexicalErrorWrapper {
    fn from(value: LexicalError<TokenOwnedStr>) -> Self {
        Self(value)
    }
}
