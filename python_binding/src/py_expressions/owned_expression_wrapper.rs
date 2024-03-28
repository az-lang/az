use pyo3::{FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python};

use az::parsing::Expression;

use crate::traits::Repr;
use crate::types::OwnedStr;

use super::py_annotated_identifier::PyAnnotatedIdentifier;
use super::py_assignment::PyAssignment;
use super::py_binary_arithmetic_operation::PyBinaryArithmeticOperation;
use super::py_binary_comparison::PyBinaryComparison;
use super::py_block::PyBlock;
use super::py_call::PyCall;
use super::py_conditional::PyConditional;
use super::py_function_definition::PyFunctionDefinition;
use super::py_grouping::PyGrouping;
use super::py_identifier::PyIdentifier;
use super::py_member_access::PyMemberAccess;
use super::py_numeric_literal::PyNumericLiteral;
use super::py_tuple::PyTuple;
use super::py_unary_arithmetic_operation::PyUnaryArithmeticOperation;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedExpressionWrapper(Expression<OwnedStr>);

impl From<Expression<OwnedStr>> for OwnedExpressionWrapper {
    fn from(value: Expression<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<OwnedExpressionWrapper> for Expression<OwnedStr> {
    fn from(value: OwnedExpressionWrapper) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for OwnedExpressionWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            Expression::AnnotatedIdentifier(value) => {
                PyAnnotatedIdentifier::from(value).into_py(py)
            }
            Expression::Assignment(value) => {
                PyAssignment::from(value).into_py(py)
            }
            Expression::BinaryArithmeticOperation(value) => {
                PyBinaryArithmeticOperation::from(value).into_py(py)
            }
            Expression::Block(value) => PyBlock::from(value).into_py(py),
            Expression::Call(value) => PyCall::from(value).into_py(py),
            Expression::BinaryComparison(value) => {
                PyBinaryComparison::from(value).into_py(py)
            }
            Expression::Conditional(value) => {
                PyConditional::from(value).into_py(py)
            }
            Expression::FunctionDefinition(value) => {
                PyFunctionDefinition::from(value).into_py(py)
            }
            Expression::Grouping(value) => PyGrouping::from(value).into_py(py),
            Expression::Identifier(value) => {
                PyIdentifier::from(value).into_py(py)
            }
            Expression::MemberAccess(value) => {
                PyMemberAccess::from(value).into_py(py)
            }
            Expression::NumericLiteral(value) => {
                PyNumericLiteral::from(value).into_py(py)
            }
            Expression::Tuple(value) => PyTuple::from(value).into_py(py),
            Expression::UnaryArithmeticOperation(value) => {
                PyUnaryArithmeticOperation::from(value).into_py(py)
            }
        }
    }
}

impl<'source> FromPyObject<'source> for OwnedExpressionWrapper {
    fn extract(object: &'source PyAny) -> PyResult<Self> {
        object
            .extract::<PyAnnotatedIdentifier>()
            .map(|value| {
                OwnedExpressionWrapper(Expression::AnnotatedIdentifier(
                    value.into(),
                ))
            })
            .or_else(|_| {
                object.extract::<PyAssignment>().map(|value| {
                    OwnedExpressionWrapper(Expression::Assignment(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object
                    .extract::<PyBinaryArithmeticOperation>()
                    .map(|value| {
                        OwnedExpressionWrapper(
                            Expression::BinaryArithmeticOperation(
                                value.into(),
                            ),
                        )
                    })
            })
            .or_else(|_| {
                object.extract::<PyBlock>().map(|value| {
                    OwnedExpressionWrapper(Expression::Block(value.into()))
                })
            })
            .or_else(|_| {
                object.extract::<PyCall>().map(|value| {
                    OwnedExpressionWrapper(Expression::Call(value.into()))
                })
            })
            .or_else(|_| {
                object.extract::<PyBinaryComparison>().map(|value| {
                    OwnedExpressionWrapper(Expression::BinaryComparison(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyConditional>().map(|value| {
                    OwnedExpressionWrapper(Expression::Conditional(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyFunctionDefinition>().map(|value| {
                    OwnedExpressionWrapper(Expression::FunctionDefinition(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyGrouping>().map(|value| {
                    OwnedExpressionWrapper(Expression::Grouping(value.into()))
                })
            })
            .or_else(|_| {
                object.extract::<PyIdentifier>().map(|value| {
                    OwnedExpressionWrapper(Expression::Identifier(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyMemberAccess>().map(|value| {
                    OwnedExpressionWrapper(Expression::MemberAccess(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyNumericLiteral>().map(|value| {
                    OwnedExpressionWrapper(Expression::NumericLiteral(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyTuple>().map(|value| {
                    OwnedExpressionWrapper(Expression::Tuple(value.into()))
                })
            })
            .or_else(|_| {
                object.extract::<PyUnaryArithmeticOperation>().map(|value| {
                    OwnedExpressionWrapper(
                        Expression::UnaryArithmeticOperation(value.into()),
                    )
                })
            })
    }
}

impl Repr for Expression<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            Expression::AnnotatedIdentifier(value) => value.repr(py),
            Expression::Assignment(value) => value.repr(py),
            Expression::BinaryArithmeticOperation(value) => value.repr(py),
            Expression::Block(value) => value.repr(py),
            Expression::Call(value) => value.repr(py),
            Expression::BinaryComparison(value) => value.repr(py),
            Expression::Conditional(value) => value.repr(py),
            Expression::FunctionDefinition(value) => value.repr(py),
            Expression::Grouping(value) => value.repr(py),
            Expression::Identifier(value) => value.repr(py),
            Expression::MemberAccess(value) => value.repr(py),
            Expression::NumericLiteral(value) => value.repr(py),
            Expression::Tuple(value) => value.repr(py),
            Expression::UnaryArithmeticOperation(value) => value.repr(py),
        }
    }
}

impl Repr for OwnedExpressionWrapper {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
