use az::parsing::Call;
use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::{PyFiller, PyFillers};
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Call", extends = PyExpression, frozen
)]
pub(crate) struct PyCall(Call<OwnedString>);

#[pymethods]
impl PyCall {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (
    callable_, arguments, /, *, open_parenthesis_position, comma_positions,
    close_parenthesis_position, open_parenthesis_fillers, comma_fillers,
    close_parenthesis_fillers,
    ))]
    fn new(
        callable_: OwnedExpressionWrapper,
        arguments: Vec<OwnedExpressionWrapper>,
        open_parenthesis_position: PySubstringPosition,
        comma_positions: Vec<PySubstringPosition>,
        close_parenthesis_position: PySubstringPosition,
        open_parenthesis_fillers: PyFillers,
        comma_fillers: Vec<PyFillers>,
        close_parenthesis_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Call {
            callable: Box::new(callable_.into()),
            arguments: arguments.into_iter().map(Into::into).collect(),
            open_parenthesis_position: open_parenthesis_position.into(),
            comma_positions: comma_positions
                .into_iter()
                .map(Into::into)
                .collect(),
            close_parenthesis_position: close_parenthesis_position.into(),
            open_parenthesis_fillers: open_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            comma_fillers: comma_fillers
                .into_iter()
                .map(|comma_fillers| {
                    comma_fillers.into_iter().map(Into::into).collect()
                })
                .collect(),
            close_parenthesis_fillers: close_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    #[getter]
    fn arguments(&self) -> Vec<OwnedExpressionWrapper> {
        self.0.arguments.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn callable(&self) -> OwnedExpressionWrapper {
        self.0.callable.as_ref().clone().into()
    }

    #[getter]
    fn close_parenthesis_position(&self) -> PySubstringPosition {
        self.0.close_parenthesis_position.clone().into()
    }

    #[getter]
    fn close_parenthesis_fillers(&self) -> PyFillers {
        self.0
            .close_parenthesis_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn comma_fillers(&self) -> Vec<PyFillers> {
        self.0
            .comma_fillers
            .iter()
            .map(|comma_fillers| {
                comma_fillers.iter().cloned().map(Into::into).collect()
            })
            .collect()
    }

    #[getter]
    fn comma_positions(&self) -> Vec<PySubstringPosition> {
        self.0
            .comma_positions
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn open_parenthesis_position(&self) -> PySubstringPosition {
        self.0.open_parenthesis_position.clone().into()
    }

    #[getter]
    fn open_parenthesis_fillers(&self) -> PyFillers {
        self.0
            .open_parenthesis_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
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
            "open_parenthesis_position",
            PySubstringPosition::from(
                self.0.open_parenthesis_position.clone(),
            )
            .into_py(py),
        )?;
        kwargs.set_item(
            "comma_positions",
            self.0
                .comma_positions
                .iter()
                .cloned()
                .map(|value| PySubstringPosition::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        kwargs.set_item(
            "close_parenthesis_position",
            PySubstringPosition::from(
                self.0.close_parenthesis_position.clone(),
            )
            .into_py(py),
        )?;
        kwargs.set_item(
            "open_parenthesis_fillers",
            self.0
                .open_parenthesis_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        kwargs.set_item(
            "comma_fillers",
            self.0
                .comma_fillers
                .iter()
                .map(|value| {
                    value
                        .iter()
                        .cloned()
                        .map(|value| PyFiller::from(value).into_py(py))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )?;
        kwargs.set_item(
            "close_parenthesis_fillers",
            self.0
                .close_parenthesis_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        Ok((
            PyTuple::new_bound(
                py,
                &[
                    OwnedExpressionWrapper::from(
                        self.0.callable.as_ref().clone(),
                    )
                    .into_py(py),
                    self.0
                        .arguments
                        .iter()
                        .cloned()
                        .map(|value| {
                            OwnedExpressionWrapper::from(value).into_py(py)
                        })
                        .collect::<Vec<_>>()
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

impl From<Call<OwnedString>> for PyCall {
    fn from(value: Call<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyCall> for Call<OwnedString> {
    fn from(value: PyCall) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyCall {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Call<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, open_parenthesis_position={}, comma_positions={}, close_parenthesis_position={}, open_parenthesis_fillers={}, comma_fillers={}, close_parenthesis_fillers={})",
            PyCall::NAME,
            self.callable.repr(py)?,
            self.arguments.repr(py)?,
            self.open_parenthesis_position.repr(py)?,
            self.comma_positions.repr(py)?,
            self.close_parenthesis_position.repr(py)?,
            self.open_parenthesis_fillers.repr(py)?,
            self.comma_fillers.repr(py)?,
            self.close_parenthesis_fillers.repr(py)?
        ))
    }
}

impl Repr for PyCall {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyCall, PyExpression);
