use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::MemberAccess;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;
use super::py_identifier::PyIdentifier;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "MemberAccess", extends = PyExpression,
    frozen
)]
pub(crate) struct PyMemberAccess(MemberAccess<OwnedStr>);

#[pymethods]
impl PyMemberAccess {
    #[getter]
    fn member(&self) -> PyIdentifier {
        self.0.member.clone().into()
    }

    #[getter]
    fn object(&self) -> OwnedExpressionWrapper {
        self.0.object.as_ref().clone().into()
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

    #[new]
    #[pyo3(signature = (object, member, /, *, operator_position, operator_fillers))]
    fn new(
        object: OwnedExpressionWrapper,
        member: PyIdentifier,
        operator_position: PySubstringPosition,
        operator_fillers: PyFillers,
    ) -> PyClassInitializer<PyMemberAccess> {
        PyExpression::new().add_subclass(Self(MemberAccess {
            object: Box::new(object.into()),
            member: member.into(),
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

impl From<MemberAccess<OwnedStr>> for PyMemberAccess {
    fn from(value: MemberAccess<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyMemberAccess> for MemberAccess<OwnedStr> {
    fn from(value: PyMemberAccess) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyMemberAccess {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for MemberAccess<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, operator_position={}, operator_fillers={})",
            PyMemberAccess::NAME,
            self.object.repr(py)?,
            self.member.repr(py)?,
            self.operator_position.repr(py)?,
            self.operator_fillers.repr(py)?
        ))
    }
}

impl Repr for PyMemberAccess {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyMemberAccess, PyExpression);
