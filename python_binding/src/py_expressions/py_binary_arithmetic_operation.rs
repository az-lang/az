use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::BinaryArithmeticOperation;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_binary_arithmetic_operators::BinaryArithmeticOperatorWrapper;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "BinaryArithmeticOperation",
    extends = PyExpression, frozen
)]
pub(crate) struct PyBinaryArithmeticOperation(
    BinaryArithmeticOperation<OwnedStr>,
);

#[pymethods]
impl PyBinaryArithmeticOperation {
    #[getter]
    fn left(&self) -> OwnedExpressionWrapper {
        self.0.left.as_ref().clone().into()
    }

    #[getter]
    fn operator(&self) -> BinaryArithmeticOperatorWrapper {
        self.0.operator.into()
    }

    #[getter]
    fn operator_fillers(&self) -> PyFillers {
        self.0
            .operator_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn operator_position(&self) -> PySubstringPosition {
        self.0.operator_position.clone().into()
    }

    #[getter]
    fn right(&self) -> OwnedExpressionWrapper {
        self.0.right.as_ref().clone().into()
    }

    #[new]
    #[pyo3(signature = (left, right, operator, /, *, operator_position, operator_fillers))]
    fn new(
        left: OwnedExpressionWrapper,
        right: OwnedExpressionWrapper,
        operator: BinaryArithmeticOperatorWrapper,
        operator_position: PySubstringPosition,
        operator_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(BinaryArithmeticOperation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator: operator.into(),
            operator_position: operator_position.into(),
            operator_fillers: operator_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyclass::CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<BinaryArithmeticOperation<OwnedStr>>
    for PyBinaryArithmeticOperation
{
    fn from(value: BinaryArithmeticOperation<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyBinaryArithmeticOperation>
    for BinaryArithmeticOperation<OwnedStr>
{
    fn from(value: PyBinaryArithmeticOperation) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyBinaryArithmeticOperation {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for BinaryArithmeticOperation<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, operator_position={}, operator_fillers={})",
            PyBinaryArithmeticOperation::NAME,
            self.left.repr(py)?,
            self.right.repr(py)?,
            self.operator.repr(py)?,
            self.operator_position.repr(py)?,
            self.operator_fillers.repr(py)?,
        ))
    }
}

impl Repr for PyBinaryArithmeticOperation {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyBinaryArithmeticOperation,
    PyExpression
);
