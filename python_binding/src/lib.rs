use std::ops::Deref;

use pyo3::types::PyModule;
use pyo3::{
    pyfunction, pymodule, wrap_pyfunction, IntoPy, PyObject, PyRef, PyResult,
    Python,
};

use az::parsing::TryParse;
use az::tokenization::TryTokenize;

use self::py_binary_arithmetic_operator::PyBinaryArithmeticOperator;
use self::py_character_position::PyCharacterPosition;
use self::py_comparison_operator::PyComparisonOperator;
use self::py_expression::{
    PyAnnotatedIdentifier, PyAssignment, PyBinaryOperation, PyBlock, PyCall,
    PyComparison, PyConditional, PyExpression, PyIdentifier, PyMemberAccess,
    PyNumericLiteral, PyTuple, PyUnaryOperation,
};
use self::py_lexical_error::{
    OwnedLexicalError, PyCommentBlockIncomplete, PyIdentifierIncomplete,
    PyIdentifierUnexpectedCharacter, PyLexicalError,
    PyNumericLiteralTypeSuffixIncomplete,
    PyNumericLiteralTypeSuffixUnexpectedCharacter,
    PyNumericLiteralTypeSuffixUnknown, PyNumericLiteralValueIncomplete,
    PyNumericLiteralValueTypeSuffixConflict,
    PyNumericLiteralValueUnexpectedCharacter, PyUnexpectedCharacter,
};
use self::py_numeric_literal_type::PyNumericLiteralType;
use self::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use self::py_parsing_error::{
    OwnedParsingError, PyMismatchedOpenParentheses, PyMissingSemicolon,
    PyOutOfTokens, PyParsingError, PyUnexpectedToken,
};
use self::py_positioned_token::PyPositionedToken;
use self::py_statement::{OwnedStatement, PyExpressionStatement, PyStatement};
use self::py_substring_position::PySubstringPosition;
use self::py_token::PyToken;
use self::py_token_kind::PyTokenKind;
use self::py_unary_arithmetic_operator::PyUnaryArithmeticOperator;

mod macros;
mod py_binary_arithmetic_operator;
mod py_character_position;
mod py_comparison_operator;
mod py_expression;
mod py_lexical_error;
mod py_numeric_literal_type;
mod py_numeric_literal_value_kind;
mod py_parsing_error;
mod py_positioned_token;
mod py_statement;
mod py_substring_position;
mod py_token;
mod py_token_kind;
mod py_unary_arithmetic_operator;
mod traits;

#[pyfunction]
fn parse_tokens<'py>(
    tokens: Vec<PyRef<'py, PyPositionedToken>>,
    py: Python<'py>,
) -> PyResult<Vec<PyObject>> {
    tokens
        .iter()
        .map(|token| token.deref().into())
        .collect::<Vec<_>>()
        .try_parse()
        .map(|statements| {
            statements
                .into_iter()
                .map(|statement| OwnedStatement::from(statement).into_py(py))
                .collect::<Vec<_>>()
        })
        .map_err(|error| OwnedParsingError::from(error).into())
}

#[pyfunction]
fn tokenize_string(string: &str) -> PyResult<Vec<PyPositionedToken>> {
    string
        .try_tokenize()
        .map(|tokens| {
            tokens
                .into_iter()
                .map(PyPositionedToken::from)
                .collect::<Vec<_>>()
        })
        .map_err(|error| OwnedLexicalError::from(error).into())
}

#[pymodule]
fn _az(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    // classes
    module.add_class::<PyCharacterPosition>()?;
    module.add_class::<PySubstringPosition>()?;
    module.add_class::<PyPositionedToken>()?;
    module.add_class::<PyToken>()?;

    module.add_class::<PyExpression>()?;
    module.add_class::<PyAnnotatedIdentifier>()?;
    module.add_class::<PyAssignment>()?;
    module.add_class::<PyBinaryOperation>()?;
    module.add_class::<PyBlock>()?;
    module.add_class::<PyCall>()?;
    module.add_class::<PyComparison>()?;
    module.add_class::<PyConditional>()?;
    module.add_class::<PyIdentifier>()?;
    module.add_class::<PyMemberAccess>()?;
    module.add_class::<PyUnaryOperation>()?;
    module.add_class::<PyNumericLiteral>()?;
    module.add_class::<PyTuple>()?;

    module.add_class::<PyStatement>()?;
    module.add_class::<PyExpressionStatement>()?;

    // enumerations
    module.add_class::<PyBinaryArithmeticOperator>()?;
    module.add_class::<PyComparisonOperator>()?;
    module.add_class::<PyNumericLiteralType>()?;
    module.add_class::<PyNumericLiteralValueKind>()?;
    module.add_class::<PyTokenKind>()?;
    module.add_class::<PyUnaryArithmeticOperator>()?;

    // exceptions
    module.add_class::<PyLexicalError>()?;
    module.add_class::<PyIdentifierIncomplete>()?;
    module.add_class::<PyIdentifierUnexpectedCharacter>()?;
    module.add_class::<PyCommentBlockIncomplete>()?;
    module.add_class::<PyNumericLiteralTypeSuffixIncomplete>()?;
    module.add_class::<PyNumericLiteralTypeSuffixUnexpectedCharacter>()?;
    module.add_class::<PyNumericLiteralTypeSuffixUnknown>()?;
    module.add_class::<PyNumericLiteralValueIncomplete>()?;
    module.add_class::<PyNumericLiteralValueTypeSuffixConflict>()?;
    module.add_class::<PyNumericLiteralValueUnexpectedCharacter>()?;
    module.add_class::<PyUnexpectedCharacter>()?;

    module.add_class::<PyParsingError>()?;
    module.add_class::<PyMismatchedOpenParentheses>()?;
    module.add_class::<PyMissingSemicolon>()?;
    module.add_class::<PyOutOfTokens>()?;
    module.add_class::<PyUnexpectedToken>()?;

    // functions
    {
        let parse_tokens_pyfunction = wrap_pyfunction!(parse_tokens, module)?;
        parse_tokens_pyfunction.setattr("__module__", "az.parsing")?;
        module.add_function(parse_tokens_pyfunction)?;
    }
    {
        let tokenize_string_pyfunction =
            wrap_pyfunction!(tokenize_string, module)?;
        tokenize_string_pyfunction.setattr("__module__", "az.tokenization")?;
        module.add_function(tokenize_string_pyfunction)?;
    }
    Ok(())
}
