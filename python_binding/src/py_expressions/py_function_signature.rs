use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::FunctionSignature;

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
    module = "az.parsing", name = "FunctionSignature", extends = PyExpression,
    frozen
)]
pub(crate) struct PyFunctionSignature(FunctionSignature<OwnedString>);

#[pymethods]
impl PyFunctionSignature {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (
    parameters, return_type, /, *, opener_position, open_parenthesis_position,
    comma_positions, close_parenthesis_position, arrow_position,
    opener_fillers, open_parenthesis_fillers, comma_fillers,
    close_parenthesis_fillers, arrow_fillers,
    ))]
    fn new(
        parameters: Vec<OwnedExpressionWrapper>,
        return_type: OwnedExpressionWrapper,
        opener_position: PySubstringPosition,
        open_parenthesis_position: PySubstringPosition,
        comma_positions: Vec<PySubstringPosition>,
        close_parenthesis_position: PySubstringPosition,
        arrow_position: PySubstringPosition,
        opener_fillers: PyFillers,
        open_parenthesis_fillers: PyFillers,
        comma_fillers: Vec<PyFillers>,
        close_parenthesis_fillers: PyFillers,
        arrow_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(FunctionSignature {
            parameters: parameters.into_iter().map(Into::into).collect(),
            return_type: Box::new(return_type.into()),
            opener_position: opener_position.into(),
            open_parenthesis_position: open_parenthesis_position.into(),
            comma_positions: comma_positions
                .into_iter()
                .map(Into::into)
                .collect(),
            close_parenthesis_position: close_parenthesis_position.into(),
            arrow_position: arrow_position.into(),
            opener_fillers: opener_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            open_parenthesis_fillers: open_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            comma_fillers: comma_fillers
                .into_iter()
                .map(|fillers| fillers.into_iter().map(Into::into).collect())
                .collect(),
            close_parenthesis_fillers: close_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            arrow_fillers: arrow_fillers.into_iter().map(Into::into).collect(),
        }))
    }

    #[getter]
    fn arrow_fillers(&self) -> PyFillers {
        self.0
            .arrow_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn arrow_position(&self) -> PySubstringPosition {
        self.0.arrow_position.clone().into()
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
    fn close_parenthesis_position(&self) -> PySubstringPosition {
        self.0.close_parenthesis_position.clone().into()
    }

    #[getter]
    fn comma_fillers(&self) -> Vec<PyFillers> {
        self.0
            .comma_fillers
            .iter()
            .map(|fillers| fillers.iter().cloned().map(Into::into).collect())
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
    fn open_parenthesis_fillers(&self) -> PyFillers {
        self.0
            .open_parenthesis_fillers
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

    #[getter]
    fn parameters(&self) -> Vec<OwnedExpressionWrapper> {
        self.0.parameters.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn return_type(&self) -> OwnedExpressionWrapper {
        self.0.return_type.as_ref().clone().into()
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
            "opener_position",
            PySubstringPosition::from(self.0.opener_position.clone())
                .into_py(py),
        )?;
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
            "arrow_position",
            PySubstringPosition::from(self.0.arrow_position.clone())
                .into_py(py),
        )?;
        kwargs.set_item(
            "opener_fillers",
            self.0
                .opener_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
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
        kwargs.set_item(
            "arrow_fillers",
            self.0
                .arrow_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        Ok((
            PyTuple::new_bound(
                py,
                &[
                    self.0
                        .parameters
                        .iter()
                        .cloned()
                        .map(|value| {
                            OwnedExpressionWrapper::from(value).into_py(py)
                        })
                        .collect::<Vec<_>>()
                        .into_py(py),
                    OwnedExpressionWrapper::from(
                        self.0.return_type.as_ref().clone(),
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

impl From<FunctionSignature<OwnedString>> for PyFunctionSignature {
    fn from(value: FunctionSignature<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyFunctionSignature> for FunctionSignature<OwnedString> {
    fn from(value: PyFunctionSignature) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyFunctionSignature {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for FunctionSignature<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, opener_position={}, open_parenthesis_position={}, comma_positions={}, close_parenthesis_position={}, arrow_position={}, opener_fillers={}, open_parenthesis_fillers={}, comma_fillers={}, close_parenthesis_fillers={}, arrow_fillers={})",
            PyFunctionSignature::NAME,
            self.parameters.repr(py)?,
            self.return_type.repr(py)?,
            self.opener_position.repr(py)?,
            self.open_parenthesis_position.repr(py)?,
            self.comma_positions.repr(py)?,
            self.close_parenthesis_position.repr(py)?,
            self.arrow_position.repr(py)?,
            self.opener_fillers.repr(py)?,
            self.open_parenthesis_fillers.repr(py)?,
            self.comma_fillers.repr(py)?,
            self.close_parenthesis_fillers.repr(py)?,
            self.arrow_fillers.repr(py)?
        ))
    }
}

impl Repr for PyFunctionSignature {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyFunctionSignature,
    PyExpression
);
