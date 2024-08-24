use pyo3::exceptions::PyBaseExceptionGroup;
use pyo3::{PyErr, PyResult, Python};

use az::parsing::{
    AnnotatedIdentifier, Assignment, BidirectionalConditional,
    BinaryArithmeticOperation, BinaryComparison, Block, Call,
    ExpressionStatement, FunctionDefinition, FunctionType, Grouping,
    Identifier, MemberAccess, NumericLiteral, Return, Script, Tuple,
    UnaryArithmeticOperation, UnidirectionalConditional, WhileLoop,
};
use az::tokenization::{Token, TokenCollection};

use crate::types::{OwnedString, TokenOwnedString};

use super::py_validation_errors::{
    PyContentsValidationError, PyPositionsValidationError,
};
use super::traits::Repr;

pub(crate) trait ValidateContents {
    fn validate_contents(&self) -> Result<(), Vec<impl std::error::Error>>;
}

pub(crate) trait ValidatePositions {
    fn validate_positions(&self) -> Result<(), Vec<impl std::error::Error>>;
}

macro_rules! impl_validate_contents {
    ($expression_type:ty) => {
        impl ValidateContents for $expression_type {
            fn validate_contents(
                &self,
            ) -> Result<(), Vec<impl std::error::Error>> {
                Self::validate_contents(self)
            }
        }
    };
}

impl_validate_contents!(Token<TokenOwnedString>);

impl_validate_contents!(TokenCollection<TokenOwnedString>);

impl_validate_contents!(AnnotatedIdentifier<OwnedString>);
impl_validate_contents!(Assignment<OwnedString>);
impl_validate_contents!(BidirectionalConditional<OwnedString>);
impl_validate_contents!(BinaryArithmeticOperation<OwnedString>);
impl_validate_contents!(BinaryComparison<OwnedString>);
impl_validate_contents!(Block<OwnedString>);
impl_validate_contents!(Call<OwnedString>);
impl_validate_contents!(FunctionDefinition<OwnedString>);
impl_validate_contents!(FunctionType<OwnedString>);
impl_validate_contents!(Grouping<OwnedString>);
impl_validate_contents!(Identifier<OwnedString>);
impl_validate_contents!(MemberAccess<OwnedString>);
impl_validate_contents!(NumericLiteral<OwnedString>);
impl_validate_contents!(Return<OwnedString>);
impl_validate_contents!(Tuple<OwnedString>);
impl_validate_contents!(UnaryArithmeticOperation<OwnedString>);
impl_validate_contents!(UnidirectionalConditional<OwnedString>);
impl_validate_contents!(WhileLoop<OwnedString>);

impl_validate_contents!(ExpressionStatement<OwnedString>);

impl_validate_contents!(Script<OwnedString>);

macro_rules! impl_validate_positions {
    ($expression_type:ty) => {
        impl ValidatePositions for $expression_type {
            fn validate_positions(
                &self,
            ) -> Result<(), Vec<impl std::error::Error>> {
                Self::validate_positions(self)
            }
        }
    };
}

impl_validate_positions!(Token<TokenOwnedString>);

impl_validate_positions!(TokenCollection<TokenOwnedString>);

impl_validate_positions!(AnnotatedIdentifier<OwnedString>);
impl_validate_positions!(Assignment<OwnedString>);
impl_validate_positions!(BidirectionalConditional<OwnedString>);
impl_validate_positions!(BinaryArithmeticOperation<OwnedString>);
impl_validate_positions!(BinaryComparison<OwnedString>);
impl_validate_positions!(Block<OwnedString>);
impl_validate_positions!(Call<OwnedString>);
impl_validate_positions!(FunctionDefinition<OwnedString>);
impl_validate_positions!(FunctionType<OwnedString>);
impl_validate_positions!(Grouping<OwnedString>);
impl_validate_positions!(Identifier<OwnedString>);
impl_validate_positions!(MemberAccess<OwnedString>);
impl_validate_positions!(NumericLiteral<OwnedString>);
impl_validate_positions!(Return<OwnedString>);
impl_validate_positions!(Tuple<OwnedString>);
impl_validate_positions!(UnaryArithmeticOperation<OwnedString>);
impl_validate_positions!(UnidirectionalConditional<OwnedString>);
impl_validate_positions!(WhileLoop<OwnedString>);

impl_validate_positions!(ExpressionStatement<OwnedString>);

impl_validate_positions!(Script<OwnedString>);

pub(crate) fn validate_contents<T: Repr + ValidateContents>(
    value: &T,
    py: Python<'_>,
) -> PyResult<()> {
    value.validate_contents().or_else(|errors| {
        value.repr(py).and_then(|repr_string| {
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

pub(crate) fn validate_positions<T: Repr + ValidatePositions>(
    value: &T,
    py: Python<'_>,
) -> PyResult<()> {
    value.validate_positions().or_else(|errors| {
        value.repr(py).and_then(|repr_string| {
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
