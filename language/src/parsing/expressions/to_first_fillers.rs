use crate::parsing::filler::Filler;

use super::annotated_identifier::AnnotatedIdentifier;
use super::assignment::Assignment;
use super::bidirectional_conditional::BidirectionalConditional;
use super::binary_arithmetic_operation::BinaryArithmeticOperation;
use super::binary_comparison::BinaryComparison;
use super::block::Block;
use super::call::Call;
use super::expression::Expression;
use super::function_definition::FunctionDefinition;
use super::function_type::FunctionType;
use super::grouping::Grouping;
use super::identifier::Identifier;
use super::member_access::MemberAccess;
use super::numeric_literal::NumericLiteral;
use super::return_::Return;
use super::tuple::Tuple;
use super::unary_arithmetic_operation::UnaryArithmeticOperation;
use super::unidirectional_conditional::UnidirectionalConditional;
use super::while_loop::WhileLoop;

pub(crate) trait ToFirstFillers<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>];
}

impl<StringType> ToFirstFillers<StringType>
    for AnnotatedIdentifier<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.identifier.to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType> for Assignment<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.target.as_ref().to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType>
    for BidirectionalConditional<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.antecedent_opener_fillers
    }
}

impl<StringType> ToFirstFillers<StringType>
    for BinaryArithmeticOperation<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.left.as_ref().to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType> for BinaryComparison<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.left.as_ref().to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType> for Block<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.open_brace_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for Call<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.callable.as_ref().to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType>
    for FunctionDefinition<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.opener_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for FunctionType<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.open_parenthesis_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for Expression<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        match self {
            Expression::AnnotatedIdentifier(value) => value.to_first_fillers(),
            Expression::Assignment(value) => value.to_first_fillers(),
            Expression::BidirectionalConditional(value) => {
                value.to_first_fillers()
            }
            Expression::BinaryArithmeticOperation(value) => {
                value.to_first_fillers()
            }
            Expression::BinaryComparison(value) => value.to_first_fillers(),
            Expression::Block(value) => value.to_first_fillers(),
            Expression::Call(value) => value.to_first_fillers(),
            Expression::FunctionDefinition(value) => value.to_first_fillers(),
            Expression::FunctionType(value) => value.to_first_fillers(),
            Expression::Grouping(value) => value.to_first_fillers(),
            Expression::Identifier(value) => value.to_first_fillers(),
            Expression::MemberAccess(value) => value.to_first_fillers(),
            Expression::NumericLiteral(value) => value.to_first_fillers(),
            Expression::Return(value) => value.to_first_fillers(),
            Expression::Tuple(value) => value.to_first_fillers(),
            Expression::UnaryArithmeticOperation(value) => {
                value.to_first_fillers()
            }
            Expression::UnidirectionalConditional(value) => {
                value.to_first_fillers()
            }
            Expression::WhileLoop(value) => value.to_first_fillers(),
        }
    }
}

impl<StringType> ToFirstFillers<StringType> for Grouping<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.open_parenthesis_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for Identifier<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for MemberAccess<StringType>
where
    Expression<StringType>: ToFirstFillers<StringType>,
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        self.object.as_ref().to_first_fillers()
    }
}

impl<StringType> ToFirstFillers<StringType> for NumericLiteral<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for Return<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.operator_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for Tuple<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.open_parenthesis_fillers
    }
}

impl<StringType> ToFirstFillers<StringType>
    for UnaryArithmeticOperation<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.operator_fillers
    }
}

impl<StringType> ToFirstFillers<StringType>
    for UnidirectionalConditional<StringType>
{
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.opener_fillers
    }
}

impl<StringType> ToFirstFillers<StringType> for WhileLoop<StringType> {
    fn to_first_fillers(&self) -> &[Filler<StringType>] {
        &self.opener_fillers
    }
}
