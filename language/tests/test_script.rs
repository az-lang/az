use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::sync::Arc;

use proptest::prelude::{prop, Strategy};
use proptest::strategy::Union;
use proptest::{prop_oneof, proptest};
use stacker::grow;

use az::parsing::{
    AnnotatedIdentifier, AnnotationOperator, Assignment, AssignmentOperator,
    Associativity, BidirectionalConditional, BinaryArithmeticOperation,
    BinaryArithmeticOperator, BinaryComparison, BinaryComparisonOperator,
    Block, Call, CallOperator, Expression, ExpressionStatement, Filler,
    FillerContent, FunctionDefinition, FunctionType, FunctionTypeOperator,
    Grouping, Identifier, MemberAccess, MemberAccessOperator, NumericLiteral,
    Precedence, Return, ReturnOperator, Script, Statement, Tuple,
    UnaryArithmeticOperation, UnaryArithmeticOperator,
    UnidirectionalConditional, WhileLoop,
};
use az::tokenization::{
    ByteCount, CharacterPosition, NumericLiteralType, SubstringPosition,
    TokenCollection, Tokenize, Utf8Count,
};
use common::patterns::{
    to_floating_point_value_pattern, to_integer_value_pattern,
};
use common::strategy_factories::{
    to_comment_block_string_strategy, to_comment_line_string_strategy,
    to_identifier_string_strategy, to_whitespace_string_strategy,
};

mod common;

const MAX_EXPRESSIONS_SIZE: usize = 4usize;
const MAX_FILLERS_SIZE: usize = 4usize;
const MAX_EXPRESSION_DEPTH: usize = 5usize;
const MAX_STATEMENTS_SIZE: usize = 4usize;

type Fillers<StringType> = Vec<Filler<StringType>>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum InternalNodeExpressionKind {
    AnnotatedIdentifier(AnnotationOperator),
    Assignment(AssignmentOperator),
    BidirectionalConditional,
    BinaryArithmeticOperation(BinaryArithmeticOperator),
    BinaryComparison(BinaryComparisonOperator),
    Block,
    Call(CallOperator),
    FunctionDefinition(
        // function parameters are annotated identifiers
        AnnotationOperator,
    ),
    FunctionType(FunctionTypeOperator),
    Grouping,
    MemberAccess(MemberAccessOperator),
    Return(ReturnOperator),
    Tuple,
    UnaryArithmeticOperation(UnaryArithmeticOperator),
    UnidirectionalConditional,
    WhileLoop,
}

/// This function is not used,
/// its purpose is to make sure that all expression variants are covered.
fn _from<StringType: Debug>(
    value: Expression<StringType>,
) -> InternalNodeExpressionKind {
    match value {
        Expression::AnnotatedIdentifier(_) => {
            InternalNodeExpressionKind::AnnotatedIdentifier(AnnotationOperator)
        }
        Expression::Assignment(_) => {
            InternalNodeExpressionKind::Assignment(AssignmentOperator)
        }
        Expression::BidirectionalConditional(_) => {
            InternalNodeExpressionKind::BidirectionalConditional
        }
        Expression::BinaryArithmeticOperation(_) => {
            InternalNodeExpressionKind::BinaryArithmeticOperation(
                BinaryArithmeticOperator::Addition,
            )
        }
        Expression::BinaryComparison(_) => {
            InternalNodeExpressionKind::BinaryComparison(
                BinaryComparisonOperator::EqualTo,
            )
        }
        Expression::Block(_) => InternalNodeExpressionKind::Block,
        Expression::Call(_) => InternalNodeExpressionKind::Call(CallOperator),
        Expression::FunctionDefinition(_) => {
            InternalNodeExpressionKind::FunctionDefinition(AnnotationOperator)
        }
        Expression::FunctionType(_) => {
            InternalNodeExpressionKind::FunctionType(FunctionTypeOperator)
        }
        Expression::Grouping(_) => InternalNodeExpressionKind::Grouping,
        Expression::Identifier(value) => {
            unimplemented!("{:?} is a leaf expression", value)
        }
        Expression::MemberAccess(_) => {
            InternalNodeExpressionKind::MemberAccess(MemberAccessOperator)
        }
        Expression::NumericLiteral(value) => {
            unimplemented!("{:?} is a leaf expression", value)
        }
        Expression::Return(_) => {
            InternalNodeExpressionKind::Return(ReturnOperator)
        }
        Expression::Tuple(_) => InternalNodeExpressionKind::Tuple,
        Expression::UnaryArithmeticOperation(_) => {
            InternalNodeExpressionKind::UnaryArithmeticOperation(
                UnaryArithmeticOperator::Negation,
            )
        }
        Expression::UnidirectionalConditional(_) => {
            InternalNodeExpressionKind::UnidirectionalConditional
        }
        Expression::WhileLoop(_) => InternalNodeExpressionKind::WhileLoop,
    }
}

const INTERNAL_NODE_EXPRESSION_KINDS: [InternalNodeExpressionKind; 24] = [
    InternalNodeExpressionKind::AnnotatedIdentifier(AnnotationOperator),
    InternalNodeExpressionKind::Assignment(AssignmentOperator),
    InternalNodeExpressionKind::BidirectionalConditional,
    InternalNodeExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Addition,
    ),
    InternalNodeExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Division,
    ),
    InternalNodeExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Multiplication,
    ),
    InternalNodeExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Subtraction,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::EqualTo,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::GreaterThan,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::GreaterThanOrEqualTo,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::LessThan,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::LessThanOrEqualTo,
    ),
    InternalNodeExpressionKind::BinaryComparison(
        BinaryComparisonOperator::NotEqualTo,
    ),
    InternalNodeExpressionKind::Block,
    InternalNodeExpressionKind::Call(CallOperator),
    InternalNodeExpressionKind::FunctionDefinition(AnnotationOperator),
    InternalNodeExpressionKind::FunctionType(FunctionTypeOperator),
    InternalNodeExpressionKind::Grouping,
    InternalNodeExpressionKind::MemberAccess(MemberAccessOperator),
    InternalNodeExpressionKind::Return(ReturnOperator),
    InternalNodeExpressionKind::Tuple,
    InternalNodeExpressionKind::UnaryArithmeticOperation(
        UnaryArithmeticOperator::Negation,
    ),
    InternalNodeExpressionKind::UnidirectionalConditional,
    InternalNodeExpressionKind::WhileLoop,
];

#[derive(Clone, Eq, PartialEq)]
struct OptionalPrecedence(Option<Precedence>);

impl PartialOrd for OptionalPrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Some(left), Some(right)) => Some(left.cmp(right)),
            _ => None,
        }
    }
}

impl From<&InternalNodeExpressionKind> for OptionalPrecedence {
    fn from(value: &InternalNodeExpressionKind) -> Self {
        match value {
            InternalNodeExpressionKind::AnnotatedIdentifier(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::Assignment(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::BinaryArithmeticOperation(
                operator,
            ) => OptionalPrecedence(Some(Precedence::from(*operator))),
            InternalNodeExpressionKind::BinaryComparison(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::Call(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::FunctionDefinition(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::FunctionType(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::MemberAccess(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::Return(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::UnaryArithmeticOperation(operator) => {
                OptionalPrecedence(Some(Precedence::from(*operator)))
            }
            InternalNodeExpressionKind::BidirectionalConditional
            | InternalNodeExpressionKind::Block
            | InternalNodeExpressionKind::Grouping
            | InternalNodeExpressionKind::Tuple
            | InternalNodeExpressionKind::UnidirectionalConditional
            | InternalNodeExpressionKind::WhileLoop => {
                OptionalPrecedence(None)
            }
        }
    }
}

impl PartialOrd for InternalNodeExpressionKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        OptionalPrecedence::from(self)
            .partial_cmp(&OptionalPrecedence::from(other))
    }
}

trait ToNonLexicallyConflicting<StringType> {
    fn to_non_lexically_conflicting(
        self,
        non_comment_filler: Filler<StringType>,
    ) -> Self;
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for AnnotatedIdentifier<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.identifier = self
            .identifier
            .to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for Assignment<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.target =
            self.target.to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for BinaryArithmeticOperation<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.left = self.left.to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for BinaryComparison<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.left = self.left.to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType> for Block<StringType> {
    fn to_non_lexically_conflicting(
        self,
        _non_comment_filler: Filler<StringType>,
    ) -> Self {
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for Box<Expression<StringType>>
{
    fn to_non_lexically_conflicting(
        self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        Self::new((*self).to_non_lexically_conflicting(non_comment_filler))
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType> for Call<StringType> {
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.callable = self
            .callable
            .to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for BidirectionalConditional<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.antecedent_opener_fillers.is_empty() {
            self.antecedent_opener_fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for Expression<StringType>
{
    fn to_non_lexically_conflicting(
        self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                Expression::AnnotatedIdentifier(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::Assignment(value) => Expression::Assignment(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::BidirectionalConditional(value) => {
                Expression::BidirectionalConditional(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::BinaryArithmeticOperation(value) => {
                Expression::BinaryArithmeticOperation(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::BinaryComparison(value) => {
                Expression::BinaryComparison(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::Block(value) => Expression::Block(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::Call(value) => Expression::Call(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::FunctionDefinition(value) => {
                Expression::FunctionDefinition(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::FunctionType(value) => Expression::FunctionType(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::Grouping(value) => Expression::Grouping(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::Identifier(value) => Expression::Identifier(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::MemberAccess(value) => Expression::MemberAccess(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::NumericLiteral(value) => Expression::NumericLiteral(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::Return(value) => Expression::Return(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::Tuple(value) => Expression::Tuple(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
            Expression::UnaryArithmeticOperation(value) => {
                Expression::UnaryArithmeticOperation(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::UnidirectionalConditional(value) => {
                Expression::UnidirectionalConditional(
                    value.to_non_lexically_conflicting(non_comment_filler),
                )
            }
            Expression::WhileLoop(value) => Expression::WhileLoop(
                value.to_non_lexically_conflicting(non_comment_filler),
            ),
        }
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for FunctionDefinition<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.opener_fillers.is_empty() {
            self.opener_fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for FunctionType<StringType>
{
    fn to_non_lexically_conflicting(
        self,
        _non_comment_filler: Filler<StringType>,
    ) -> Self {
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for Grouping<StringType>
{
    fn to_non_lexically_conflicting(
        self,
        _non_comment_filler: Filler<StringType>,
    ) -> Self {
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for Identifier<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.fillers.is_empty() {
            self.fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for MemberAccess<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.object =
            self.object.to_non_lexically_conflicting(non_comment_filler);
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for NumericLiteral<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.fillers.is_empty() {
            self.fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType> for Return<StringType> {
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.operator_fillers.is_empty() {
            self.operator_fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType> for Tuple<StringType> {
    fn to_non_lexically_conflicting(
        self,
        _non_comment_filler: Filler<StringType>,
    ) -> Self {
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for UnaryArithmeticOperation<StringType>
{
    fn to_non_lexically_conflicting(
        self,
        _non_comment_filler: Filler<StringType>,
    ) -> Self {
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for UnidirectionalConditional<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.opener_fillers.is_empty() {
            self.opener_fillers.push(non_comment_filler)
        }
        self
    }
}

impl<StringType> ToNonLexicallyConflicting<StringType>
    for WhileLoop<StringType>
{
    fn to_non_lexically_conflicting(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        if self.opener_fillers.is_empty() {
            self.opener_fillers.push(non_comment_filler)
        }
        self
    }
}

trait ToValidDivisor<StringType> {
    fn to_valid_divisor(self, non_comment_filler: Filler<StringType>) -> Self;
}

impl<StringType> ToValidDivisor<StringType>
    for AnnotatedIdentifier<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.identifier = self.identifier.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Assignment<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.target = self.target.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType>
    for BinaryArithmeticOperation<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.left = self.left.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType>
    for BidirectionalConditional<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(
            &mut self.antecedent_opener_fillers,
            non_comment_filler,
        );
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for BinaryComparison<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.left = self.left.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Block<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.open_brace_fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Box<Expression<StringType>> {
    fn to_valid_divisor(self, non_comment_filler: Filler<StringType>) -> Self {
        Self::new((*self).to_valid_divisor(non_comment_filler))
    }
}

impl<StringType> ToValidDivisor<StringType> for Call<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.callable = self.callable.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Expression<StringType> {
    fn to_valid_divisor(self, non_comment_filler: Filler<StringType>) -> Self {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                Expression::AnnotatedIdentifier(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::Assignment(value) => Expression::Assignment(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::BidirectionalConditional(value) => {
                Expression::BidirectionalConditional(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::BinaryArithmeticOperation(value) => {
                Expression::BinaryArithmeticOperation(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::BinaryComparison(value) => {
                Expression::BinaryComparison(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::Block(value) => {
                Expression::Block(value.to_valid_divisor(non_comment_filler))
            }
            Expression::Call(value) => {
                Expression::Call(value.to_valid_divisor(non_comment_filler))
            }
            Expression::FunctionDefinition(value) => {
                Expression::FunctionDefinition(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::FunctionType(value) => Expression::FunctionType(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::Grouping(value) => Expression::Grouping(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::Identifier(value) => Expression::Identifier(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::MemberAccess(value) => Expression::MemberAccess(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::NumericLiteral(value) => Expression::NumericLiteral(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::Return(value) => {
                Expression::Return(value.to_valid_divisor(non_comment_filler))
            }
            Expression::Tuple(value) => {
                Expression::Tuple(value.to_valid_divisor(non_comment_filler))
            }
            Expression::UnaryArithmeticOperation(value) => {
                Expression::UnaryArithmeticOperation(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::UnidirectionalConditional(value) => {
                Expression::UnidirectionalConditional(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
            Expression::WhileLoop(value) => Expression::WhileLoop(
                value.to_valid_divisor(non_comment_filler),
            ),
        }
    }
}

impl<StringType> ToValidDivisor<StringType>
    for FunctionDefinition<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.opener_fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for FunctionType<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(
            &mut self.open_parenthesis_fillers,
            non_comment_filler,
        );
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Grouping<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(
            &mut self.open_parenthesis_fillers,
            non_comment_filler,
        );
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Identifier<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for MemberAccess<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        self.object = self.object.to_valid_divisor(non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for NumericLiteral<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Return<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.operator_fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for Tuple<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(
            &mut self.open_parenthesis_fillers,
            non_comment_filler,
        );
        self
    }
}

impl<StringType> ToValidDivisor<StringType>
    for UnaryArithmeticOperation<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.operator_fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType>
    for UnidirectionalConditional<StringType>
{
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.opener_fillers, non_comment_filler);
        self
    }
}

impl<StringType> ToValidDivisor<StringType> for WhileLoop<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.opener_fillers, non_comment_filler);
        self
    }
}

fn fill_divisor_fillers<StringType>(
    fillers: &mut Fillers<StringType>,
    non_comment_filler: Filler<StringType>,
) {
    let is_comment_filler = |filler: &Filler<StringType>| {
        matches!(
            filler.content,
            FillerContent::CommentBlock(_) | FillerContent::CommentLine(_)
        )
    };
    assert!(!is_comment_filler(&non_comment_filler));
    if !fillers.is_empty() && is_comment_filler(&fillers[0]) {
        fillers.insert(0, non_comment_filler);
    }
}

fn to_expression_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    depth_range: RangeInclusive<usize>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Expression<StringType>> {
    let base = prop_oneof![
        to_identifier_strategy().prop_map(Expression::Identifier),
        to_numeric_literal_strategy().prop_map(Expression::NumericLiteral),
    ];
    let (min_depth, max_depth) = depth_range.into_inner();
    to_internal_node_expression_kinds_strategy(min_depth..=max_depth)
    .prop_flat_map(move |kind_groups| {
        let operation_groups_count = kind_groups.iter().filter(|group| matches!(OptionalPrecedence::from(&group[0]), OptionalPrecedence(Some(_)))).count();
        let operation_max_depth = if operation_groups_count == 0usize {
            0usize
        } else {
            max_depth.saturating_sub(kind_groups.len()) / operation_groups_count
        };
        kind_groups.into_iter().fold(
            base.clone().boxed(),
            |expression_strategy, kind_group| {
                {
                    Union::new(kind_group.into_iter().map(|kind| {
                        let expression_strategy = expression_strategy.clone();
                        match kind {
                            InternalNodeExpressionKind::AnnotatedIdentifier(_) => {
                                to_annotated_identifier_strategy(
                                    expression_strategy, operation_max_depth
                                )
                                .prop_map(Expression::AnnotatedIdentifier)
                                .boxed()
                            }
                            InternalNodeExpressionKind::Assignment(_) => {
                                to_assignment_strategy(expression_strategy, operation_max_depth)
                                    .prop_map(Expression::Assignment)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::BidirectionalConditional => {
                                to_bidirectional_conditional_strategy(
                                    expression_strategy,
                                    statement_count_range.clone(),
                                )
                                    .prop_map(Expression::BidirectionalConditional)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::BinaryArithmeticOperation(
                                operator,
                            ) => to_binary_arithmetic_operation_strategy(
                                operator, expression_strategy, operation_max_depth
                            )
                                .prop_map(Expression::BinaryArithmeticOperation)
                                .boxed(),
                            InternalNodeExpressionKind::BinaryComparison(operator) => to_binary_comparison_strategy(
                                operator, expression_strategy, operation_max_depth
                            )
                                .prop_map(Expression::BinaryComparison)
                                .boxed(),
                            InternalNodeExpressionKind::Block => to_block_strategy(
                                expression_strategy,
                                statement_count_range.clone(),
                            )
                            .prop_map(Expression::Block)
                            .boxed(),
                            InternalNodeExpressionKind::Call(_) => {
                                to_call_strategy(expression_strategy, operation_max_depth)
                                    .prop_map(Expression::Call)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::FunctionDefinition(_) => {
                                to_function_definition_strategy(
                                    expression_strategy,
                                    statement_count_range.clone(),
                                    operation_max_depth,
                                )
                                .prop_map(Expression::FunctionDefinition)
                                .boxed()
                            }
                            InternalNodeExpressionKind::FunctionType(_) => {
                                to_function_type_strategy(expression_strategy)
                                .prop_map(Expression::FunctionType)
                                .boxed()
                            }
                            InternalNodeExpressionKind::Grouping => {
                                to_grouping_strategy(expression_strategy)
                                    .prop_map(Expression::Grouping)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::MemberAccess(_) => {
                                to_member_access_strategy(expression_strategy, operation_max_depth)
                                    .prop_map(Expression::MemberAccess)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::Return(_) => {
                                to_return_strategy(expression_strategy, operation_max_depth)
                                    .prop_map(Expression::Return)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::Tuple => {
                                to_tuple_strategy(expression_strategy)
                                    .prop_map(Expression::Tuple)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::UnaryArithmeticOperation(
                                operator,
                            ) => to_unary_arithmetic_operation_strategy(
                                expression_strategy,
                                operator,
                                operation_max_depth,
                            )
                            .prop_map(Expression::UnaryArithmeticOperation)
                            .boxed(),
                            InternalNodeExpressionKind::UnidirectionalConditional => {
                                to_unidirectional_conditional_strategy(
                                    expression_strategy,
                                    statement_count_range.clone(),
                                )
                                    .prop_map(Expression::UnidirectionalConditional)
                                    .boxed()
                            }
                            InternalNodeExpressionKind::WhileLoop => {
                                to_while_loop_strategy(
                                    expression_strategy,
                                    statement_count_range.clone(),
                                )
                                    .prop_map(Expression::WhileLoop)
                                    .boxed()
                            }
                        }
                    }))
                    .boxed()
                }
            },
        )
    })
}

fn to_internal_node_expression_kinds_strategy(
    count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<Vec<InternalNodeExpressionKind>>> {
    assert!(!count_range.is_empty());
    prop::collection::vec(
        proptest::sample::select(INTERNAL_NODE_EXPRESSION_KINDS.to_vec()),
        count_range,
    )
    .prop_map(|mut kinds| {
        kinds.sort_by(|left, right| {
            left.partial_cmp(right).unwrap_or(Ordering::Equal)
        });
        if let Some(kind) = kinds.pop() {
            let mut result = Vec::with_capacity(kinds.len());
            let mut group_precedence = OptionalPrecedence::from(&kind);
            let mut group = vec![kind];
            while let Some(kind) = kinds.pop() {
                let precedence = OptionalPrecedence::from(&kind);
                if group_precedence != precedence
                    || precedence == OptionalPrecedence(None)
                {
                    result.push(group);
                    group = vec![];
                    group_precedence = precedence;
                }
                group.push(kind);
            }
            result.push(group);
            result
        } else {
            vec![]
        }
    })
}

fn to_annotated_identifier_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = AnnotatedIdentifier<StringType>> {
    (
        to_identifier_strategy(),
        expression_strategy.prop_map(Box::new),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(identifier, annotation, operator_position, operator_fillers)| {
                AnnotatedIdentifier {
                    identifier,
                    annotation,
                    operator_position,
                    operator_fillers,
                }
            },
        )
        .prop_recursive(
            max_depth as u32,
            (max_depth as u32 / 2u32).max(1u32),
            1u32,
            move |step| {
                (
                    to_identifier_strategy(),
                    step.prop_map(|value| {
                        Box::new(Expression::AnnotatedIdentifier(value))
                    }),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(
                        |(
                            identifier,
                            annotation,
                            operator_position,
                            operator_fillers,
                        )| {
                            let result = AnnotatedIdentifier {
                                identifier,
                                annotation,
                                operator_position,
                                operator_fillers,
                            };
                            result.validate_contents().unwrap_or_else(
                                |errors| {
                                    panic_on_validation_errors(&result, errors)
                                },
                            );
                            result
                        },
                    )
            },
        )
}

fn to_assignment_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = Assignment<StringType>> {
    let target_strategy = expression_strategy.clone().prop_map(Box::new);
    let value_strategy = expression_strategy.prop_map(Box::new);
    let constructor =
        |(target, value, operator_position, operator_fillers)| {
            let result = Assignment {
                target,
                value,
                operator_position,
                operator_fillers,
            };
            result.validate_contents().unwrap_or_else(|errors| {
                panic_on_validation_errors(&result, errors)
            });
            result
        };
    (
        target_strategy.clone(),
        value_strategy,
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                (
                    target_strategy.clone(),
                    step.prop_map(|value| {
                        Box::new(Expression::Assignment(value))
                    }),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(constructor)
            },
        )
}

fn to_bidirectional_conditional_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = BidirectionalConditional<StringType>> {
    let antecedents = (
        expression_strategy.clone(),
        to_non_comment_filler_strategy(),
    )
        .prop_map(|(antecedent, non_comment_filler)| {
            antecedent.to_non_lexically_conflicting(non_comment_filler)
        })
        .prop_map(Box::new)
        .boxed();
    (
        antecedents.clone(),
        to_block_strategy(
            expression_strategy.clone(),
            statement_count_range.clone(),
        ),
        prop_oneof![
            to_block_strategy(
                expression_strategy.clone(),
                statement_count_range.clone(),
            )
            .prop_map(|value| Box::new(Expression::Block(value))),
            (
                to_unidirectional_conditional_strategy(
                    expression_strategy.clone(),
                    statement_count_range.clone(),
                ),
                to_non_comment_filler_strategy()
            )
                .prop_map(
                    |(mut unidirectional_conditional, non_comment_filler)| {
                        if unidirectional_conditional.opener_fillers.is_empty()
                        {
                            unidirectional_conditional
                                .opener_fillers
                                .push(non_comment_filler)
                        }
                        unidirectional_conditional
                    }
                )
                .prop_map(|value| {
                    Box::new(Expression::UnidirectionalConditional(value))
                })
        ],
        to_substring_position_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(
                antecedent,
                consequent,
                alternative,
                antecedent_opener_position,
                alternative_opener_position,
                antecedent_opener_fillers,
                alternative_opener_fillers,
            )| {
                BidirectionalConditional {
                    antecedent,
                    consequent,
                    alternative,
                    antecedent_opener_position,
                    alternative_opener_position,
                    antecedent_opener_fillers,
                    alternative_opener_fillers,
                }
            },
        )
        .prop_recursive(2u32, 2u32, 1u32, move |step| {
            (
                antecedents.clone(),
                to_block_strategy(
                    expression_strategy.clone(),
                    statement_count_range.clone(),
                ),
                (step, to_non_comment_filler_strategy())
                    .prop_map(|(mut conditional, non_comment_filler)| {
                        if conditional.antecedent_opener_fillers.is_empty() {
                            conditional
                                .antecedent_opener_fillers
                                .push(non_comment_filler)
                        }
                        conditional
                    })
                    .prop_map(|value| {
                        Box::new(Expression::BidirectionalConditional(value))
                    }),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
            )
                .prop_map(
                    |(
                        antecedent,
                        consequent,
                        alternative,
                        antecedent_opener_position,
                        alternative_opener_position,
                        antecedent_opener_fillers,
                        alternative_opener_fillers,
                    )| {
                        BidirectionalConditional {
                            antecedent,
                            consequent,
                            alternative,
                            antecedent_opener_position,
                            alternative_opener_position,
                            antecedent_opener_fillers,
                            alternative_opener_fillers,
                        }
                    },
                )
        })
}

fn to_binary_arithmetic_operation_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    operator: BinaryArithmeticOperator,
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = BinaryArithmeticOperation<StringType>> {
    let left_operands = expression_strategy.clone().prop_map(Box::new);
    let right_operands = if operator == BinaryArithmeticOperator::Division {
        (expression_strategy, to_non_comment_filler_strategy())
            .prop_map(|(expression, non_comment_filler)| {
                expression.to_valid_divisor(non_comment_filler)
            })
            .prop_map(Box::new)
            .boxed()
    } else {
        expression_strategy.clone().prop_map(Box::new).boxed()
    };
    let constructor =
        move |(left, right, operator_position, operator_fillers)| {
            let result = BinaryArithmeticOperation {
                left,
                right,
                operator,
                operator_position,
                operator_fillers,
            };
            result.validate_contents().unwrap_or_else(|errors| {
                panic_on_validation_errors(&result, errors)
            });
            result
        };
    (
        left_operands.clone(),
        right_operands.clone(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                match Associativity::from(operator) {
                    Associativity::LeftToRight => (
                        step.prop_map(|value| {
                            Box::new(Expression::BinaryArithmeticOperation(
                                value,
                            ))
                        })
                        .boxed(),
                        right_operands.clone().boxed(),
                        to_substring_position_strategy(),
                        to_fillers_strategy(),
                    ),
                    Associativity::RightToLeft => (
                        left_operands.clone().boxed(),
                        step.prop_map(|value| {
                            Box::new(Expression::BinaryArithmeticOperation(
                                value,
                            ))
                        })
                        .boxed(),
                        to_substring_position_strategy(),
                        to_fillers_strategy(),
                    ),
                }
                .prop_map(constructor)
            },
        )
}

fn to_binary_comparison_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    operator: BinaryComparisonOperator,
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = BinaryComparison<StringType>> {
    let left_operands = expression_strategy.clone().prop_map(Box::new);
    let right_operands = expression_strategy.clone().prop_map(Box::new);
    let constructor =
        move |(left, right, operator_position, operator_fillers)| {
            let result = BinaryComparison {
                left,
                right,
                operator,
                operator_position,
                operator_fillers,
            };
            result.validate_contents().unwrap_or_else(|errors| {
                panic_on_validation_errors(&result, errors)
            });
            result
        };
    (
        left_operands.clone(),
        right_operands.clone(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                match Associativity::from(operator) {
                    Associativity::LeftToRight => (
                        step.prop_map(|value| {
                            Box::new(Expression::BinaryComparison(value))
                        })
                        .boxed(),
                        right_operands.clone().boxed(),
                        to_substring_position_strategy(),
                        to_fillers_strategy(),
                    ),
                    Associativity::RightToLeft => (
                        left_operands.clone().boxed(),
                        step.prop_map(|value| {
                            Box::new(Expression::BinaryComparison(value))
                        })
                        .boxed(),
                        to_substring_position_strategy(),
                        to_fillers_strategy(),
                    ),
                }
                .prop_map(constructor)
            },
        )
}

fn to_block_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Block<StringType>> {
    (
        to_statements_strategy(
            to_statement_strategy(expression_strategy.clone()),
            statement_count_range,
        ),
        prop::option::of(expression_strategy.prop_map(Box::new)),
        to_substring_position_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(
                statements,
                expression,
                open_brace_position,
                close_brace_position,
                open_brace_fillers,
                close_brace_fillers,
            )| Block {
                statements,
                expression,
                open_brace_position,
                close_brace_position,
                open_brace_fillers,
                close_brace_fillers,
            },
        )
}

fn to_call_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = Call<StringType>> {
    let arguments_with_comma_positions =
        to_expressions_strategy(expression_strategy.clone())
            .prop_flat_map(|arguments| {
                let arguments_count = arguments.len();
                (
                    prop::strategy::Just(arguments),
                    prop::collection::vec(
                        to_substring_position_strategy(),
                        arguments_count.saturating_sub(1usize)
                            ..=arguments_count,
                    ),
                )
            })
            .boxed();
    let constructor = |(
        callable,
        (arguments, comma_positions),
        open_parenthesis_position,
        close_parenthesis_position,
        open_parenthesis_fillers,
        mut comma_fillers,
        close_parenthesis_fillers,
    ): (_, (_, Vec<_>), _, _, _, Vec<Vec<_>>, _)| {
        comma_fillers.truncate(comma_positions.len());
        for _ in comma_fillers.len()..comma_positions.len() {
            comma_fillers.push(vec![]);
        }
        let result = Call {
            callable,
            arguments,
            open_parenthesis_position,
            comma_positions,
            close_parenthesis_position,
            open_parenthesis_fillers,
            comma_fillers,
            close_parenthesis_fillers,
        };
        result.validate_contents().unwrap_or_else(|errors| {
            panic_on_validation_errors(&result, errors)
        });
        result
    };
    (
        expression_strategy.clone().prop_map(Box::new),
        arguments_with_comma_positions.clone(),
        to_substring_position_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
        prop::collection::vec(
            to_fillers_strategy(),
            0usize..=MAX_EXPRESSIONS_SIZE,
        ),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                (
                    step.prop_map(|value| Box::new(Expression::Call(value))),
                    arguments_with_comma_positions.clone(),
                    to_substring_position_strategy(),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                    prop::collection::vec(
                        to_fillers_strategy(),
                        0usize..=MAX_EXPRESSIONS_SIZE,
                    ),
                    to_fillers_strategy(),
                )
                    .prop_map(constructor)
            },
        )
}

fn to_grouping_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Grouping<StringType>> {
    (
        expression_strategy.prop_map(Box::new),
        to_substring_position_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(
                expression,
                open_parenthesis_position,
                close_parenthesis_position,
                open_parenthesis_fillers,
                close_parenthesis_fillers,
            )| Grouping {
                expression,
                open_parenthesis_position,
                close_parenthesis_position,
                open_parenthesis_fillers,
                close_parenthesis_fillers,
            },
        )
}

fn to_member_access_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = MemberAccess<StringType>> {
    let constructor =
        |(object, member, operator_position, operator_fillers)| {
            let result = MemberAccess {
                object,
                member,
                operator_position,
                operator_fillers,
            };
            result.validate_contents().unwrap_or_else(|errors| {
                panic_on_validation_errors(&result, errors)
            });
            result
        };
    (
        expression_strategy.prop_map(Box::new),
        to_identifier_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                (
                    step.prop_map(|value| {
                        Box::new(Expression::MemberAccess(value))
                    }),
                    to_identifier_strategy(),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(constructor)
            },
        )
}

fn to_expressions_strategy<Expression: Clone + Debug>(
    expression_strategy: impl Strategy<Value = Expression>,
) -> impl Strategy<Value = Vec<Expression>> {
    prop::collection::vec(expression_strategy, 0usize..=MAX_EXPRESSIONS_SIZE)
}

fn to_return_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static + Strategy<Value = Expression<StringType>>,
    max_depth: usize,
) -> impl Strategy<Value = Return<StringType>> {
    let expressions = (expression_strategy, to_non_comment_filler_strategy())
        .prop_map(|(expression, non_comment_filler)| {
            expression.to_non_lexically_conflicting(non_comment_filler)
        })
        .prop_map(Box::new)
        .boxed();
    let constructor =
        move |(expression, operator_position, operator_fillers)| {
            let result = Return {
                expression,
                operator_position,
                operator_fillers,
            };
            result.validate_contents().unwrap_or_else(|errors| {
                panic_on_validation_errors(&result, errors)
            });
            result
        };
    (
        expressions,
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                (
                    (step, to_non_comment_filler_strategy())
                        .prop_map(|(expression, non_comment_filler)| {
                            expression.to_non_lexically_conflicting(
                                non_comment_filler,
                            )
                        })
                        .prop_map(|value| Box::new(Expression::Return(value))),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(constructor)
            },
        )
}

fn to_statements_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    statement_strategy: impl Strategy<Value = Statement<StringType>>,
    count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<Statement<StringType>>> {
    prop::collection::vec(statement_strategy, count_range)
}

fn to_statement_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Statement<StringType>> {
    (
        expression_strategy,
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(expression, semicolon_position, semicolon_fillers)| {
                Statement::Expression(ExpressionStatement {
                    expression,
                    semicolon_position,
                    semicolon_fillers,
                })
            },
        )
}

fn to_unary_arithmetic_operation_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static + Strategy<Value = Expression<StringType>>,
    operator: UnaryArithmeticOperator,
    max_depth: usize,
) -> impl Strategy<Value = UnaryArithmeticOperation<StringType>> {
    let constructor = move |(operand, operator_position, operator_fillers)| {
        let result = UnaryArithmeticOperation {
            operand,
            operator,
            operator_position,
            operator_fillers,
        };
        result.validate_contents().unwrap_or_else(|errors| {
            panic_on_validation_errors(&result, errors)
        });
        result
    };
    (
        expression_strategy.prop_map(Box::new),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(constructor)
        .prop_recursive(
            max_depth as u32,
            max_recursion_depth_to_desired_size(max_depth) as u32,
            1u32,
            move |step| {
                (
                    step.prop_map(|value| {
                        Box::new(Expression::UnaryArithmeticOperation(value))
                    }),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(constructor)
            },
        )
}

fn to_unidirectional_conditional_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = UnidirectionalConditional<StringType>> {
    let antecedents = (
        expression_strategy.clone(),
        to_non_comment_filler_strategy(),
    )
        .prop_map(|(antecedent, non_comment_filler)| {
            antecedent.to_non_lexically_conflicting(non_comment_filler)
        })
        .prop_map(Box::new)
        .boxed();
    (
        antecedents,
        to_block_strategy(
            expression_strategy.clone(),
            statement_count_range.clone(),
        ),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(antecedent, consequent, opener_position, opener_fillers)| {
                UnidirectionalConditional {
                    antecedent,
                    consequent,
                    opener_position,
                    opener_fillers,
                }
            },
        )
}

fn to_while_loop_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = WhileLoop<StringType>> {
    let conditions = (
        expression_strategy.clone(),
        to_non_comment_filler_strategy(),
    )
        .prop_map(|(condition, non_comment_filler)| {
            condition.to_non_lexically_conflicting(non_comment_filler)
        })
        .prop_map(Box::new)
        .boxed();
    (
        conditions,
        to_block_strategy(
            expression_strategy.clone(),
            statement_count_range.clone(),
        ),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(|(condition, body, opener_position, opener_fillers)| {
            WhileLoop {
                condition,
                body,
                opener_position,
                opener_fillers,
            }
        })
}

fn to_byte_count_strategy() -> impl Strategy<Value = ByteCount> {
    (0usize..).prop_map(ByteCount::from)
}

fn to_character_position_strategy() -> impl Strategy<Value = CharacterPosition>
{
    (to_byte_count_strategy(), to_utf_8_count_strategy())
        .prop_map(|(byte, utf_8)| CharacterPosition { byte, utf_8 })
}

fn to_filler_content_strategy<
    StringType: AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = FillerContent<StringType>> {
    prop_oneof![
        to_non_comment_filler_content_strategy(),
        to_comment_line_string_strategy().prop_map(|value| {
            FillerContent::CommentLine(StringType::from(value.as_str()))
        }),
        to_comment_block_string_strategy().prop_map(|value| {
            let result =
                FillerContent::CommentBlock(StringType::from(value.as_str()));
            result.validate().unwrap_or_else(|errors| {
                const ERRORS_SEPARATOR: &str = "\n    ";
                panic!(
                    "Invalid comment block content: {:?}.\nErrors:{}{}",
                    result,
                    ERRORS_SEPARATOR,
                    errors
                        .into_iter()
                        .map(|error| error.to_string())
                        .collect::<Vec<_>>()
                        .join(ERRORS_SEPARATOR)
                )
            });
            result
        }),
    ]
}

fn to_filler_strategy<
    StringType: AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = Filler<StringType>> {
    (
        to_filler_content_strategy(),
        to_substring_position_strategy(),
    )
        .prop_map(|(content, position)| Filler { content, position })
}

fn to_fillers_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
) -> impl Strategy<Value = Fillers<StringType>> {
    prop::collection::vec(
        to_filler_strategy::<String>(),
        0usize..=MAX_FILLERS_SIZE,
    )
    .prop_map(|fillers| {
        let mut result = Fillers::<StringType>::with_capacity(fillers.len());
        let convert_non_whitespace_filler =
            |filler: Filler<String>| match filler.content {
                FillerContent::CommentBlock(value) => Filler {
                    content: FillerContent::CommentBlock(
                        value.as_str().into(),
                    ),
                    position: filler.position,
                },
                FillerContent::CommentLine(value) => Filler {
                    content: FillerContent::CommentLine(value.as_str().into()),
                    position: filler.position,
                },
                FillerContent::Newline => Filler {
                    content: FillerContent::Newline,
                    position: filler.position,
                },
                FillerContent::Whitespace(_) => {
                    unreachable!("Unexpected whitespace")
                }
            };
        let mut fillers = fillers.into_iter();
        while let Some(filler) = fillers.next() {
            match filler.content {
                FillerContent::Whitespace(value) => {
                    let mut whitespace_string = value;
                    for next_filler in fillers.by_ref() {
                        match next_filler.content {
                            FillerContent::Whitespace(next_value) => {
                                whitespace_string
                                    .push_str(next_value.as_str());
                            }
                            _ => {
                                result.push(Filler {
                                    content: FillerContent::Whitespace(
                                        whitespace_string.as_str().into(),
                                    ),
                                    position: SubstringPosition {
                                        start: filler.position.start,
                                        end: next_filler.position.start,
                                    },
                                });
                                result.push(convert_non_whitespace_filler(
                                    next_filler,
                                ));
                                break;
                            }
                        }
                    }
                }
                _ => {
                    result.push(convert_non_whitespace_filler(filler));
                }
            }
        }
        result
    })
    .prop_map(|fillers| {
        let mut result = Fillers::<StringType>::with_capacity(fillers.len());
        for filler in fillers {
            if matches!(filler.content, FillerContent::CommentLine(_)) {
                let newline = Filler {
                    content: FillerContent::Newline,
                    position: filler.position.clone(),
                };
                result.push(filler);
                result.push(newline);
            } else {
                result.push(filler);
            }
        }
        result
    })
}

fn to_floating_point_literal_value_strategy<
    StringType: Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = StringType> {
    prop::string::string_regex(&to_floating_point_value_pattern())
        .unwrap()
        .prop_map(|value| StringType::from(value.as_str()))
}

fn to_function_definition_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
    annotation_max_depth: usize,
) -> impl Strategy<Value = FunctionDefinition<StringType>> {
    to_expressions_strategy(to_annotated_identifier_strategy(
        expression_strategy.clone(),
        annotation_max_depth,
    ))
    .prop_flat_map(|parameters| {
        prop::collection::vec(
            to_substring_position_strategy(),
            parameters.len().saturating_sub(1usize)..=parameters.len(),
        )
        .prop_flat_map(move |comma_positions| {
            let comma_count = comma_positions.len();
            (
                prop::strategy::Just(parameters.clone()),
                prop::strategy::Just(comma_positions),
                prop::collection::vec(to_fillers_strategy(), comma_count),
            )
        })
    })
    .prop_flat_map(
        move |(parameters, comma_positions, comma_fillers)| {
            (
                expression_strategy.clone().prop_map(Box::new),
                to_block_strategy(
                    expression_strategy.clone(),
                    statement_count_range.clone(),
                ),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
            )
                .prop_map(
                    move |(
                        return_type,
                        body,
                        opener_position,
                        open_parenthesis_position,
                        close_parenthesis_position,
                        arrow_position,
                        opener_fillers,
                        open_parenthesis_fillers,
                        close_parenthesis_fillers,
                        arrow_fillers,
                    )| {
                        FunctionDefinition {
                            parameters: parameters.clone(),
                            return_type,
                            body,
                            opener_position,
                            open_parenthesis_position,
                            comma_positions: comma_positions.clone(),
                            close_parenthesis_position,
                            arrow_position,
                            opener_fillers,
                            open_parenthesis_fillers,
                            comma_fillers: comma_fillers.clone(),
                            close_parenthesis_fillers,
                            arrow_fillers,
                        }
                    },
                )
        },
    )
}

fn to_function_type_strategy<
    StringType: 'static + AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = FunctionType<StringType>> {
    to_expressions_strategy(expression_strategy.clone())
        .prop_flat_map(|parameters| {
            prop::collection::vec(
                to_substring_position_strategy(),
                parameters.len().saturating_sub(1usize)..=parameters.len(),
            )
            .prop_flat_map(move |comma_positions| {
                let comma_count = comma_positions.len();
                (
                    prop::strategy::Just(parameters.clone()),
                    prop::strategy::Just(comma_positions),
                    prop::collection::vec(to_fillers_strategy(), comma_count),
                )
            })
        })
        .prop_flat_map(move |(parameters, comma_positions, comma_fillers)| {
            (
                expression_strategy.clone().prop_map(Box::new),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
            )
                .prop_map(
                    move |(
                        return_type,
                        open_parenthesis_position,
                        close_parenthesis_position,
                        operator_position,
                        open_parenthesis_fillers,
                        close_parenthesis_fillers,
                        operator_fillers,
                    )| {
                        FunctionType {
                            parameters: parameters.clone(),
                            return_type,
                            open_parenthesis_position,
                            comma_positions: comma_positions.clone(),
                            close_parenthesis_position,
                            operator_position,
                            open_parenthesis_fillers,
                            comma_fillers: comma_fillers.clone(),
                            close_parenthesis_fillers,
                            operator_fillers,
                        }
                    },
                )
        })
}

fn to_identifier_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = Identifier<StringType>> {
    (
        to_identifier_string_strategy()
            .prop_map(|value| StringType::from(value.as_str())),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(|(string, position, fillers)| Identifier {
            string,
            position,
            fillers,
        })
}

fn to_integer_literal_value_strategy<
    StringType: Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = StringType> {
    prop::string::string_regex(&to_integer_value_pattern())
        .unwrap()
        .prop_map(|value| StringType::from(value.as_str()))
}

fn to_non_comment_filler_content_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = FillerContent<StringType>> {
    prop_oneof![
        prop::strategy::Just(FillerContent::Newline),
        to_whitespace_string_strategy().prop_map(|value| {
            FillerContent::Whitespace(StringType::from(value.as_str()))
        }),
    ]
}

fn to_non_comment_filler_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = Filler<StringType>> {
    (
        to_non_comment_filler_content_strategy(),
        to_substring_position_strategy(),
    )
        .prop_map(|(content, position)| Filler { content, position })
}

fn to_numeric_literal_strategy<
    StringType: AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = NumericLiteral<StringType>> {
    prop_oneof![
        (
            to_integer_literal_value_strategy(),
            prop_oneof![
                prop::strategy::Just(NumericLiteralType::I8),
                prop::strategy::Just(NumericLiteralType::I16),
                prop::strategy::Just(NumericLiteralType::I32),
                prop::strategy::Just(NumericLiteralType::I64),
                prop::strategy::Just(NumericLiteralType::ISize),
                prop::strategy::Just(NumericLiteralType::U8),
                prop::strategy::Just(NumericLiteralType::U16),
                prop::strategy::Just(NumericLiteralType::U32),
                prop::strategy::Just(NumericLiteralType::U64),
                prop::strategy::Just(NumericLiteralType::USize),
            ],
            to_substring_position_strategy(),
            to_fillers_strategy(),
        ),
        (
            to_floating_point_literal_value_strategy(),
            prop_oneof![
                prop::strategy::Just(NumericLiteralType::F32),
                prop::strategy::Just(NumericLiteralType::F64),
            ],
            to_substring_position_strategy(),
            to_fillers_strategy(),
        ),
    ]
    .prop_map(|(value, type_, position, fillers)| {
        let result = NumericLiteral {
            value,
            type_,
            position,
            fillers,
        };
        result.validate_contents().unwrap_or_else(|errors| {
            const ERRORS_SEPARATOR: &str = "\n    ";
            panic!(
                "Invalid numeric literal: {:?}.\nErrors:{}{}",
                result,
                ERRORS_SEPARATOR,
                errors
                    .into_iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<_>>()
                    .join(ERRORS_SEPARATOR)
            )
        });
        result
    })
}

fn to_script_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
    statement_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Script<StringType>> {
    (
        to_statements_strategy(
            to_statement_strategy(expression_strategy),
            statement_count_range,
        ),
        to_fillers_strategy(),
    )
        .prop_map(|(statements, fillers)| Script {
            statements,
            fillers,
        })
}

fn to_substring_position_strategy() -> impl Strategy<Value = SubstringPosition>
{
    (
        to_character_position_strategy(),
        to_character_position_strategy(),
    )
        .prop_map(|(start, end)| SubstringPosition { start, end })
}

fn to_tuple_strategy<
    StringType: AsRef<str> + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Tuple<StringType>> {
    to_expressions_strategy(expression_strategy)
        .prop_flat_map(|elements| {
            prop::collection::vec(
                to_substring_position_strategy(),
                if elements.len() == 1 {
                    elements.len()
                } else {
                    elements.len().saturating_sub(1usize)
                }..=elements.len(),
            )
            .prop_flat_map(move |comma_positions| {
                let comma_count = comma_positions.len();
                (
                    prop::strategy::Just(elements.clone()),
                    prop::strategy::Just(comma_positions),
                    prop::collection::vec(to_fillers_strategy(), comma_count),
                )
            })
        })
        .prop_flat_map(move |(elements, comma_positions, comma_fillers)| {
            (
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_fillers_strategy(),
                to_fillers_strategy(),
            )
                .prop_map(
                    move |(
                        open_parenthesis_position,
                        close_parenthesis_position,
                        open_parenthesis_fillers,
                        close_parenthesis_fillers,
                    )| {
                        let result = Tuple {
                            elements: elements.clone(),
                            open_parenthesis_position,
                            comma_positions: comma_positions.clone(),
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            comma_fillers: comma_fillers.clone(),
                            close_parenthesis_fillers,
                        };
                        result.validate_contents().unwrap_or_else(|errors| {
                            const ERRORS_SEPARATOR: &str = "\n    ";
                            panic!(
                                "Invalid tuple: {:?}.\nErrors:{}{}",
                                result,
                                ERRORS_SEPARATOR,
                                errors
                                    .into_iter()
                                    .map(|error| error.to_string())
                                    .collect::<Vec<_>>()
                                    .join(ERRORS_SEPARATOR)
                            )
                        });
                        result
                    },
                )
        })
}

fn to_utf_8_count_strategy() -> impl Strategy<Value = Utf8Count> {
    (0usize..).prop_map(Utf8Count::from)
}

const fn max_recursion_depth_to_desired_size(depth: usize) -> usize {
    match depth / 2usize {
        0usize => 1usize,
        value => value,
    }
}

fn panic_on_validation_errors<T: Debug>(
    result: &T,
    errors: Vec<impl Error + Sized>,
) {
    const ERRORS_SEPARATOR: &str = "\n    ";
    panic!(
        "Invalid {:?}.\nErrors:{}{}",
        result,
        ERRORS_SEPARATOR,
        errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join(ERRORS_SEPARATOR)
    )
}

const STACK_SIZE: usize = 20usize * 1024usize * 1024usize;

#[test]
fn test_script_reset_positions() {
    macro_rules! run_sub_test {
        ($string_type:ty, $token_string_type:ty) => {
            grow(STACK_SIZE, || {
                proptest!(
                    |(
                        mut script in to_script_strategy(
                            to_expression_strategy::<$string_type>(
                                0usize..=MAX_EXPRESSION_DEPTH,
                                0usize..=MAX_STATEMENTS_SIZE,
                            ),
                            0usize..=MAX_STATEMENTS_SIZE,
                        ),
                    )| {
                        let tokens_from_string =
                            TokenCollection::<$token_string_type>::try_from(
                                Tokenize::<$token_string_type>::tokenize(
                                    script.clone(),
                                )
                                .into_iter()
                                .map(|token| token.content.to_string())
                                .collect::<Vec<_>>()
                                .join("")
                                .as_str(),
                            );
                        script.reset_positions();
                        assert_eq!(
                            tokens_from_string,
                            Ok(Tokenize::<$token_string_type>::tokenize(script))
                        )
                    }
                )
            })
        };
    }

    run_sub_test!(Arc<str>, Arc<str>);
    run_sub_test!(Box<str>, Arc<str>);
    run_sub_test!(Box<str>, Box<str>);
    run_sub_test!(Box<str>, Rc<str>);
    run_sub_test!(Box<str>, String);
    run_sub_test!(Rc<str>, Rc<str>);
    run_sub_test!(String, Arc<str>);
    run_sub_test!(String, Box<str>);
    run_sub_test!(String, Rc<str>);
    run_sub_test!(String, String);
}

#[test]
fn test_script_round_trip() {
    macro_rules! run_sub_test {
        ($string_type:ty, $token_string_type:ty) => {
            grow(STACK_SIZE, || {
                proptest!(
                    |(
                        script in to_script_strategy(
                            to_expression_strategy::<$string_type>(
                                0usize..=MAX_EXPRESSION_DEPTH,
                                0usize..=MAX_STATEMENTS_SIZE,
                            ),
                            0usize..=MAX_STATEMENTS_SIZE,
                        ),
                    )| {
                        assert_eq!(
                            Script::try_from(
                                Tokenize::<$token_string_type>::tokenize(
                                    script.clone()
                                )
                            ),
                            Ok(Script::<$token_string_type>::from(script))
                        )
                })
            })
        };
    }

    run_sub_test!(Arc<str>, Arc<str>);
    run_sub_test!(Box<str>, Arc<str>);
    run_sub_test!(Box<str>, Box<str>);
    run_sub_test!(Box<str>, Rc<str>);
    run_sub_test!(Box<str>, String);
    run_sub_test!(Rc<str>, Rc<str>);
    run_sub_test!(String, Arc<str>);
    run_sub_test!(String, Box<str>);
    run_sub_test!(String, Rc<str>);
    run_sub_test!(String, String);
}
