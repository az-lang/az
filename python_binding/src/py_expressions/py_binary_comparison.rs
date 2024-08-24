use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::BinaryComparison;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_binary_comparison_operators::BinaryComparisonOperatorWrapper;
use crate::py_filler::{PyFiller, PyFillers};
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "BinaryComparison", extends = PyExpression,
    frozen
)]
pub(crate) struct PyBinaryComparison(BinaryComparison<OwnedString>);

#[pymethods]
impl PyBinaryComparison {
    #[new]
    #[pyo3(signature = (
    left, right, operator, /, *, operator_position, operator_fillers
    ))]
    fn new(
        left: OwnedExpressionWrapper,
        right: OwnedExpressionWrapper,
        operator: BinaryComparisonOperatorWrapper,
        operator_position: PySubstringPosition,
        operator_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(BinaryComparison {
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

    #[getter]
    fn left(&self) -> OwnedExpressionWrapper {
        self.0.left.as_ref().clone().into()
    }

    #[getter]
    fn operator(&self) -> BinaryComparisonOperatorWrapper {
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

    fn validate_contents(&self, py: Python<'_>) -> PyResult<()> {
        validate_contents(&self.0, py)
    }

    fn validate_positions(&self, py: Python<'_>) -> PyResult<()> {
        validate_positions(&self.0, py)
    }

    fn __getnewargs_ex__<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item(
            "operator_position",
            PySubstringPosition::from(self.0.operator_position.clone())
                .into_py(py),
        )?;
        kwargs.set_item(
            "operator_fillers",
            self.0
                .operator_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        Ok((
            PyTuple::new_bound(
                py,
                [
                    OwnedExpressionWrapper::from(self.0.left.as_ref().clone())
                        .into_py(py),
                    OwnedExpressionWrapper::from(
                        self.0.right.as_ref().clone(),
                    )
                    .into_py(py),
                    BinaryComparisonOperatorWrapper::from(self.0.operator)
                        .into_py(py),
                ],
            ),
            kwargs,
        ))
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

impl From<BinaryComparison<OwnedString>> for PyBinaryComparison {
    fn from(value: BinaryComparison<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyBinaryComparison> for BinaryComparison<OwnedString> {
    fn from(value: PyBinaryComparison) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyBinaryComparison {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for BinaryComparison<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, operator_position={}, operator_fillers={})",
            PyBinaryComparison::NAME,
            self.left.repr(py)?,
            self.right.repr(py)?,
            self.operator.repr(py)?,
            self.operator_position.repr(py)?,
            self.operator_fillers.repr(py)?
        ))
    }
}

impl Repr for PyBinaryComparison {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyBinaryComparison,
    PyExpression
);
