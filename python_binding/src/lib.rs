use pyo3::types::{PyAnyMethods, PyModule, PyModuleMethods};
use pyo3::{
    pyfunction, pymodule, wrap_pyfunction_bound, Bound, PyResult, Python,
};

use az::parsing::is_keyword;

use self::py_annotation_operator::PyAnnotationOperator;
use self::py_assignment_operator::PyAssignmentOperator;
use self::py_associativity::PyAssociativity;
use self::py_binary_arithmetic_operators::{
    PyBinaryAdditionOperator, PyBinaryDivisionOperator,
    PyBinaryMultiplicationOperator, PyBinarySubtractionOperator,
};
use self::py_binary_comparison_operators::{
    PyBinaryEqualToOperator, PyBinaryGreaterThanOperator,
    PyBinaryGreaterThanOrEqualToOperator, PyBinaryLessThanOperator,
    PyBinaryLessThanOrEqualToOperator, PyBinaryNotEqualToOperator,
};
use self::py_byte_count::PyByteCount;
use self::py_call_operator::PyCallOperator;
use self::py_character_position::PyCharacterPosition;
use self::py_expressions::{
    PyAnnotatedIdentifier, PyAssignment, PyBidirectionalConditional,
    PyBinaryArithmeticOperation, PyBinaryComparison, PyBlock, PyCall,
    PyExpression, PyFunctionDefinition, PyFunctionType, PyGrouping,
    PyIdentifier, PyMemberAccess, PyNumericLiteral, PyReturn, PyTuple,
    PyUnaryArithmeticOperation, PyUnidirectionalConditional, PyWhileLoop,
};
use self::py_filler::PyFiller;
use self::py_filler_content::PyFillerContent;
use self::py_filler_kind::PyFillerKind;
use self::py_function_type_operator::PyFunctionTypeOperator;
use self::py_lexical_errors::{
    PyCommentBlockIncomplete, PyIdentifierIncomplete,
    PyIdentifierUnexpectedCharacter, PyLexicalError,
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
use self::py_return_operator::PyReturnOperator;
use self::py_script::PyScript;
use self::py_statements::{PyExpressionStatement, PyStatement};
use self::py_substring_position::PySubstringPosition;
use self::py_token::PyToken;
use self::py_token_collection::PyTokenCollection;
use self::py_token_content::PyTokenContent;
use self::py_token_kind::PyTokenKind;
use self::py_unary_arithmetic_operators::PyUnaryNegationOperator;
use self::py_utf_8_count::PyUtf8Count;
use self::py_validation_errors::{
    PyContentsValidationError, PyPositionsValidationError, PyValidationError,
};

mod macros;
mod py_annotation_operator;
mod py_assignment_operator;
mod py_associativity;
mod py_binary_arithmetic_operators;
mod py_binary_comparison_operators;
mod py_byte_count;
mod py_call_operator;
mod py_character_position;
mod py_expressions;
mod py_filler;
mod py_filler_content;
mod py_filler_kind;
mod py_function_type_operator;
mod py_lexical_errors;
mod py_member_access_operator;
mod py_numeric_literal_type;
mod py_numeric_literal_value_kind;
mod py_parsing_errors;
mod py_precedence;
mod py_return_operator;
mod py_script;
mod py_statements;
mod py_substring_position;
mod py_token;
mod py_token_collection;
mod py_token_content;
mod py_token_kind;
mod py_unary_arithmetic_operators;
mod py_utf_8_count;
mod py_validation_errors;
mod traits;
mod types;
mod validation;

const PARSING_MODULE_NAME: &str = "az.parsing";

#[pyfunction]
#[pyo3(name = "is_keyword")]
#[pyo3(signature = (value, /))]
fn py_is_keyword(value: &str) -> bool {
    is_keyword(value)
}

#[pymodule]
fn _az(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    module.add_class::<PyByteCount>()?;
    module.add_class::<PyCharacterPosition>()?;
    module.add_class::<PyFiller>()?;
    module.add_class::<PyFillerContent>()?;
    module.add_class::<PyPrecedence>()?;
    module.add_class::<PySubstringPosition>()?;
    module.add_class::<PyToken>()?;
    module.add_class::<PyTokenCollection>()?;
    module.add_class::<PyTokenContent>()?;
    module.add_class::<PyUtf8Count>()?;

    module.add_class::<PyBinaryAdditionOperator>()?;
    module.add_class::<PyBinaryDivisionOperator>()?;
    module.add_class::<PyBinaryMultiplicationOperator>()?;
    module.add_class::<PyBinarySubtractionOperator>()?;

    module.add_class::<PyAnnotationOperator>()?;
    module.add_class::<PyAssignmentOperator>()?;
    module.add_class::<PyCallOperator>()?;
    module.add_class::<PyFunctionTypeOperator>()?;
    module.add_class::<PyMemberAccessOperator>()?;

    module.add_class::<PyBinaryEqualToOperator>()?;
    module.add_class::<PyBinaryGreaterThanOperator>()?;
    module.add_class::<PyBinaryGreaterThanOrEqualToOperator>()?;
    module.add_class::<PyBinaryLessThanOperator>()?;
    module.add_class::<PyBinaryLessThanOrEqualToOperator>()?;
    module.add_class::<PyBinaryNotEqualToOperator>()?;

    module.add_class::<PyReturnOperator>()?;
    module.add_class::<PyUnaryNegationOperator>()?;

    module.add_class::<PyExpression>()?;
    module.add_class::<PyAnnotatedIdentifier>()?;
    module.add_class::<PyAssignment>()?;
    module.add_class::<PyBidirectionalConditional>()?;
    module.add_class::<PyBinaryArithmeticOperation>()?;
    module.add_class::<PyBinaryComparison>()?;
    module.add_class::<PyBlock>()?;
    module.add_class::<PyCall>()?;
    module.add_class::<PyFunctionDefinition>()?;
    module.add_class::<PyFunctionType>()?;
    module.add_class::<PyGrouping>()?;
    module.add_class::<PyIdentifier>()?;
    module.add_class::<PyMemberAccess>()?;
    module.add_class::<PyNumericLiteral>()?;
    module.add_class::<PyReturn>()?;
    module.add_class::<PyTuple>()?;
    module.add_class::<PyUnaryArithmeticOperation>()?;
    module.add_class::<PyUnidirectionalConditional>()?;
    module.add_class::<PyWhileLoop>()?;

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

    module.add_class::<PyValidationError>()?;
    module.add_class::<PyContentsValidationError>()?;
    module.add_class::<PyPositionsValidationError>()?;

    module.add_class::<PyParsingError>()?;
    module.add_class::<PyMismatchedOpenBrace>()?;
    module.add_class::<PyMismatchedOpenParenthesis>()?;
    module.add_class::<PyMissingSemicolon>()?;
    module.add_class::<PyOutOfTokens>()?;
    module.add_class::<PyUnexpectedExpression>()?;
    module.add_class::<PyUnexpectedToken>()?;

    {
        let py_function = wrap_pyfunction_bound!(py_is_keyword, module)?;
        py_function.setattr("__module__", PARSING_MODULE_NAME)?;
        module.add_function(py_function)?;
    }

    Ok(())
}
