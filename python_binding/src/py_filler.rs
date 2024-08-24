use pyo3::exceptions::PyBaseExceptionGroup;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, PyAny, PyErr, PyObject, PyResult,
    PyTypeInfo, Python,
};

use az::parsing::Filler;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_filler_content::PyFillerContent;
use super::py_substring_position::PySubstringPosition;
use super::py_validation_errors::{
    PyContentsValidationError, PyPositionsValidationError,
};
use super::traits::{Repr, RichCmp};
use super::types::OwnedString;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.parsing", name = "Filler", frozen)]
pub(crate) struct PyFiller(Filler<OwnedString>);

#[pymethods]
impl PyFiller {
    #[getter]
    fn content(&self) -> PyFillerContent {
        self.0.content.clone().into()
    }

    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[new]
    #[pyo3(signature = (content, /, *, position))]
    fn new(content: PyFillerContent, position: PySubstringPosition) -> Self {
        Self(Filler {
            content: content.into(),
            position: position.into(),
        })
    }

    fn validate_contents(&self, py: Python<'_>) -> PyResult<()> {
        self.0.validate_contents().or_else(|errors| {
            self.repr(py).and_then(|repr_string| {
                Err(PyBaseExceptionGroup::new_err((
                    format!("Failed contents validation: {}.", repr_string),
                    errors
                        .into_iter()
                        .map(|error| {
                            PyErr::new::<PyContentsValidationError, _>((
                                error.to_string(),
                            ))
                        })
                        .collect::<Vec<_>>(),
                )))
            })
        })
    }

    fn validate_positions(&self, py: Python<'_>) -> PyResult<()> {
        self.0.validate_positions().or_else(|errors| {
            self.repr(py).and_then(|repr_string| {
                Err(PyBaseExceptionGroup::new_err((
                    format!("Failed positions validation: {}.", repr_string),
                    errors
                        .into_iter()
                        .map(|error| {
                            PyErr::new::<PyPositionsValidationError, _>((
                                error.to_string(),
                            ))
                        })
                        .collect::<Vec<_>>(),
                )))
            })
        })
    }

    fn __getnewargs_ex__<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item(
            "position",
            PySubstringPosition::from(self.0.position.clone()).into_py(py),
        )?;
        Ok((PyTuple::new_bound(py, [self.content().into_py(py)]), kwargs))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyFiller);

pub(crate) type PyFillers = Vec<PyFiller>;

impl From<Filler<OwnedString>> for PyFiller {
    fn from(value: Filler<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyFiller> for Filler<OwnedString> {
    fn from(value: PyFiller) -> Self {
        value.0
    }
}

impl Repr for Filler<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, position={})",
            PyFiller::NAME,
            self.content.repr(py)?,
            self.position.repr(py)?
        ))
    }
}

impl Repr for PyFiller {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
