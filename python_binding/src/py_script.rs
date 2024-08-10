use pyo3::pyclass::CompareOp;
use pyo3::types::{PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyResult, PyTypeInfo,
    Python,
};

use az::parsing::Script;
use az::tokenization::{TokenCollection, Tokenize};

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_filler::PyFiller;
use super::py_parsing_errors::OwnedParsingErrorWrapper;
use super::py_statements::OwnedStatementWrapper;
use super::py_token_collection::PyTokenCollection;
use super::traits::{Repr, RichCmp};
use super::types::OwnedString;
use super::validation::{validate_contents, validate_positions};

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.parsing", name = "Script")]
pub(crate) struct PyScript(Script<OwnedString>);

#[pymethods]
impl PyScript {
    #[classmethod]
    #[pyo3(signature = (tokens, /))]
    fn from_tokens(
        _cls: &Bound<'_, PyType>,
        tokens: PyTokenCollection,
    ) -> PyResult<Self> {
        Script::try_from(TokenCollection::from(tokens))
            .map(Into::into)
            .map_err(|error| OwnedParsingErrorWrapper::from(error).into())
    }

    #[new]
    #[pyo3(signature = (statements, /, *, fillers))]
    fn new(
        statements: Vec<OwnedStatementWrapper>,
        fillers: Vec<PyFiller>,
    ) -> Self {
        Self(Script {
            statements: statements.into_iter().map(Into::into).collect(),
            fillers: fillers.into_iter().map(Into::into).collect(),
        })
    }

    #[getter]
    fn fillers(&self) -> Vec<PyFiller> {
        self.0.fillers.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn statements(&self) -> Vec<OwnedStatementWrapper> {
        self.0.statements.iter().cloned().map(Into::into).collect()
    }

    fn format(&mut self) {
        self.0.format();
    }

    fn reset_positions(&mut self) {
        self.0.reset_positions();
    }

    fn tokenize(&self) -> PyTokenCollection {
        self.0.clone().tokenize().into()
    }

    fn validate_contents(&self, py: Python<'_>) -> PyResult<()> {
        validate_contents(&self.0, py)
    }

    fn validate_positions(&self, py: Python<'_>) -> PyResult<()> {
        validate_positions(&self.0, py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __getnewargs_ex__<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item(
            "fillers",
            self.0
                .fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        Ok((
            PyTuple::new_bound(
                py,
                [self
                    .0
                    .statements
                    .iter()
                    .cloned()
                    .map(|value| {
                        OwnedStatementWrapper::from(value).into_py(py)
                    })
                    .collect::<Vec<_>>()],
            ),
            kwargs,
        ))
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

impl From<Script<OwnedString>> for PyScript {
    fn from(value: Script<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyScript> for Script<OwnedString> {
    fn from(value: PyScript) -> Self {
        value.0
    }
}

impl Repr for Script<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, fillers={})",
            PyScript::NAME,
            self.statements.repr(py)?,
            self.fillers.repr(py)?
        ))
    }
}

impl Repr for PyScript {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyScript);
