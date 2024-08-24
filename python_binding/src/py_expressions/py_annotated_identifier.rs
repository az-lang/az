use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::AnnotatedIdentifier;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::{PyFiller, PyFillers};
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;
use super::py_identifier::PyIdentifier;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "AnnotatedIdentifier",
    extends = PyExpression, frozen
)]
pub(crate) struct PyAnnotatedIdentifier(AnnotatedIdentifier<OwnedString>);

#[pymethods]
impl PyAnnotatedIdentifier {
    #[new]
    #[pyo3(signature = (identifier, annotation, /, *, operator_position, operator_fillers))]
    fn new(
        identifier: PyIdentifier,
        annotation: OwnedExpressionWrapper,
        operator_position: PySubstringPosition,
        operator_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(AnnotatedIdentifier {
            identifier: identifier.into(),
            annotation: Box::new(annotation.into()),
            operator_position: operator_position.into(),
            operator_fillers: operator_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    #[getter]
    fn annotation(&self) -> OwnedExpressionWrapper {
        self.0.annotation.as_ref().clone().into()
    }

    #[getter]
    fn identifier(&self) -> PyIdentifier {
        self.0.identifier.clone().into()
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
                    PyIdentifier::from(self.0.identifier.clone()).into_py(py),
                    OwnedExpressionWrapper::from(
                        self.0.annotation.as_ref().clone(),
                    )
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

impl From<AnnotatedIdentifier<OwnedString>> for PyAnnotatedIdentifier {
    fn from(value: AnnotatedIdentifier<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyAnnotatedIdentifier> for AnnotatedIdentifier<OwnedString> {
    fn from(value: PyAnnotatedIdentifier) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyAnnotatedIdentifier {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for AnnotatedIdentifier<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, operator_position={}, operator_fillers={})",
            PyAnnotatedIdentifier::NAME,
            self.identifier.repr(py)?,
            self.annotation.repr(py)?,
            self.operator_position.repr(py)?,
            self.operator_fillers.repr(py)?,
        ))
    }
}

impl Repr for PyAnnotatedIdentifier {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyAnnotatedIdentifier,
    PyExpression
);
