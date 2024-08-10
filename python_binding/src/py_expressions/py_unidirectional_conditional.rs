use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::UnidirectionalConditional;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::{PyFiller, PyFillers};
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_block::PyBlock;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "UnidirectionalConditional",
    extends = PyExpression, frozen
)]
pub(crate) struct PyUnidirectionalConditional(
    UnidirectionalConditional<OwnedString>,
);

#[pymethods]
impl PyUnidirectionalConditional {
    #[new]
    #[pyo3(signature = (
    antecedent, consequent, /, *, opener_position, opener_fillers
    ))]
    fn new(
        antecedent: OwnedExpressionWrapper,
        consequent: PyBlock,
        opener_position: PySubstringPosition,
        opener_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(UnidirectionalConditional {
            antecedent: Box::new(antecedent.into()),
            consequent: consequent.into(),
            opener_position: opener_position.into(),
            opener_fillers: opener_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
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
            "opener_fillers",
            self.0
                .opener_fillers
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
                        self.0.antecedent.as_ref().clone(),
                    )
                    .into_py(py),
                    PyBlock::from(self.0.consequent.clone()).into_py(py),
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

impl From<UnidirectionalConditional<OwnedString>>
    for PyUnidirectionalConditional
{
    fn from(value: UnidirectionalConditional<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyUnidirectionalConditional>
    for UnidirectionalConditional<OwnedString>
{
    fn from(value: PyUnidirectionalConditional) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyUnidirectionalConditional {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for UnidirectionalConditional<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, opener_position={}, opener_fillers={})",
            PyUnidirectionalConditional::NAME,
            self.antecedent.repr(py)?,
            self.consequent.repr(py)?,
            self.opener_position.repr(py)?,
            self.opener_fillers.repr(py)?,
        ))
    }
}

impl Repr for PyUnidirectionalConditional {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyUnidirectionalConditional,
    PyExpression
);
