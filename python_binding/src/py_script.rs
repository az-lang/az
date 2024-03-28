use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Script;
use az::tokenization::Tokenize;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_filler::PyFiller;
use super::py_parsing_errors::OwnedParsingErrorWrapper;
use super::py_statements::OwnedStatementWrapper;
use super::py_token::PyToken;
use super::traits::{Repr, RichCmp};
use super::types::OwnedStr;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.parsing", name = "Script")]
pub(crate) struct PyScript(Script<OwnedStr>);

#[pymethods]
impl PyScript {
    #[getter]
    fn fillers(&self) -> Vec<PyFiller> {
        self.0.fillers.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn statements(&self) -> Vec<OwnedStatementWrapper> {
        self.0.statements.iter().cloned().map(Into::into).collect()
    }

    #[classmethod]
    fn from_tokens(
        _cls: &Bound<'_, PyType>,
        tokens: Vec<PyToken>,
    ) -> PyResult<Self> {
        let tokens: Vec<_> = tokens.into_iter().map(Into::into).collect();
        Script::try_from(tokens)
            .map(Into::into)
            .map_err(|err| OwnedParsingErrorWrapper::from(err).into())
    }

    fn reset_positions(&mut self) {
        self.0.reset_positions();
    }

    fn tokenize(&self) -> Vec<PyToken> {
        Tokenize::tokenize(self.0.clone())
            .into_iter()
            .map(PyToken::from)
            .collect()
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

impl From<Script<OwnedStr>> for PyScript {
    fn from(value: Script<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyScript> for Script<OwnedStr> {
    fn from(value: PyScript) -> Self {
        value.0
    }
}

impl Repr for Script<OwnedStr> {
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
