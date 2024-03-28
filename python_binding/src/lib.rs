use pyo3::types::{PyAnyMethods, PyModule};
use pyo3::{pyfunction, pymodule, wrap_pyfunction, Bound, PyResult, Python};

use az::tokenization::TryTokenize;

use self::py_associativity::PyAssociativity;
use self::py_binary_annotation_operator::PyBinaryAnnotationOperator;
use self::py_binary_arithmetic_operators::{
    PyBinaryAdditionOperator, PyBinaryDivisionOperator,
    PyBinaryMultiplicationOperator, PyBinarySubtractionOperator,
};
use self::py_binary_assignment_operator::PyBinaryAssignmentOperator;
use self::py_binary_comparison_operators::{
    PyBinaryEqualToOperator, PyBinaryGreaterThanOperator,
    PyBinaryGreaterThanOrEqualToOperator, PyBinaryLowerThanOperator,
    PyBinaryLowerThanOrEqualToOperator, PyBinaryNotEqualToOperator,
};
use self::py_byte_index::PyByteIndex;
use self::py_call_operator::PyCallOperator;
use self::py_character_position::PyCharacterPosition;
use self::py_expressions::{
    PyAnnotatedIdentifier, PyAssignment, PyBinaryArithmeticOperation,
    PyBinaryComparison, PyBlock, PyCall, PyConditional, PyExpression,
    PyFunctionDefinition, PyGrouping, PyIdentifier, PyMemberAccess,
    PyNumericLiteral, PyTuple, PyUnaryArithmeticOperation,
};
use self::py_filler::PyFiller;
use self::py_filler_content::PyFillerContent;
use self::py_filler_kind::PyFillerKind;
use self::py_lexical_errors::{
    OwnedLexicalErrorWrapper, PyCommentBlockIncomplete,
    PyIdentifierIncomplete, PyIdentifierUnexpectedCharacter, PyLexicalError,
    PyNumericLiteralTypeSuffixIncomplete,
    PyNumericLiteralTypeSuffixUnexpectedCharacter,
    PyNumericLiteralTypeSuffixUnknown, PyNumericLiteralValueIncomplete,
    PyNumericLiteralValueTypeSuffixConflict,
    PyNumericLiteralValueUnexpectedCharacter, PyUnexpectedCharacter,
};
use self::py_member_access_operator::PyMemberAccessOperator;
use self::py_numeric_literal_type::PyNumericLiteralType;
use self::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use self::py_parsing_errors::{
    PyMismatchedOpenBrace, PyMismatchedOpenParenthesis, PyMissingSemicolon,
    PyOutOfTokens, PyParsingError, PyUnexpectedExpression, PyUnexpectedToken,
};
use self::py_precedence::PyPrecedence;
use self::py_script::PyScript;
use self::py_statements::{PyExpressionStatement, PyStatement};
use self::py_substring_position::PySubstringPosition;
use self::py_token::PyToken;
use self::py_token_content::PyTokenContent;
use self::py_token_kind::PyTokenKind;
use self::py_unary_arithmetic_operator::PyUnaryNegationOperator;
use self::py_utf8_index::PyUtf8Index;

mod macros;
mod py_associativity;
mod py_binary_annotation_operator;
mod py_binary_arithmetic_operators;
mod py_binary_assignment_operator;
mod py_binary_comparison_operators;
mod py_byte_index;
mod py_call_operator;
mod py_character_position;
mod py_expressions;
mod py_filler;
mod py_filler_content;
mod py_filler_kind;
mod py_lexical_errors;
mod py_member_access_operator;
mod py_numeric_literal_type;
mod py_numeric_literal_value_kind;
mod py_parsing_errors;
mod py_precedence;
mod py_script;
mod py_statements;
mod py_substring_position;
mod py_token;
mod py_token_content;
mod py_token_kind;
mod py_unary_arithmetic_operator;
mod py_utf8_index;
mod traits;
mod types;

#[pyfunction]
fn tokenize_string(string: &str) -> PyResult<Vec<PyToken>> {
    string
        .try_tokenize()
        .map(|tokens| {
            tokens.into_iter().map(PyToken::from).collect::<Vec<_>>()
        })
        .map_err(|error| OwnedLexicalErrorWrapper::from(error).into())
}

#[pymodule]
fn _az(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    module.add_class::<PyByteIndex>()?;
    module.add_class::<PyCharacterPosition>()?;
    module.add_class::<PyFiller>()?;
    module.add_class::<PyFillerContent>()?;
    module.add_class::<PyPrecedence>()?;
    module.add_class::<PySubstringPosition>()?;
    module.add_class::<PyToken>()?;
    module.add_class::<PyTokenContent>()?;
    module.add_class::<PyUtf8Index>()?;

    module.add_class::<PyBinaryAdditionOperator>()?;
    module.add_class::<PyBinaryDivisionOperator>()?;
    module.add_class::<PyBinaryMultiplicationOperator>()?;
    module.add_class::<PyBinarySubtractionOperator>()?;

    module.add_class::<PyBinaryAnnotationOperator>()?;
    module.add_class::<PyBinaryAssignmentOperator>()?;
    module.add_class::<PyCallOperator>()?;
    module.add_class::<PyMemberAccessOperator>()?;

    module.add_class::<PyBinaryEqualToOperator>()?;
    module.add_class::<PyBinaryGreaterThanOperator>()?;
    module.add_class::<PyBinaryGreaterThanOrEqualToOperator>()?;
    module.add_class::<PyBinaryLowerThanOperator>()?;
    module.add_class::<PyBinaryLowerThanOrEqualToOperator>()?;
    module.add_class::<PyBinaryNotEqualToOperator>()?;

    module.add_class::<PyUnaryNegationOperator>()?;

    module.add_class::<PyExpression>()?;
    module.add_class::<PyAnnotatedIdentifier>()?;
    module.add_class::<PyAssignment>()?;
    module.add_class::<PyBinaryArithmeticOperation>()?;
    module.add_class::<PyBinaryComparison>()?;
    module.add_class::<PyBlock>()?;
    module.add_class::<PyCall>()?;
    module.add_class::<PyConditional>()?;
    module.add_class::<PyFunctionDefinition>()?;
    module.add_class::<PyGrouping>()?;
    module.add_class::<PyIdentifier>()?;
    module.add_class::<PyMemberAccess>()?;
    module.add_class::<PyUnaryArithmeticOperation>()?;
    module.add_class::<PyNumericLiteral>()?;
    module.add_class::<PyTuple>()?;

    module.add_class::<PyScript>()?;

    module.add_class::<PyStatement>()?;
    module.add_class::<PyExpressionStatement>()?;

    // enumerations
    module.add_class::<PyAssociativity>()?;
    module.add_class::<PyFillerKind>()?;
    module.add_class::<PyNumericLiteralType>()?;
    module.add_class::<PyNumericLiteralValueKind>()?;
    module.add_class::<PyTokenKind>()?;

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
    module.add_class::<PyMismatchedOpenBrace>()?;
    module.add_class::<PyMismatchedOpenParenthesis>()?;
    module.add_class::<PyMissingSemicolon>()?;
    module.add_class::<PyOutOfTokens>()?;
    module.add_class::<PyUnexpectedExpression>()?;
    module.add_class::<PyUnexpectedToken>()?;

    // functions
    {
        let pyfunction = wrap_pyfunction!(tokenize_string, module)?;
        pyfunction.setattr("__module__", "az.tokenization")?;
        module.add_function(pyfunction)?;
    }
    Ok(())
}
