use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::sync::Arc;

use proptest::prelude::{prop, Strategy};
use proptest::{prop_oneof, proptest};
use stacker::grow;

use az::parsing::{
    AnnotatedIdentifier, Assignment, BinaryAnnotationOperator,
    BinaryArithmeticOperation, BinaryArithmeticOperator,
    BinaryAssignmentOperator, BinaryComparison, BinaryComparisonOperator,
    Block, Call, CallOperator, Conditional, Expression, ExpressionStatement,
    Filler, FillerContent, FunctionDefinition, Grouping, Identifier,
    MemberAccess, MemberAccessOperator, NumericLiteral, Precedence, Script,
    Statement, Tuple, UnaryArithmeticOperation, UnaryArithmeticOperator,
};
use az::tokenization::{
    ByteIndex, CharacterPosition, NumericLiteralType, SubstringPosition,
    Tokenize, TryTokenize, Utf8Index,
};
use common::patterns::{
    to_floating_point_value_pattern, to_integer_value_pattern,
};
use common::strategies_factories::{
    to_comment_block_string_strategy, to_comment_line_string_strategy,
    to_identifier_string_strategy, to_whitespace_string_strategy,
};

mod common;

const MAX_EXPRESSIONS_SIZE: usize = 4usize;
const MAX_FILLERS_SIZE: usize = 2usize;
const MAX_OPERATION_DEPTH: usize = 3usize;
const MAX_NON_OPERATION_DEPTH: usize = 2usize;
const MAX_EXPRESSION_DEPTH: usize =
    2usize * (MAX_OPERATION_DEPTH + MAX_NON_OPERATION_DEPTH) + 1usize;
const MAX_STATEMENTS_SIZE: usize = 4usize;

type Fillers<StringType> = Vec<Filler<StringType>>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExpressionKind {
    AnnotatedIdentifier(BinaryAnnotationOperator),
    Assignment(BinaryAssignmentOperator),
    BinaryArithmeticOperation(BinaryArithmeticOperator),
    BinaryComparison(BinaryComparisonOperator),
    Block,
    Call(CallOperator),
    Conditional,
    FunctionDefinition(
        // function parameters are annotated identifiers
        BinaryAnnotationOperator,
    ),
    Grouping,
    MemberAccess(MemberAccessOperator),
    Tuple,
    UnaryArithmeticOperation(UnaryArithmeticOperator),
}

const OPERATIONS_KINDS: [ExpressionKind; 16] = [
    ExpressionKind::AnnotatedIdentifier(BinaryAnnotationOperator),
    ExpressionKind::Assignment(BinaryAssignmentOperator),
    ExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Addition,
    ),
    ExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Division,
    ),
    ExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Multiplication,
    ),
    ExpressionKind::BinaryArithmeticOperation(
        BinaryArithmeticOperator::Subtraction,
    ),
    ExpressionKind::BinaryComparison(BinaryComparisonOperator::EqualTo),
    ExpressionKind::BinaryComparison(BinaryComparisonOperator::GreaterThan),
    ExpressionKind::BinaryComparison(
        BinaryComparisonOperator::GreaterThanOrEqualTo,
    ),
    ExpressionKind::BinaryComparison(BinaryComparisonOperator::LowerThan),
    ExpressionKind::BinaryComparison(
        BinaryComparisonOperator::LowerThanOrEqualTo,
    ),
    ExpressionKind::BinaryComparison(BinaryComparisonOperator::NotEqualTo),
    ExpressionKind::Call(CallOperator),
    ExpressionKind::FunctionDefinition(BinaryAnnotationOperator),
    ExpressionKind::MemberAccess(MemberAccessOperator),
    ExpressionKind::UnaryArithmeticOperation(
        UnaryArithmeticOperator::Negation,
    ),
];
const NON_OPERATIONS_KINDS: [ExpressionKind; 4] = [
    ExpressionKind::Block,
    ExpressionKind::Conditional,
    ExpressionKind::Grouping,
    ExpressionKind::Tuple,
];

#[derive(Clone, Eq, PartialEq)]
struct OptionalPrecedence(Option<Precedence>);

impl PartialOrd for OptionalPrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptionalPrecedence {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Some(left), Some(right)) => left.cmp(right),
            _ => unreachable!("Precedence should be defined"),
        }
    }
}

impl From<ExpressionKind> for OptionalPrecedence {
    fn from(value: ExpressionKind) -> Self {
        match value {
            ExpressionKind::AnnotatedIdentifier(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::Assignment(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::BinaryArithmeticOperation(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::Call(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::BinaryComparison(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::FunctionDefinition(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::MemberAccess(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::UnaryArithmeticOperation(operator) => {
                OptionalPrecedence(Some(Precedence::from(operator)))
            }
            ExpressionKind::Block
            | ExpressionKind::Conditional
            | ExpressionKind::Grouping
            | ExpressionKind::Tuple => OptionalPrecedence(None),
        }
    }
}

impl PartialOrd for ExpressionKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            OptionalPrecedence::from(self.clone())
                .cmp(&OptionalPrecedence::from(other.clone())),
        )
    }
}

impl Ord for ExpressionKind {
    fn cmp(&self, other: &Self) -> Ordering {
        OptionalPrecedence::from(self.clone())
            .cmp(&OptionalPrecedence::from(other.clone()))
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

impl<StringType> ToValidDivisor<StringType> for Conditional<StringType> {
    fn to_valid_divisor(
        mut self,
        non_comment_filler: Filler<StringType>,
    ) -> Self {
        fill_divisor_fillers(&mut self.opener_fillers, non_comment_filler);
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
            Expression::Conditional(value) => Expression::Conditional(
                value.to_valid_divisor(non_comment_filler),
            ),
            Expression::FunctionDefinition(value) => {
                Expression::FunctionDefinition(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
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
            Expression::Tuple(value) => {
                Expression::Tuple(value.to_valid_divisor(non_comment_filler))
            }
            Expression::UnaryArithmeticOperation(value) => {
                Expression::UnaryArithmeticOperation(
                    value.to_valid_divisor(non_comment_filler),
                )
            }
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
    StringType: 'static + Clone + Debug + for<'a> From<&'a str>,
>(
    depths_range: RangeInclusive<usize>,
    non_operation_depths_range: RangeInclusive<usize>,
    operation_depths_range: RangeInclusive<usize>,
    statements_counts_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Expression<StringType>> {
    let base = prop_oneof![
        to_identifier_strategy().prop_map(Expression::Identifier),
        to_numeric_literal_strategy().prop_map(Expression::NumericLiteral),
    ];
    let (min_depth, max_depth) = depths_range.into_inner();
    to_expressions_kinds_strategy(
        (min_depth.saturating_sub(1usize))
            ..=(max_depth.saturating_sub(1usize)),
        non_operation_depths_range,
        operation_depths_range,
    )
    .prop_flat_map(move |kinds| {
        kinds.into_iter().fold(
            base.clone().boxed(),
            |expression_strategy, kind| match kind {
                ExpressionKind::AnnotatedIdentifier(_) => {
                    to_annotated_identifier_strategy(expression_strategy)
                        .prop_map(Expression::AnnotatedIdentifier)
                        .boxed()
                }
                ExpressionKind::Assignment(_) => {
                    to_assignment_strategy(expression_strategy)
                        .prop_map(Expression::Assignment)
                        .boxed()
                }
                ExpressionKind::BinaryArithmeticOperation(
                    operator @ BinaryArithmeticOperator::Division,
                ) => (
                    expression_strategy.clone().prop_map(Box::new),
                    (expression_strategy, to_non_comment_filler_strategy())
                        .prop_map(|(expression, non_comment_filler)| {
                            expression.to_valid_divisor(non_comment_filler)
                        })
                        .prop_map(Box::new),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(
                        move |(
                            left,
                            right,
                            operator_position,
                            operator_fillers,
                        )| {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left,
                                    right,
                                    operator,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        },
                    )
                    .boxed(),
                ExpressionKind::BinaryArithmeticOperation(operator) => (
                    expression_strategy.clone().prop_map(Box::new),
                    expression_strategy.prop_map(Box::new),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(
                        move |(
                            left,
                            right,
                            operator_position,
                            operator_fillers,
                        )| {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left,
                                    right,
                                    operator,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        },
                    )
                    .boxed(),
                ExpressionKind::BinaryComparison(operator) => (
                    expression_strategy.clone().prop_map(Box::new),
                    expression_strategy.prop_map(Box::new),
                    to_substring_position_strategy(),
                    to_fillers_strategy(),
                )
                    .prop_map(
                        move |(
                            left,
                            right,
                            operator_position,
                            operator_fillers,
                        )| {
                            Expression::BinaryComparison(BinaryComparison {
                                left,
                                right,
                                operator,
                                operator_position,
                                operator_fillers,
                            })
                        },
                    )
                    .boxed(),
                ExpressionKind::Block => to_block_strategy(
                    expression_strategy,
                    statements_counts_range.clone(),
                )
                .prop_map(Expression::Block)
                .boxed(),
                ExpressionKind::Call(_) => {
                    to_call_strategy(expression_strategy)
                        .prop_map(Expression::Call)
                        .boxed()
                }
                ExpressionKind::Conditional => to_conditional_strategy(
                    expression_strategy,
                    statements_counts_range.clone(),
                )
                .prop_map(Expression::Conditional)
                .boxed(),
                ExpressionKind::FunctionDefinition(_) => {
                    to_function_definition_strategy(
                        expression_strategy,
                        statements_counts_range.clone(),
                    )
                    .prop_map(Expression::FunctionDefinition)
                    .boxed()
                }
                ExpressionKind::Grouping => {
                    to_grouping_strategy(expression_strategy)
                        .prop_map(Expression::Grouping)
                        .boxed()
                }
                ExpressionKind::MemberAccess(_) => {
                    to_member_access_strategy(expression_strategy)
                        .prop_map(Expression::MemberAccess)
                        .boxed()
                }
                ExpressionKind::Tuple => {
                    to_tuple_strategy(expression_strategy)
                        .prop_map(Expression::Tuple)
                        .boxed()
                }
                ExpressionKind::UnaryArithmeticOperation(operator) => {
                    to_unary_arithmetic_operation_strategy(
                        expression_strategy,
                        operator,
                    )
                    .prop_map(Expression::UnaryArithmeticOperation)
                    .boxed()
                }
            },
        )
    })
}

fn to_expressions_kinds_strategy(
    depths_range: RangeInclusive<usize>,
    non_operation_depths_range: RangeInclusive<usize>,
    operation_depths_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<ExpressionKind>> {
    assert!(!depths_range.is_empty());
    assert!(!non_operation_depths_range.is_empty());
    assert!(!operation_depths_range.is_empty());
    let (min_depth, max_depth) = depths_range.into_inner();
    let (min_non_operation_depth, max_non_operation_depth) =
        non_operation_depths_range.into_inner();
    let (min_operation_depth, max_operation_depth) =
        operation_depths_range.into_inner();
    let to_non_operations_strategy = |min_size| {
        prop::collection::vec(
            proptest::sample::select(NON_OPERATIONS_KINDS.to_vec()),
            min_size..=max_non_operation_depth,
        )
    };
    let to_operations_strategy = |min_size| {
        proptest::collection::vec(
            proptest::sample::select(OPERATIONS_KINDS.to_vec()),
            min_size..=max_operation_depth,
        )
        .prop_map(|mut operations| {
            operations.sort();
            operations.reverse();
            operations.dedup_by(|next_kind, previous_kind| {
                matches!(
                    (
                        OptionalPrecedence::from(next_kind.clone()).0,
                        OptionalPrecedence::from(previous_kind.clone()).0,
                    ),
                    (Some(left_precedence), Some(right_precedence))
                    if left_precedence == right_precedence
                )
            });
            operations
        })
    };
    if max_non_operation_depth == 0 {
        return to_operations_strategy(min_depth).boxed();
    } else if max_operation_depth == 0 {
        return to_non_operations_strategy(min_depth).boxed();
    }
    let merge_kinds = |(mut left, mut right)| {
        Vec::<ExpressionKind>::append(&mut left, &mut right);
        left
    };
    let flatten_kinds_vec = |kinds_vec| {
        Vec::<Vec<ExpressionKind>>::into_iter(kinds_vec)
            .flatten()
            .collect::<Vec<_>>()
    };
    prop_oneof![
        (
            prop::collection::vec(
                (
                    to_operations_strategy(min_operation_depth),
                    to_non_operations_strategy(
                        if min_non_operation_depth == 0usize {
                            1usize
                        } else {
                            min_non_operation_depth
                        }
                    ),
                )
                    .prop_map(merge_kinds),
                min_depth / (max_operation_depth + max_non_operation_depth)
                    ..=max_depth
                        / (max_operation_depth + max_non_operation_depth)
                        - 1usize,
            )
            .prop_map(flatten_kinds_vec),
            to_operations_strategy(if min_operation_depth == 0usize {
                1usize
            } else {
                min_operation_depth
            })
        )
            .prop_map(merge_kinds),
        (
            prop::collection::vec(
                (
                    to_non_operations_strategy(
                        if min_non_operation_depth == 0usize {
                            1usize
                        } else {
                            min_non_operation_depth
                        }
                    ),
                    to_operations_strategy(min_operation_depth),
                )
                    .prop_map(merge_kinds),
                min_depth / (max_operation_depth + max_non_operation_depth)
                    ..=max_depth
                        / (max_operation_depth + max_non_operation_depth)
                        - 1usize,
            )
            .prop_map(flatten_kinds_vec),
            to_non_operations_strategy(if min_non_operation_depth == 0usize {
                1usize
            } else {
                min_non_operation_depth
            })
        )
            .prop_map(merge_kinds),
    ]
    .boxed()
}

fn to_annotated_identifier_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
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
}

fn to_assignment_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Assignment<StringType>> {
    (
        expression_strategy.clone().prop_map(Box::new),
        expression_strategy.prop_map(Box::new),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(target, value, operator_position, operator_fillers)| {
                Assignment {
                    target,
                    value,
                    operator_position,
                    operator_fillers,
                }
            },
        )
}

fn to_block_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
    statements_counts_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Block<StringType>> {
    (
        to_statements_strategy(
            to_statement_strategy(expression_strategy.clone()),
            statements_counts_range,
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
                opening_fillers,
                closing_fillers,
            )| Block {
                statements,
                expression,
                open_brace_position,
                close_brace_position,
                open_brace_fillers: opening_fillers,
                close_brace_fillers: closing_fillers,
            },
        )
}

fn to_call_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Call<StringType>> {
    to_expressions_strategy(expression_strategy.clone())
        .prop_flat_map(|arguments| {
            (
                prop::strategy::Just(arguments.clone()),
                prop::collection::vec(
                    to_substring_position_strategy(),
                    arguments.len().saturating_sub(1usize)..=arguments.len(),
                ),
            )
        })
        .prop_flat_map(move |(arguments, commas_positions)| {
            (
                expression_strategy.clone().prop_map(Box::new),
                to_substring_position_strategy(),
                to_substring_position_strategy(),
                to_fillers_strategy(),
                prop::collection::vec(
                    to_fillers_strategy(),
                    commas_positions.len(),
                ),
                to_fillers_strategy(),
            )
                .prop_map(
                    move |(
                        callable,
                        open_parenthesis_position,
                        close_parenthesis_position,
                        open_parenthesis_fillers,
                        commas_fillers,
                        close_parenthesis_fillers,
                    )| {
                        Call {
                            callable,
                            arguments: arguments.clone(),
                            open_parenthesis_position,
                            commas_positions: commas_positions.clone(),
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            commas_fillers,
                            close_parenthesis_fillers,
                        }
                    },
                )
        })
}

fn to_conditional_strategy<
    StringType: 'static + Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl 'static
        + Clone
        + Strategy<Value = Expression<StringType>>,
    statements_counts_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Conditional<StringType>> {
    let antecedents = (expression_strategy.clone(), to_filler_strategy())
        .prop_map(|(mut antecedent, filler)| {
            let mut cursor = &mut antecedent;
            loop {
                match cursor {
                    Expression::AnnotatedIdentifier(value) => {
                        value.identifier.fillers.push(filler);
                        break;
                    }
                    Expression::Assignment(value) => {
                        cursor = &mut value.target;
                    }
                    Expression::BinaryArithmeticOperation(value) => {
                        cursor = &mut value.left;
                    }
                    Expression::BinaryComparison(value) => {
                        cursor = &mut value.left;
                    }
                    Expression::Block(_) => break,
                    Expression::Call(value) => {
                        cursor = &mut value.callable;
                    }
                    Expression::Conditional(value) => {
                        value.opener_fillers.push(filler);
                        break;
                    }
                    Expression::FunctionDefinition(value) => {
                        value.opener_fillers.push(filler);
                        break;
                    }
                    Expression::Grouping(_) => break,
                    Expression::Identifier(value) => {
                        value.fillers.push(filler);
                        break;
                    }
                    Expression::MemberAccess(value) => {
                        cursor = &mut value.object;
                    }
                    Expression::NumericLiteral(value) => {
                        value.fillers.push(filler);
                        break;
                    }
                    Expression::Tuple(_) => break,
                    Expression::UnaryArithmeticOperation(_) => break,
                }
            }
            antecedent
        })
        .prop_map(Box::new)
        .boxed();
    prop_oneof![
        (
            antecedents.clone(),
            to_block_strategy(
                expression_strategy.clone(),
                statements_counts_range.clone()
            ),
            to_substring_position_strategy(),
            to_fillers_strategy(),
        )
            .prop_map(
                |(antecedent, consequent, opener_position, opener_fillers)| {
                    Conditional {
                        antecedent,
                        consequent,
                        alternative: None,
                        opener_position,
                        alternative_opener_position: None,
                        opener_fillers,
                        alternative_opener_fillers: vec![],
                    }
                },
            ),
        (
            antecedents.clone(),
            to_block_strategy(
                expression_strategy.clone(),
                statements_counts_range.clone(),
            ),
            to_block_strategy(
                expression_strategy.clone(),
                statements_counts_range.clone(),
            )
            .prop_map(|value| Some(Box::new(Expression::Block(value)))),
            to_substring_position_strategy(),
            to_substring_position_strategy().prop_map(Some),
            to_fillers_strategy(),
            to_fillers_strategy(),
        )
            .prop_map(
                |(
                    antecedent,
                    consequent,
                    alternative,
                    opener_position,
                    alternative_opener_position,
                    opener_fillers,
                    alternative_opener_fillers,
                )| {
                    Conditional {
                        antecedent,
                        consequent,
                        alternative,
                        opener_position,
                        alternative_opener_position,
                        opener_fillers,
                        alternative_opener_fillers,
                    }
                },
            ),
    ]
    .prop_recursive(2, 2, 1, move |step| {
        (
            antecedents.clone(),
            to_block_strategy(
                expression_strategy.clone(),
                statements_counts_range.clone(),
            ),
            (step, to_filler_strategy())
                .prop_map(|(mut conditional, filler)| {
                    conditional.opener_fillers.push(filler);
                    conditional
                })
                .prop_map(|value| {
                    Some(Box::new(Expression::Conditional(value)))
                }),
            to_substring_position_strategy(),
            to_substring_position_strategy().prop_map(Some),
            to_fillers_strategy(),
            to_fillers_strategy(),
        )
            .prop_map(
                |(
                    antecedent,
                    consequent,
                    alternative,
                    opener_position,
                    alternative_opener_position,
                    opener_fillers,
                    alternative_opener_fillers,
                )| {
                    Conditional {
                        antecedent,
                        consequent,
                        alternative,
                        opener_position,
                        alternative_opener_position,
                        opener_fillers,
                        alternative_opener_fillers,
                    }
                },
            )
    })
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
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = MemberAccess<StringType>> {
    (
        expression_strategy.prop_map(Box::new),
        to_identifier_strategy(),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            |(object, member, operator_position, operator_fillers)| {
                MemberAccess {
                    object,
                    member,
                    operator_position,
                    operator_fillers,
                }
            },
        )
}

fn to_expressions_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
) -> impl Strategy<Value = Vec<Expression<StringType>>> {
    prop::collection::vec(expression_strategy, 0usize..=MAX_EXPRESSIONS_SIZE)
}

fn to_statements_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    statement_strategy: impl Strategy<Value = Statement<StringType>>,
    counts_size_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<Statement<StringType>>> {
    prop::collection::vec(statement_strategy, counts_size_range)
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
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
    operator: UnaryArithmeticOperator,
) -> impl Strategy<Value = UnaryArithmeticOperation<StringType>> {
    (
        expression_strategy.prop_map(Box::new),
        to_substring_position_strategy(),
        to_fillers_strategy(),
    )
        .prop_map(
            move |(operand, operator_position, operator_fillers)| {
                UnaryArithmeticOperation {
                    operand,
                    operator,
                    operator_position,
                    operator_fillers,
                }
            },
        )
}

fn to_byte_index_strategy() -> impl Strategy<Value = ByteIndex> {
    (0usize..).prop_map(ByteIndex::from)
}

fn to_character_position_strategy() -> impl Strategy<Value = CharacterPosition>
{
    (to_byte_index_strategy(), to_utf_8_index_strategy())
        .prop_map(|(byte, utf_8)| CharacterPosition { byte, utf_8 })
}

fn to_filler_content_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = FillerContent<StringType>> {
    prop_oneof![
        to_non_comment_filler_content_strategy(),
        to_comment_line_string_strategy().prop_map(|value| {
            FillerContent::CommentLine(StringType::from(value.as_str()))
        }),
        to_comment_block_string_strategy().prop_map(|value| {
            FillerContent::CommentBlock(
                value
                    .split_inclusive('\n')
                    .map(StringType::from)
                    .collect::<Vec<_>>(),
            )
        }),
    ]
}

fn to_filler_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
) -> impl Strategy<Value = Filler<StringType>> {
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
                        value
                            .into_iter()
                            .map(|value| value.as_str().into())
                            .collect(),
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
                                        start_line: filler.position.start_line,
                                        start_character: filler
                                            .position
                                            .start_character,
                                        end_line: next_filler
                                            .position
                                            .start_line,
                                        end_character: next_filler
                                            .position
                                            .start_character,
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
}

fn to_floating_point_literal_value_strategy<
    StringType: Debug + for<'a> From<&'a str>,
>() -> impl Strategy<Value = StringType> {
    prop::string::string_regex(&to_floating_point_value_pattern())
        .unwrap()
        .prop_map(|value| StringType::from(value.as_str()))
}

fn to_function_definition_strategy<
    StringType: Clone + Debug + for<'a> From<&'a str>,
>(
    expression_strategy: impl Clone + Strategy<Value = Expression<StringType>>,
    statements_counts_range: RangeInclusive<usize>,
) -> impl Strategy<Value = FunctionDefinition<StringType>> {
    to_expressions_strategy(
        to_annotated_identifier_strategy(expression_strategy.clone())
            .prop_map(Expression::AnnotatedIdentifier),
    )
    .prop_flat_map(|parameters| {
        prop::collection::vec(
            to_substring_position_strategy(),
            parameters.len().saturating_sub(1usize)..=parameters.len(),
        )
        .prop_flat_map(move |commas_positions| {
            let commas_count = commas_positions.len();
            (
                prop::strategy::Just(parameters.clone()),
                prop::strategy::Just(commas_positions),
                prop::collection::vec(to_fillers_strategy(), commas_count),
            )
        })
    })
    .prop_flat_map(
        move |(parameters, commas_positions, commas_fillers)| {
            (
                expression_strategy.clone().prop_map(Box::new),
                to_block_strategy(
                    expression_strategy.clone(),
                    statements_counts_range.clone(),
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
                            commas_positions: commas_positions.clone(),
                            close_parenthesis_position,
                            arrow_position,
                            opener_fillers,
                            open_parenthesis_fillers,
                            commas_fillers: commas_fillers.clone(),
                            close_parenthesis_fillers,
                            arrow_fillers,
                        }
                    },
                )
        },
    )
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
    StringType: Clone + Debug + for<'a> From<&'a str>,
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
    .prop_map(|(value, type_, position, fillers)| NumericLiteral {
        value,
        type_,
        position,
        fillers,
    })
}

fn to_script_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
    expression_strategy: impl Strategy<Value = Expression<StringType>>,
    statements_counts_range: RangeInclusive<usize>,
) -> impl Strategy<Value = Script<StringType>> {
    (
        to_statements_strategy(
            to_statement_strategy(expression_strategy),
            statements_counts_range,
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
        0usize..,
        to_character_position_strategy(),
        0usize..,
        to_character_position_strategy(),
    )
        .prop_map(
            |(start_line, start_character, end_line, end_character)| {
                SubstringPosition {
                    start_line,
                    start_character,
                    end_line,
                    end_character,
                }
            },
        )
}

fn to_tuple_strategy<StringType: Clone + Debug + for<'a> From<&'a str>>(
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
            .prop_flat_map(move |commas_positions| {
                let commas_count = commas_positions.len();
                (
                    prop::strategy::Just(elements.clone()),
                    prop::strategy::Just(commas_positions),
                    prop::collection::vec(to_fillers_strategy(), commas_count),
                )
            })
        })
        .prop_flat_map(move |(elements, commas_positions, commas_fillers)| {
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
                    )| Tuple {
                        elements: elements.clone(),
                        open_parenthesis_position,
                        commas_positions: commas_positions.clone(),
                        close_parenthesis_position,
                        open_parenthesis_fillers,
                        commas_fillers: commas_fillers.clone(),
                        close_parenthesis_fillers,
                    },
                )
        })
}

const STACK_SIZE: usize = 7usize * 1024usize * 1024usize / 4usize;

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
                                0usize..=MAX_NON_OPERATION_DEPTH,
                                0usize..=MAX_OPERATION_DEPTH,
                                0usize..=MAX_STATEMENTS_SIZE,
                            ),
                            0usize..=MAX_STATEMENTS_SIZE,
                        ),
                    )| {
                        let tokens_from_string =
                            TryTokenize::<$token_string_type>::try_tokenize(
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
                                0usize..=MAX_NON_OPERATION_DEPTH,
                                0usize..=MAX_OPERATION_DEPTH,
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

fn to_utf_8_index_strategy() -> impl Strategy<Value = Utf8Index> {
    (0usize..).prop_map(Utf8Index::from)
}
