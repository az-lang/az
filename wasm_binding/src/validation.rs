use wasm_bindgen::JsError;

use az::parsing::{
    AnnotatedIdentifier, Assignment, BidirectionalConditional,
    BinaryArithmeticOperation, BinaryComparison, Block, Call,
    ExpressionStatement, Filler, FunctionDefinition, FunctionType, Grouping,
    Identifier, MemberAccess, NumericLiteral, Return, Script, Tuple,
    UnaryArithmeticOperation, UnidirectionalConditional, WhileLoop,
};
use az::tokenization::{Token, TokenCollection};

use super::traits::TryToJsString;
use super::types::{OwnedString, TokenOwnedString};

pub(crate) trait ValidateContents {
    fn validate_contents(&self) -> Result<(), Vec<impl std::error::Error>>;
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
impl_validate_contents!(Filler<OwnedString>);
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

pub(crate) trait ValidatePositions {
    fn validate_positions(&self) -> Result<(), Vec<impl std::error::Error>>;
}

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
impl_validate_positions!(Filler<OwnedString>);
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

pub(crate) fn validate_contents<T: TryToJsString + ValidateContents>(
    value: &T,
) -> Result<(), <T as TryToJsString>::Error>
where
    <T as TryToJsString>::Error: From<JsError>,
{
    value.validate_contents().or_else(|errors| {
        const ERRORS_SEPARATOR: &str = "\n    ";
        value.try_to_js_string().and_then(|string| {
            Err(JsError::new(&format!(
                "Failed contents validation {}:{}{}",
                string,
                ERRORS_SEPARATOR,
                errors
                    .into_iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<_>>()
                    .join(ERRORS_SEPARATOR)
            ))
            .into())
        })
    })
}

pub(crate) fn validate_positions<T: TryToJsString + ValidatePositions>(
    value: &T,
) -> Result<(), <T as TryToJsString>::Error>
where
    <T as TryToJsString>::Error: From<JsError>,
{
    value.validate_positions().or_else(|errors| {
        const ERRORS_SEPARATOR: &str = "\n    ";
        value.try_to_js_string().and_then(|string| {
            Err(JsError::new(&format!(
                "Failed positions validation {}:{}{}",
                string,
                ERRORS_SEPARATOR,
                errors
                    .into_iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<_>>()
                    .join(ERRORS_SEPARATOR)
            ))
            .into())
        })
    })
}
