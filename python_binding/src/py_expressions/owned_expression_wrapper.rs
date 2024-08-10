use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python};

use az::parsing::Expression;

use crate::traits::Repr;
use crate::types::OwnedString;

use super::py_annotated_identifier::PyAnnotatedIdentifier;
use super::py_assignment::PyAssignment;
use super::py_bidirectional_conditional::PyBidirectionalConditional;
use super::py_binary_arithmetic_operation::PyBinaryArithmeticOperation;
use super::py_binary_comparison::PyBinaryComparison;
use super::py_block::PyBlock;
use super::py_call::PyCall;
use super::py_function_definition::PyFunctionDefinition;
use super::py_function_type::PyFunctionType;
use super::py_grouping::PyGrouping;
use super::py_identifier::PyIdentifier;
use super::py_member_access::PyMemberAccess;
use super::py_numeric_literal::PyNumericLiteral;
use super::py_return::PyReturn;
use super::py_tuple::PyTuple;
use super::py_unary_arithmetic_operation::PyUnaryArithmeticOperation;
use super::py_unidirectional_conditional::PyUnidirectionalConditional;
use super::py_while_loop::PyWhileLoop;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedExpressionWrapper(Expression<OwnedString>);

impl From<Expression<OwnedString>> for OwnedExpressionWrapper {
    fn from(value: Expression<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<OwnedExpressionWrapper> for Expression<OwnedString> {
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
            Expression::BidirectionalConditional(value) => {
                PyBidirectionalConditional::from(value).into_py(py)
            }
            Expression::BinaryComparison(value) => {
                PyBinaryComparison::from(value).into_py(py)
            }
            Expression::FunctionDefinition(value) => {
                PyFunctionDefinition::from(value).into_py(py)
            }
            Expression::FunctionType(value) => {
                PyFunctionType::from(value).into_py(py)
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
            Expression::Return(value) => PyReturn::from(value).into_py(py),
            Expression::Tuple(value) => PyTuple::from(value).into_py(py),
            Expression::UnaryArithmeticOperation(value) => {
                PyUnaryArithmeticOperation::from(value).into_py(py)
            }
            Expression::UnidirectionalConditional(value) => {
                PyUnidirectionalConditional::from(value).into_py(py)
            }
            Expression::WhileLoop(value) => {
                PyWhileLoop::from(value).into_py(py)
            }
        }
    }
}

impl<'source> FromPyObject<'source> for OwnedExpressionWrapper {
    fn extract_bound(object: &Bound<'source, PyAny>) -> PyResult<Self> {
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
                object.extract::<PyBidirectionalConditional>().map(|value| {
                    OwnedExpressionWrapper(
                        Expression::BidirectionalConditional(value.into()),
                    )
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
                object.extract::<PyFunctionDefinition>().map(|value| {
                    OwnedExpressionWrapper(Expression::FunctionDefinition(
                        value.into(),
                    ))
                })
            })
            .or_else(|_| {
                object.extract::<PyFunctionType>().map(|value| {
                    OwnedExpressionWrapper(Expression::FunctionType(
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
                object.extract::<PyReturn>().map(|value| {
                    OwnedExpressionWrapper(Expression::Return(value.into()))
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
            .or_else(|_| {
                object
                    .extract::<PyUnidirectionalConditional>()
                    .map(|value| {
                        OwnedExpressionWrapper(
                            Expression::UnidirectionalConditional(
                                value.into(),
                            ),
                        )
                    })
            })
            .or_else(|_| {
                object.extract::<PyWhileLoop>().map(|value| {
                    OwnedExpressionWrapper(Expression::WhileLoop(value.into()))
                })
            })
    }
}

impl Repr for Expression<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            Expression::AnnotatedIdentifier(value) => value.repr(py),
            Expression::Assignment(value) => value.repr(py),
            Expression::BinaryArithmeticOperation(value) => value.repr(py),
            Expression::Block(value) => value.repr(py),
            Expression::Call(value) => value.repr(py),
            Expression::BidirectionalConditional(value) => value.repr(py),
            Expression::BinaryComparison(value) => value.repr(py),
            Expression::FunctionDefinition(value) => value.repr(py),
            Expression::FunctionType(value) => value.repr(py),
            Expression::Grouping(value) => value.repr(py),
            Expression::Identifier(value) => value.repr(py),
            Expression::MemberAccess(value) => value.repr(py),
            Expression::NumericLiteral(value) => value.repr(py),
            Expression::Return(value) => value.repr(py),
            Expression::Tuple(value) => value.repr(py),
            Expression::UnaryArithmeticOperation(value) => value.repr(py),
            Expression::UnidirectionalConditional(value) => value.repr(py),
            Expression::WhileLoop(value) => value.repr(py),
        }
    }
}

impl Repr for OwnedExpressionWrapper {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
