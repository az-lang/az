use az::parsing::{Block, Expression, Identifier};
use pyo3::{
    pyclass, pymethods, FromPyObject, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use super::py_binary_arithmetic_operator::PyBinaryArithmeticOperator;
use super::py_comparison_operator::PyComparisonOperator;
use super::py_numeric_literal_type::PyNumericLiteralType;
use super::py_statement::OwnedStatement;
use super::py_substring_position::PySubstringPosition;
use super::py_unary_arithmetic_operator::PyUnaryArithmeticOperator;
use super::traits::Repr;

#[pyclass(module = "az.parsing", name = "Expression", frozen, subclass)]
pub(super) struct PyExpression {}

impl PyExpression {
    fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "AnnotatedIdentifier", extends = PyExpression, frozen, get_all)]
pub(super) struct PyAnnotatedIdentifier {
    identifier: PyIdentifier,
    annotation: OwnedExpression,
}

impl PyAnnotatedIdentifier {
    fn new(
        identifier: PyIdentifier,
        annotation: OwnedExpression,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            identifier,
            annotation,
        })
    }
}

#[pymethods]
impl PyAnnotatedIdentifier {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Assignment", extends = PyExpression, frozen, get_all)]
pub(super) struct PyAssignment {
    target: OwnedExpression,
    value: OwnedExpression,
}

impl PyAssignment {
    fn new(
        target: OwnedExpression,
        value: OwnedExpression,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self { target, value })
    }
}

#[pymethods]
impl PyAssignment {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "BinaryOperation", extends = PyExpression, frozen, get_all)]
pub(super) struct PyBinaryOperation {
    left: OwnedExpression,
    right: OwnedExpression,
    operator: PyBinaryArithmeticOperator,
}

impl PyBinaryOperation {
    fn new(
        left: OwnedExpression,
        right: OwnedExpression,
        operator: PyBinaryArithmeticOperator,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            left,
            right,
            operator,
        })
    }
}

#[pymethods]
impl PyBinaryOperation {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Block", extends = PyExpression, frozen, get_all)]
pub(super) struct PyBlock {
    statements: Vec<OwnedStatement>,
    expression: Option<OwnedExpression>,
}

impl PyBlock {
    fn new(
        statements: Vec<OwnedStatement>,
        expression: Option<OwnedExpression>,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            statements,
            expression,
        })
    }
}

#[pymethods]
impl PyBlock {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Call", extends = PyExpression, frozen, get_all)]
pub(super) struct PyCall {
    callable: OwnedExpression,
    arguments: Vec<OwnedExpression>,
}

impl PyCall {
    fn new(
        callable: OwnedExpression,
        arguments: Vec<OwnedExpression>,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            callable,
            arguments,
        })
    }
}

#[pymethods]
impl PyCall {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Comparison", extends = PyExpression, frozen, get_all)]
pub(super) struct PyComparison {
    left: OwnedExpression,
    operator: PyComparisonOperator,
    right: OwnedExpression,
}

impl PyComparison {
    fn new(
        left: OwnedExpression,
        operator: PyComparisonOperator,
        right: OwnedExpression,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            left,
            right,
            operator,
        })
    }
}

#[pymethods]
impl PyComparison {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Conditional", extends = PyExpression, frozen, get_all)]
pub(super) struct PyConditional {
    antecedent: OwnedExpression,
    consequent: OwnedExpression,
    alternative: Option<OwnedExpression>,
}

impl PyConditional {
    fn new(
        antecedent: OwnedExpression,
        consequent: OwnedExpression,
        alternative: Option<OwnedExpression>,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            antecedent,
            consequent,
            alternative,
        })
    }
}

#[pymethods]
impl PyConditional {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "FunctionDefinition", extends = PyExpression, frozen, get_all)]
pub(super) struct PyFunctionDefinition {
    parameters: Vec<OwnedExpression>,
    return_type: OwnedExpression,
    body: OwnedExpression,
}

impl PyFunctionDefinition {
    fn new(
        parameters: Vec<OwnedExpression>,
        return_type: OwnedExpression,
        body: OwnedExpression,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            parameters,
            return_type,
            body,
        })
    }
}

#[pymethods]
impl PyFunctionDefinition {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Identifier", extends = PyExpression, frozen, get_all)]
pub(super) struct PyIdentifier {
    string: String,
    position: PySubstringPosition,
}

impl PyIdentifier {
    fn new(
        string: String,
        position: PySubstringPosition,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self { string, position })
    }
}

#[pymethods]
impl PyIdentifier {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "MemberAccess", extends = PyExpression, frozen, get_all)]
pub(super) struct PyMemberAccess {
    object: OwnedExpression,
    member: PyIdentifier,
}

impl PyMemberAccess {
    fn new(
        object: OwnedExpression,
        member: PyIdentifier,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self { object, member })
    }
}

#[pymethods]
impl PyMemberAccess {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "NumericLiteral", extends = PyExpression, frozen, get_all)]
pub(super) struct PyNumericLiteral {
    string: String,
    type_: PyNumericLiteralType,
    position: PySubstringPosition,
}

impl PyNumericLiteral {
    fn new(
        string: String,
        kind: PyNumericLiteralType,
        position: PySubstringPosition,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self {
            string,
            type_: kind,
            position,
        })
    }
}

#[pymethods]
impl PyNumericLiteral {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Tuple", extends = PyExpression, frozen, get_all)]
pub(super) struct PyTuple {
    elements: Vec<OwnedExpression>,
}

impl PyTuple {
    fn new(elements: Vec<OwnedExpression>) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self { elements })
    }
}

#[pymethods]
impl PyTuple {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "UnaryOperation", extends = PyExpression, frozen, get_all)]
pub(super) struct PyUnaryOperation {
    operand: OwnedExpression,
    operator: PyUnaryArithmeticOperator,
}

impl PyUnaryOperation {
    fn new(
        operand: OwnedExpression,
        operator: PyUnaryArithmeticOperator,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self { operand, operator })
    }
}

#[pymethods]
impl PyUnaryOperation {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
pub(super) enum OwnedExpression {
    AnnotatedIdentifier(Box<PyAnnotatedIdentifier>),
    Assignment(Box<PyAssignment>),
    Block(Box<PyBlock>),
    BinaryOperation(Box<PyBinaryOperation>),
    Call(Box<PyCall>),
    Comparison(Box<PyComparison>),
    Conditional(Box<PyConditional>),
    FunctionDefinition(Box<PyFunctionDefinition>),
    Identifier(PyIdentifier),
    MemberAccess(Box<PyMemberAccess>),
    NumericLiteral(PyNumericLiteral),
    Tuple(Box<PyTuple>),
    UnaryOperation(Box<PyUnaryOperation>),
}

impl<'a> From<Identifier<'a>> for PyIdentifier {
    fn from(value: Identifier<'a>) -> Self {
        Self {
            string: value.string.to_string(),
            position: value.position.into(),
        }
    }
}

impl<'a> From<Expression<'a>> for OwnedExpression {
    fn from(value: Expression<'a>) -> Self {
        match value {
            Expression::AnnotatedIdentifier {
                identifier,
                annotation,
            } => Self::AnnotatedIdentifier(Box::new(PyAnnotatedIdentifier {
                identifier: identifier.into(),
                annotation: (*annotation).into(),
            })),
            Expression::Assignment { target, value } => {
                Self::Assignment(Box::new(PyAssignment {
                    target: (*target).into(),
                    value: (*value).into(),
                }))
            }
            Expression::BinaryArithmeticOperation {
                left,
                operator,
                right,
            } => Self::BinaryOperation(Box::new(PyBinaryOperation {
                left: (*left).into(),
                right: (*right).into(),
                operator: operator.into(),
            })),
            Expression::Block(Block {
                statements,
                expression,
            }) => Self::Block(Box::new(PyBlock {
                statements: statements.into_iter().map(Into::into).collect(),
                expression: expression.map(|value| (*value).into()),
            })),
            Expression::Call {
                callable,
                arguments,
            } => Self::Call(Box::new(PyCall {
                callable: (*callable).into(),
                arguments: arguments.into_iter().map(Into::into).collect(),
            })),
            Expression::Comparison {
                left,
                operator,
                right,
            } => Self::Comparison(Box::new(PyComparison {
                left: (*left).into(),
                operator: operator.into(),
                right: (*right).into(),
            })),
            Expression::Conditional {
                antecedent,
                consequent,
                alternative,
            } => Self::Conditional(Box::new(PyConditional {
                antecedent: (*antecedent).into(),
                consequent: (*consequent).into(),
                alternative: alternative.map(|value| (*value).into()),
            })),
            Expression::FunctionDefinition {
                parameters,
                return_type,
                body,
            } => Self::FunctionDefinition(Box::new(PyFunctionDefinition {
                parameters: parameters.into_iter().map(Into::into).collect(),
                return_type: (*return_type).into(),
                body: Self::Block(Box::new(PyBlock {
                    statements: body
                        .statements
                        .into_iter()
                        .map(|value| value.into())
                        .collect(),
                    expression: body.expression.map(|value| (*value).into()),
                })),
            })),
            Expression::Identifier(Identifier { position, string }) => {
                Self::Identifier(PyIdentifier {
                    position: position.into(),
                    string: string.to_string(),
                })
            }
            Expression::MemberAccess { member, object } => {
                Self::MemberAccess(Box::new(PyMemberAccess {
                    object: (*object).into(),
                    member: member.into(),
                }))
            }
            Expression::NumericLiteral {
                type_: kind,
                position,
                value: string,
            } => Self::NumericLiteral(PyNumericLiteral {
                type_: kind.into(),
                position: position.into(),
                string: string.to_string(),
            }),
            Expression::Tuple { elements } => Self::Tuple(Box::new(PyTuple {
                elements: elements.into_iter().map(Into::into).collect(),
            })),
            Expression::UnaryArithmeticOperation { operand, operator } => {
                Self::UnaryOperation(Box::new(PyUnaryOperation {
                    operand: (*operand).into(),
                    operator: operator.into(),
                }))
            }
        }
    }
}

impl<'source> FromPyObject<'source> for OwnedExpression {
    fn extract(object: &'source PyAny) -> PyResult<Self> {
        object
            .extract::<PyAnnotatedIdentifier>()
            .map(Box::new)
            .map(Self::AnnotatedIdentifier)
            .or_else(|_| {
                object
                    .extract::<PyAssignment>()
                    .map(Box::new)
                    .map(Self::Assignment)
            })
            .or_else(|_| {
                object
                    .extract::<PyBinaryOperation>()
                    .map(Box::new)
                    .map(Self::BinaryOperation)
            })
            .or_else(|_| {
                object.extract::<PyBlock>().map(Box::new).map(Self::Block)
            })
            .or_else(|_| {
                object.extract::<PyCall>().map(Box::new).map(Self::Call)
            })
            .or_else(|_| {
                object
                    .extract::<PyComparison>()
                    .map(Box::new)
                    .map(Self::Comparison)
            })
            .or_else(|_| {
                object
                    .extract::<PyConditional>()
                    .map(Box::new)
                    .map(Self::Conditional)
            })
            .or_else(|_| {
                object
                    .extract::<PyFunctionDefinition>()
                    .map(Box::new)
                    .map(Self::FunctionDefinition)
            })
            .or_else(|_| {
                object.extract::<PyIdentifier>().map(Self::Identifier)
            })
            .or_else(|_| {
                object
                    .extract::<PyMemberAccess>()
                    .map(Box::new)
                    .map(Self::MemberAccess)
            })
            .or_else(|_| {
                object
                    .extract::<PyNumericLiteral>()
                    .map(Self::NumericLiteral)
            })
            .or_else(|_| {
                object.extract::<PyTuple>().map(Box::new).map(Self::Tuple)
            })
            .or_else(|_| {
                object
                    .extract::<PyUnaryOperation>()
                    .map(Box::new)
                    .map(Self::UnaryOperation)
            })
    }
}

impl IntoPy<PyObject> for PyIdentifier {
    fn into_py(self, py: Python<'_>) -> PyObject {
        {
            Py::new(py, PyIdentifier::new(self.string, self.position))
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to create {}: {}.",
                        PyIdentifier::NAME,
                        error
                    )
                })
                .into_py(py)
        }
    }
}

impl IntoPy<PyObject> for OwnedExpression {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            OwnedExpression::AnnotatedIdentifier(wrapped) => Py::new(
                py,
                PyAnnotatedIdentifier::new(
                    wrapped.identifier,
                    wrapped.annotation,
                ),
            )
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to create {}: {}.",
                    PyAnnotatedIdentifier::NAME,
                    error
                )
            })
            .into_py(py),
            OwnedExpression::Assignment(wrapped) => {
                Py::new(py, PyAssignment::new(wrapped.target, wrapped.value))
                    .unwrap_or_else(|error| {
                        panic!(
                            "Failed to create {}: {}.",
                            PyAssignment::NAME,
                            error
                        )
                    })
                    .into_py(py)
            }
            OwnedExpression::BinaryOperation(wrapped) => Py::new(
                py,
                PyBinaryOperation::new(
                    wrapped.left,
                    wrapped.right,
                    wrapped.operator,
                ),
            )
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to create {}: {}.",
                    PyBinaryOperation::NAME,
                    error
                )
            })
            .into_py(py),
            OwnedExpression::Block(wrapped) => Py::new(
                py,
                PyBlock::new(wrapped.statements, wrapped.expression),
            )
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", PyBlock::NAME, error)
            })
            .into_py(py),
            OwnedExpression::Call(wrapped) => {
                Py::new(py, PyCall::new(wrapped.callable, wrapped.arguments))
                    .unwrap_or_else(|error| {
                        panic!("Failed to create {}: {}.", PyCall::NAME, error)
                    })
                    .into_py(py)
            }
            OwnedExpression::Comparison(wrapped) => Py::new(
                py,
                PyComparison::new(
                    wrapped.left,
                    wrapped.operator,
                    wrapped.right,
                ),
            )
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", PyComparison::NAME, error)
            })
            .into_py(py),
            OwnedExpression::Conditional(wrapped) => Py::new(
                py,
                PyConditional::new(
                    wrapped.antecedent,
                    wrapped.consequent,
                    wrapped.alternative,
                ),
            )
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", PyConditional::NAME, error)
            })
            .into_py(py),
            OwnedExpression::FunctionDefinition(wrapped) => Py::new(
                py,
                PyFunctionDefinition::new(
                    wrapped.parameters,
                    wrapped.return_type,
                    wrapped.body,
                ),
            )
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to create {}: {}.",
                    PyFunctionDefinition::NAME,
                    error
                )
            })
            .into_py(py),
            OwnedExpression::Identifier(wrapped) => wrapped.into_py(py),
            OwnedExpression::MemberAccess(wrapped) => Py::new(
                py,
                PyMemberAccess::new(wrapped.object, wrapped.member),
            )
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", PyMemberAccess::NAME, error)
            })
            .into_py(py),
            OwnedExpression::NumericLiteral(PyNumericLiteral {
                string,
                type_: kind,
                position,
            }) => Py::new(py, PyNumericLiteral::new(string, kind, position))
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to create {}: {}.",
                        PyNumericLiteral::NAME,
                        error
                    )
                })
                .into_py(py),
            OwnedExpression::Tuple(wrapped) => {
                Py::new(py, PyTuple::new(wrapped.elements))
                    .unwrap_or_else(|error| {
                        panic!(
                            "Failed to create {}: {}.",
                            PyTuple::NAME,
                            error
                        )
                    })
                    .into_py(py)
            }
            OwnedExpression::UnaryOperation(wrapped) => Py::new(
                py,
                PyUnaryOperation::new(wrapped.operand, wrapped.operator),
            )
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to create {}: {}.",
                    PyUnaryOperation::NAME,
                    error
                )
            })
            .into_py(py),
        }
    }
}

impl Repr for PyAnnotatedIdentifier {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.identifier.repr(py)?,
            self.annotation.repr(py)?,
        ))
    }
}

impl Repr for PyAssignment {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.target.repr(py)?,
            self.value.repr(py)?,
        ))
    }
}

impl Repr for PyBinaryOperation {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            Self::NAME,
            self.left.repr(py)?,
            self.right.repr(py)?,
            self.operator.repr(py)?
        ))
    }
}

impl Repr for PyBlock {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.statements.repr(py)?,
            self.expression.repr(py)?,
        ))
    }
}

impl Repr for PyCall {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.callable.repr(py)?,
            self.arguments.repr(py)?
        ))
    }
}

impl Repr for PyComparison {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            Self::NAME,
            self.left.repr(py)?,
            self.operator.repr(py)?,
            self.right.repr(py)?
        ))
    }
}

impl Repr for PyConditional {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            Self::NAME,
            self.antecedent.repr(py)?,
            self.consequent.repr(py)?,
            self.alternative.repr(py)?
        ))
    }
}

impl Repr for PyFunctionDefinition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            Self::NAME,
            self.parameters.repr(py)?,
            self.return_type.repr(py)?,
            self.body.repr(py)?
        ))
    }
}

impl Repr for PyIdentifier {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.string.repr(py)?,
            self.position.repr(py)?,
        ))
    }
}

impl Repr for PyMemberAccess {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.object.repr(py)?,
            self.member.repr(py)?
        ))
    }
}

impl Repr for PyUnaryOperation {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.operand.repr(py)?,
            self.operator.repr(py)?
        ))
    }
}

impl Repr for PyNumericLiteral {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            Self::NAME,
            self.string.repr(py)?,
            self.type_.repr(py)?,
            self.position.repr(py)?
        ))
    }
}

impl Repr for PyTuple {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("{}({})", Self::NAME, self.elements.repr(py)?))
    }
}

impl Repr for OwnedExpression {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            Self::AnnotatedIdentifier(wrapped) => wrapped.repr(py),
            Self::Assignment(wrapped) => wrapped.repr(py),
            Self::BinaryOperation(wrapped) => wrapped.repr(py),
            Self::Block(wrapped) => wrapped.repr(py),
            Self::Call(wrapped) => wrapped.repr(py),
            Self::Comparison(wrapped) => wrapped.repr(py),
            Self::Conditional(wrapped) => wrapped.repr(py),
            Self::FunctionDefinition(wrapped) => wrapped.repr(py),
            Self::Identifier(wrapped) => wrapped.repr(py),
            Self::NumericLiteral(wrapped) => wrapped.repr(py),
            Self::MemberAccess(wrapped) => wrapped.repr(py),
            Self::Tuple(wrapped) => wrapped.repr(py),
            Self::UnaryOperation(wrapped) => wrapped.repr(py),
        }
    }
}
