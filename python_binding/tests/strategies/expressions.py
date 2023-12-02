from __future__ import annotations

import string
import typing as _t
from functools import partial, reduce
from itertools import chain, groupby
from operator import add

from hypothesis import strategies as _st
from hypothesis.extra.lark import from_lark
from lark import Lark

from az.parsing import (
    AnnotatedIdentifier,
    Assignment,
    BinaryAdditionOperator,
    BinaryAnnotationOperator,
    BinaryArithmeticOperation,
    BinaryArithmeticOperator,
    BinaryAssignmentOperator,
    BinaryComparison,
    BinaryComparisonOperator,
    BinaryDivisionOperator,
    BinaryEqualToOperator,
    BinaryGreaterThanOperator,
    BinaryGreaterThanOrEqualToOperator,
    BinaryLowerThanOperator,
    BinaryLowerThanOrEqualToOperator,
    BinaryMultiplicationOperator,
    BinaryNotEqualToOperator,
    BinarySubtractionOperator,
    Block,
    Call,
    CallOperator,
    Conditional,
    Expression,
    FunctionDefinition,
    Grouping,
    Identifier,
    MemberAccess,
    MemberAccessOperator,
    NumericLiteral,
    NumericLiteralType,
    Precedence,
    Script,
    Tuple,
    UnaryArithmeticOperation,
    UnaryNegationOperator,
)
from az.tokenization import tokenize_string

from tests.utils import statement_to_expression
from .factories import (
    fillers_lists,
    identifiers,
    substrings_positions,
    to_annotated_identifiers,
    to_assignments,
    to_binary_arithmetic_operations,
    to_binary_comparisons,
    to_blocks,
    to_calls,
    to_conditionals,
    to_function_definitions,
    to_groupings,
    to_member_accesses,
    to_tuples,
    to_unary_arithmetic_operations,
)
from .grammar import common_grammar

MAX_EXPRESSION_DEPTH = 12
MAX_OPERATORS_SIZE = 4
MAX_NON_OPERATORS_SIZE = 2

maybe_digits = _st.text(string.digits)
integer_literals_values = _st.just('0') | _st.builds(
    add, _st.sampled_from(string.digits[1:]), maybe_digits
)
digit_pattern = r'\d'
variants = [
    integer_literals_values,
    _st.builds(add, _st.just('.'), integer_literals_values),
    _st.builds(
        add,
        integer_literals_values.map(lambda value: value + '.'),
        maybe_digits,
    ),
]
variants += [
    _st.tuples(
        variant,
        _st.sampled_from('eE'),
        _st.sampled_from(['+', '-', '']),
        integer_literals_values,
    ).map(''.join)
    for variant in variants
]
floating_point_literals_values = _st.one_of(variants)
numeric_literals = _st.builds(
    NumericLiteral,
    integer_literals_values,
    _st.sampled_from(
        [
            NumericLiteralType.I8,
            NumericLiteralType.I16,
            NumericLiteralType.I32,
            NumericLiteralType.I64,
            NumericLiteralType.ISIZE,
            NumericLiteralType.U8,
            NumericLiteralType.U16,
            NumericLiteralType.U32,
            NumericLiteralType.U64,
            NumericLiteralType.USIZE,
        ]
    ),
    fillers=fillers_lists,
    position=substrings_positions,
) | _st.builds(
    NumericLiteral,
    floating_point_literals_values,
    _st.sampled_from([NumericLiteralType.F32, NumericLiteralType.F64]),
    fillers=fillers_lists,
    position=substrings_positions,
)
_base_expressions_strategies = {
    identifiers: Identifier,
    numeric_literals: NumericLiteral,
}
base_expressions = _st.one_of(list(_base_expressions_strategies.keys()))


class _ExpressionsStrategyFactoryWrapper:
    cls: type[Expression]
    factory: _t.Callable[
        [_st.SearchStrategy[Expression]], _st.SearchStrategy[Expression]
    ]
    precedence: OptionalPrecedence

    def __init__(
        self,
        cls: type[Expression],
        factory: _t.Callable[
            [_st.SearchStrategy[Expression]], _st.SearchStrategy[Expression]
        ],
        precedence: OptionalPrecedence,
    ) -> None:
        self.cls, self.factory, self.precedence = cls, factory, precedence

    def __repr__(self) -> str:
        return (
            f'{type(self).__qualname__}'
            f'({self.cls!r}, {self.factory!r}, {self.precedence!r})'
        )


class OptionalPrecedence:
    def __init__(self, value: Precedence | None) -> None:
        self.value = value

    def __eq__(self, other: object) -> bool:
        return (
            self.value == other.value
            if isinstance(other, OptionalPrecedence)
            else NotImplemented
        )

    def __lt__(self, other: _t.Self) -> bool:
        return (
            self.value is not None
            and other.value is not None
            and self.value < other.value
        )

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}({self.value!r})'


_binary_arithmetic_operators: list[BinaryArithmeticOperator] = [
    BinaryAdditionOperator,
    BinaryDivisionOperator,
    BinaryMultiplicationOperator,
    BinarySubtractionOperator,
]
_binary_comparison_operators: list[BinaryComparisonOperator] = [
    BinaryEqualToOperator,
    BinaryGreaterThanOperator,
    BinaryGreaterThanOrEqualToOperator,
    BinaryLowerThanOperator,
    BinaryLowerThanOrEqualToOperator,
    BinaryNotEqualToOperator,
]
_non_operators_factories = [
    _ExpressionsStrategyFactoryWrapper(
        Block, to_blocks, OptionalPrecedence(None)
    ),
    _ExpressionsStrategyFactoryWrapper(
        Conditional, to_conditionals, OptionalPrecedence(None)
    ),
    _ExpressionsStrategyFactoryWrapper(
        Grouping, to_groupings, OptionalPrecedence(None)
    ),
    _ExpressionsStrategyFactoryWrapper(
        Tuple, to_tuples, OptionalPrecedence(None)
    ),
]
_operators_factories: list[_ExpressionsStrategyFactoryWrapper] = (
    [
        _ExpressionsStrategyFactoryWrapper(
            FunctionDefinition,
            to_function_definitions,
            OptionalPrecedence(
                # function parameters are annotated identifiers
                BinaryAnnotationOperator.PRECEDENCE
            ),
        )
    ]
    + [
        _ExpressionsStrategyFactoryWrapper(
            UnaryArithmeticOperation,
            partial(to_unary_arithmetic_operations, operator),
            OptionalPrecedence(operator.PRECEDENCE),
        )
        for operator in [UnaryNegationOperator]
    ]
    + [
        _ExpressionsStrategyFactoryWrapper(
            AnnotatedIdentifier,
            to_annotated_identifiers,
            OptionalPrecedence(BinaryAnnotationOperator.PRECEDENCE),
        ),
        _ExpressionsStrategyFactoryWrapper(
            Assignment,
            to_assignments,
            OptionalPrecedence(BinaryAssignmentOperator.PRECEDENCE),
        ),
        _ExpressionsStrategyFactoryWrapper(
            Call, to_calls, OptionalPrecedence(CallOperator.PRECEDENCE)
        ),
        _ExpressionsStrategyFactoryWrapper(
            MemberAccess,
            to_member_accesses,
            OptionalPrecedence(MemberAccessOperator.PRECEDENCE),
        ),
    ]
    + [
        _ExpressionsStrategyFactoryWrapper(
            BinaryArithmeticOperation,
            partial(to_binary_arithmetic_operations, operator),
            OptionalPrecedence(operator.PRECEDENCE),
        )
        for operator in _binary_arithmetic_operators
    ]
    + [
        _ExpressionsStrategyFactoryWrapper(
            BinaryComparison,
            partial(to_binary_comparisons, operator),
            OptionalPrecedence(operator.PRECEDENCE),
        )
        for operator in _binary_comparison_operators
    ]
)

_expressions_factories = [*_operators_factories, *_non_operators_factories]
assert all(
    any(expression_cls is factory.cls for factory in _expressions_factories)
    for expression_cls in Expression.__subclasses__()
    if expression_cls not in _base_expressions_strategies.values()
), [
    expression_cls
    for expression_cls in Expression.__subclasses__()
    if (
        expression_cls not in _base_expressions_strategies.values()
        and all(
            expression_cls is not factory.cls
            for factory in _expressions_factories
        )
    )
]


def _reduce_operators(
    factories: list[_ExpressionsStrategyFactoryWrapper],
) -> list[_ExpressionsStrategyFactoryWrapper]:
    factories.sort(key=lambda factory: factory.precedence)
    factories.reverse()
    return [
        next(group)
        for _, group in groupby(factories, lambda factory: factory.precedence)
    ]


def _apply_factories(
    base: _st.SearchStrategy[Expression],
    wrappers: list[_ExpressionsStrategyFactoryWrapper],
) -> _st.SearchStrategy[Expression]:
    return reduce(
        lambda strategy, wrapper: wrapper.factory(strategy), wrappers, base
    )


def _flatten_list(value: list[list[_t.Any]]) -> list[_t.Any]:
    return list(chain.from_iterable(value))


_T = _t.TypeVar('_T')


def _merge_lists(left: list[_T], right: list[_T]) -> list[_T]:
    left.extend(right)
    return left


expressions = (
    base_expressions
    | (
        (
            _st.builds(
                _merge_lists,
                _st.lists(
                    _st.builds(
                        _merge_lists,
                        _st.lists(
                            _st.sampled_from(_operators_factories),
                            max_size=MAX_OPERATORS_SIZE,
                        ).map(_reduce_operators),
                        _st.lists(
                            _st.sampled_from(_non_operators_factories),
                            min_size=1,
                            max_size=MAX_NON_OPERATORS_SIZE,
                        ),
                    ),
                    max_size=(
                        MAX_EXPRESSION_DEPTH
                        // (MAX_OPERATORS_SIZE + MAX_NON_OPERATORS_SIZE)
                        - 1
                    ),
                ).map(_flatten_list),
                _st.lists(
                    _st.sampled_from(_operators_factories),
                    min_size=1,
                    max_size=MAX_OPERATORS_SIZE,
                ).map(_reduce_operators),
            )
            | _st.builds(
                _merge_lists,
                _st.lists(
                    _st.builds(
                        _merge_lists,
                        _st.lists(
                            _st.sampled_from(_non_operators_factories),
                            min_size=1,
                            max_size=MAX_NON_OPERATORS_SIZE,
                        ),
                        _st.lists(
                            _st.sampled_from(_operators_factories),
                            max_size=MAX_OPERATORS_SIZE,
                        ).map(_reduce_operators),
                    ),
                    max_size=(
                        MAX_EXPRESSION_DEPTH
                        // (MAX_OPERATORS_SIZE + MAX_NON_OPERATORS_SIZE)
                        - 1
                    ),
                ).map(_flatten_list),
                _st.lists(
                    _st.sampled_from(_non_operators_factories),
                    min_size=1,
                    max_size=MAX_OPERATORS_SIZE,
                ),
            )
        ).flatmap(partial(_apply_factories, base_expressions))
    )
    | from_lark(Lark('start: statement\n' f'{common_grammar}')).map(
        lambda statement_string: statement_to_expression(
            Script.from_tokens(tokenize_string(statement_string)).statements[0]
        )
    )
)
