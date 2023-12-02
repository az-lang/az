use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Conditional;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_block::PyBlock;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Conditional", extends = PyExpression, frozen
)]
pub(crate) struct PyConditional(Conditional<OwnedStr>);

#[pymethods]
impl PyConditional {
    #[getter]
    fn alternative(&self) -> Option<OwnedExpressionWrapper> {
        self.0
            .alternative
            .as_ref()
            .map(|expression| expression.as_ref().clone().into())
    }

    #[getter]
    fn alternative_opener_fillers(&self) -> PyFillers {
        self.0
            .alternative_opener_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn alternative_opener_position(&self) -> Option<PySubstringPosition> {
        self.0
            .alternative_opener_position
            .as_ref()
            .map(|position| position.clone().into())
    }

    #[getter]
    fn antecedent(&self) -> OwnedExpressionWrapper {
        self.0.antecedent.as_ref().clone().into()
    }

    #[getter]
    fn consequent(&self) -> PyBlock {
        self.0.consequent.clone().into()
    }

    #[getter]
    fn opener_fillers(&self) -> PyFillers {
        self.0
            .opener_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn opener_position(&self) -> PySubstringPosition {
        self.0.opener_position.clone().into()
    }

    #[new]
    #[pyo3(signature = (
        antecedent, consequent, alternative, /, *, opener_position,
        alternative_opener_position, opener_fillers,
        alternative_opener_fillers,
    ))]
    fn new(
        antecedent: OwnedExpressionWrapper,
        consequent: PyBlock,
        alternative: Option<OwnedExpressionWrapper>,
        opener_position: PySubstringPosition,
        alternative_opener_position: Option<PySubstringPosition>,
        opener_fillers: PyFillers,
        alternative_opener_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Conditional {
            antecedent: Box::new(antecedent.into()),
            consequent: consequent.into(),
            alternative: alternative.map(Into::into).map(Box::new),
            opener_position: opener_position.into(),
            alternative_opener_position: alternative_opener_position
                .map(Into::into),
            opener_fillers: opener_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            alternative_opener_fillers: alternative_opener_fillers
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

impl From<Conditional<OwnedStr>> for PyConditional {
    fn from(value: Conditional<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyConditional> for Conditional<OwnedStr> {
    fn from(value: PyConditional) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyConditional {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Conditional<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, opener_position={}, alternative_opener_position={}, opener_fillers={}, alternative_opener_fillers={})",
            PyConditional::NAME,
            self.antecedent.repr(py)?,
            self.consequent.repr(py)?,
            self.alternative.repr(py)?,
            self.opener_position.repr(py)?,
            self.alternative_opener_position.repr(py)?,
            self.opener_fillers.repr(py)?,
            self.alternative_opener_fillers.repr(py)?
        ))
    }
}

impl Repr for PyConditional {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyConditional, PyExpression);
