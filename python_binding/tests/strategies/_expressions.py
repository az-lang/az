from __future__ import annotations

import enum as _enum
from abc import ABC as _ABC, abstractmethod as _abstractmethod
from collections.abc import (
    Callable as _Callable,
    Mapping as _Mapping,
    Sequence as _Sequence,
)
from itertools import groupby as _groupby
from types import MappingProxyType as _MappingProxyType
from typing import Generic as _Generic, TypeVar as _TypeVar

from az.parsing import (
    AnnotatedIdentifier as _AnnotatedIdentifier,
    AnnotationOperator as _AnnotationOperator,
    Assignment as _Assignment,
    AssignmentOperator as _AssignmentOperator,
    Associativity as _Associativity,
    BidirectionalConditional as _BidirectionalConditional,
    BinaryAdditionOperator as _BinaryAdditionOperator,
    BinaryArithmeticOperation as _BinaryArithmeticOperation,
    BinaryArithmeticOperator as _BinaryArithmeticOperator,
    BinaryComparison as _BinaryComparison,
    BinaryComparisonOperator as _BinaryComparisonOperator,
    BinaryDivisionOperator as _BinaryDivisionOperator,
    BinaryEqualToOperator as _BinaryEqualToOperator,
    BinaryGreaterThanOperator as _BinaryGreaterThanOperator,
    BinaryGreaterThanOrEqualToOperator as _BinaryGreaterThanOrEqualToOperator,
    BinaryLessThanOperator as _BinaryLessThanOperator,
    BinaryLessThanOrEqualToOperator as _BinaryLessThanOrEqualToOperator,
    BinaryMultiplicationOperator as _BinaryMultiplicationOperator,
    BinaryNotEqualToOperator as _BinaryNotEqualToOperator,
    BinarySubtractionOperator as _BinarySubtractionOperator,
    Block as _Block,
    Call as _Call,
    CallOperator as _CallOperator,
    Expression as _Expression,
    FunctionDefinition as _FunctionDefinition,
    FunctionType as _FunctionType,
    FunctionTypeOperator as _FunctionTypeOperator,
    Grouping as _Grouping,
    Identifier as _Identifier,
    MemberAccess as _MemberAccess,
    MemberAccessOperator as _MemberAccessOperator,
    NumericLiteral as _NumericLiteral,
    Precedence as _Precedence,
    Return as _Return,
    ReturnOperator as _ReturnOperator,
    Tuple as _Tuple,
    UnaryArithmeticOperation as _UnaryArithmeticOperation,
    UnaryArithmeticOperator as _UnaryArithmeticOperator,
    UnaryNegationOperator as _UnaryNegationOperator,
    UnidirectionalConditional as _UnidirectionalConditional,
    WhileLoop as _WhileLoop,
)
from hypothesis import strategies as _st

from . import _factories
from ._fillers import (
    non_comment_filler_strategy as _non_comment_filler_strategy,
)
from ._leaf_expressions import (
    identifier_strategy as _identifier_strategy,
    numeric_literal_strategy as _numeric_literal_strategy,
)

_leaf_expression_strategies = {
    _identifier_strategy: _Identifier,
    _numeric_literal_strategy: _NumericLiteral,
}
_leaf_expression_strategy = _st.one_of(
    list(_leaf_expression_strategies.keys())
)

_ExpressionT = _TypeVar('_ExpressionT', bound=_Expression, covariant=True)


class _NoPrecedence(_enum.Enum):
    _VALUE = _enum.auto()


_NO_PRECEDENCE = _NoPrecedence._VALUE

_PrecedenceT = _TypeVar('_PrecedenceT', _Precedence, _NoPrecedence)


class _InternalNodeExpressionKind(_Generic[_ExpressionT, _PrecedenceT], _ABC):
    @property
    @_abstractmethod
    def cls(self, /) -> type[_ExpressionT]: ...

    @property
    @_abstractmethod
    def factory_argument_min_precedence_indices(self, /) -> _Sequence[int]: ...

    @property
    @_abstractmethod
    def precedence(self, /) -> _PrecedenceT: ...

    @_abstractmethod
    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_ExpressionT]: ...


class _AnnotatedIdentifierExpressionKind(
    _InternalNodeExpressionKind[_AnnotatedIdentifier, _Precedence]
):
    @property
    def cls(self, /) -> type[_AnnotatedIdentifier]:
        return _AnnotatedIdentifier

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (_to_precedence_index(self.precedence),)

    @property
    def precedence(self, /) -> _Precedence:
        return _AnnotationOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_AnnotatedIdentifier]:
        (annotation_strategy,) = arguments_strategies
        return _factories.to_annotated_identifier_strategy(annotation_strategy)


class _AssignmentExpressionKind(
    _InternalNodeExpressionKind[_Assignment, _Precedence]
):
    @property
    def cls(self, /) -> type[_Assignment]:
        return _Assignment

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return _binary_operator_to_precedence_indices(_AssignmentOperator)

    @property
    def precedence(self, /) -> _Precedence:
        return _AssignmentOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Assignment]:
        target_strategy, value_strategy = arguments_strategies
        return _factories.to_assignment_strategy(
            target_strategy, value_strategy
        )


class _BidirectionalConditionalExpressionKind(
    _InternalNodeExpressionKind[_BidirectionalConditional, _NoPrecedence]
):
    @property
    def cls(self, /) -> type[_BidirectionalConditional]:
        return _BidirectionalConditional

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return 0, 0

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_BidirectionalConditional]:
        antecedent_base_strategy, consequent_base_strategy = (
            arguments_strategies
        )
        return _factories.to_bidirectional_conditional_strategy(
            antecedent_base_strategy, consequent_base_strategy
        )


class _BinaryArithmeticOperationExpressionKind(
    _InternalNodeExpressionKind[_BinaryArithmeticOperation, _Precedence]
):
    @property
    def cls(self, /) -> type[_BinaryArithmeticOperation]:
        return _BinaryArithmeticOperation

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return _binary_operator_to_precedence_indices(self._operator)

    @property
    def precedence(self, /) -> _Precedence:
        return self._operator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_BinaryArithmeticOperation]:
        left_operand_strategy, right_operand_strategy = arguments_strategies
        return _factories.to_binary_arithmetic_operation_strategy(
            self._operator, left_operand_strategy, right_operand_strategy
        )

    def __init__(self, operator: _BinaryArithmeticOperator, /) -> None:
        self._operator = operator


class _BinaryComparisonExpressionKind(
    _InternalNodeExpressionKind[_BinaryComparison, _Precedence]
):
    @property
    def cls(self, /) -> type[_BinaryComparison]:
        return _BinaryComparison

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return _binary_operator_to_precedence_indices(self._operator)

    @property
    def precedence(self, /) -> _Precedence:
        return self._operator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_BinaryComparison]:
        left_operand_strategy, right_operand_strategy = arguments_strategies
        return _factories.to_binary_comparison_strategy(
            self._operator, left_operand_strategy, right_operand_strategy
        )

    def __init__(self, operator: _BinaryComparisonOperator, /) -> None:
        self._operator = operator


class _BlockExpressionKind(_InternalNodeExpressionKind[_Block, _NoPrecedence]):
    @property
    def cls(self, /) -> type[_Block]:
        return _Block

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (0,)

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Block]:
        (expression_strategy,) = arguments_strategies
        return _factories.to_block_strategy(expression_strategy)


class _CallExpressionKind(_InternalNodeExpressionKind[_Call, _Precedence]):
    @property
    def cls(self, /) -> type[_Call]:
        return _Call

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return _to_precedence_index(self.precedence), 0

    @property
    def precedence(self, /) -> _Precedence:
        return _CallOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Call]:
        callable_strategy, argument_strategy = arguments_strategies
        return _factories.to_call_strategy(
            callable_strategy, argument_strategy
        )


class _FunctionTypeExpressionKind(
    _InternalNodeExpressionKind[_FunctionType, _Precedence]
):
    @property
    def cls(self, /) -> type[_FunctionType]:
        return _FunctionType

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return _binary_operator_to_precedence_indices(_FunctionTypeOperator)

    @property
    def precedence(self, /) -> _Precedence:
        return _FunctionTypeOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_FunctionType]:
        parameter_strategy, return_type_strategy = arguments_strategies
        return _factories.to_function_type_strategy(
            parameter_strategy, return_type_strategy
        )


class _FunctionDefinitionExpressionKind(
    _InternalNodeExpressionKind[_FunctionDefinition, _Precedence]
):
    @property
    def cls(self, /) -> type[_FunctionDefinition]:
        return _FunctionDefinition

    @property
    def factory_argument_min_precedence_indices(
        self, /
    ) -> tuple[int, int, int]:
        return _to_precedence_index(self.precedence), 0, 0

    @property
    def precedence(self, /) -> _Precedence:
        # function parameters are annotated identifiers
        return _AnnotationOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_FunctionDefinition]:
        (
            parameter_annotations,
            return_type_strategy,
            body_expression_strategy,
        ) = arguments_strategies
        return _factories.to_function_definition_strategy(
            parameter_annotations,
            return_type_strategy,
            body_expression_strategy,
        )


class _GroupingExpressionKind(
    _InternalNodeExpressionKind[_Grouping, _NoPrecedence]
):
    @property
    def cls(self, /) -> type[_Grouping]:
        return _Grouping

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (0,)

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Grouping]:
        (expression_strategy,) = arguments_strategies
        return _factories.to_grouping_strategy(expression_strategy)


class _MemberAccessExpressionKind(
    _InternalNodeExpressionKind[_MemberAccess, _Precedence]
):
    @property
    def cls(self, /) -> type[_MemberAccess]:
        return _MemberAccess

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (_to_precedence_index(self.precedence),)

    @property
    def precedence(self, /) -> _Precedence:
        return _MemberAccessOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_MemberAccess]:
        (object_strategy,) = arguments_strategies
        return _factories.to_member_access_strategy(object_strategy)


class _ReturnExpressionKind(_InternalNodeExpressionKind[_Return, _Precedence]):
    @property
    def cls(self, /) -> type[_Return]:
        return _Return

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (_to_precedence_index(self.precedence),)

    @property
    def precedence(self, /) -> _Precedence:
        return _ReturnOperator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Return]:
        (expression_strategy,) = arguments_strategies
        return _factories.to_return_strategy(expression_strategy)


class _TupleExpressionKind(_InternalNodeExpressionKind[_Tuple, _NoPrecedence]):
    @property
    def cls(self, /) -> type[_Tuple]:
        return _Tuple

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (0,)

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_Tuple]:
        (element_strategy,) = arguments_strategies
        return _factories.to_tuple_strategy(element_strategy)


class _UnaryArithmeticOperationExpressionKind(
    _InternalNodeExpressionKind[_UnaryArithmeticOperation, _Precedence]
):
    @property
    def cls(self, /) -> type[_UnaryArithmeticOperation]:
        return _UnaryArithmeticOperation

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int]:
        return (_to_precedence_index(self.precedence),)

    @property
    def precedence(self, /) -> _Precedence:
        return self._operator.PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_UnaryArithmeticOperation]:
        (operand_strategy,) = arguments_strategies
        return _factories.to_unary_arithmetic_operation_strategy(
            self._operator, operand_strategy
        )

    def __init__(self, operator: _UnaryArithmeticOperator, /) -> None:
        self._operator = operator


class _UnidirectionalConditionalExpressionKind(
    _InternalNodeExpressionKind[_UnidirectionalConditional, _NoPrecedence]
):
    @property
    def cls(self, /) -> type[_UnidirectionalConditional]:
        return _UnidirectionalConditional

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return 0, 0

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_UnidirectionalConditional]:
        antecedent_base_strategy, consequent_base_strategy = (
            arguments_strategies
        )
        return _factories.to_unidirectional_conditional_strategy(
            antecedent_base_strategy, consequent_base_strategy
        )


class _WhileLoopExpressionKind(
    _InternalNodeExpressionKind[_WhileLoop, _NoPrecedence]
):
    @property
    def cls(self, /) -> type[_WhileLoop]:
        return _WhileLoop

    @property
    def factory_argument_min_precedence_indices(self, /) -> tuple[int, int]:
        return 0, 0

    @property
    def precedence(self, /) -> _NoPrecedence:
        return _NO_PRECEDENCE

    def factory(
        self,
        arguments_strategies: _Sequence[_st.SearchStrategy[_Expression]],
        /,
    ) -> _st.SearchStrategy[_WhileLoop]:
        condition_base_strategy, body_base_strategy = arguments_strategies
        return _factories.to_while_loop_strategy(
            condition_base_strategy, body_base_strategy
        )


def _binary_operator_to_precedence_indices(
    operator: (
        type[_AssignmentOperator]
        | type[_FunctionTypeOperator]
        | _BinaryArithmeticOperator
        | _BinaryComparisonOperator
    ),
    /,
) -> tuple[int, int]:
    operator_precedence_index = _to_precedence_index(operator.PRECEDENCE)
    return (
        (operator_precedence_index, operator_precedence_index + 1)
        if operator.ASSOCIATIVITY is _Associativity.LEFT_TO_RIGHT
        else (operator_precedence_index + 1, operator_precedence_index)
    )


_operation_kinds: list[
    _InternalNodeExpressionKind[_Expression, _Precedence]
] = [
    _AnnotatedIdentifierExpressionKind(),
    _AssignmentExpressionKind(),
    _CallExpressionKind(),
    _FunctionDefinitionExpressionKind(),
    _FunctionTypeExpressionKind(),
    _MemberAccessExpressionKind(),
    _ReturnExpressionKind(),
]
_binary_arithmetic_operators: list[_BinaryArithmeticOperator] = [
    _BinaryAdditionOperator,
    _BinaryDivisionOperator,
    _BinaryMultiplicationOperator,
    _BinarySubtractionOperator,
]
_operation_kinds += [
    _BinaryArithmeticOperationExpressionKind(operator)
    for operator in _binary_arithmetic_operators
]
_binary_comparison_operators: list[_BinaryComparisonOperator] = [
    _BinaryEqualToOperator,
    _BinaryGreaterThanOperator,
    _BinaryGreaterThanOrEqualToOperator,
    _BinaryLessThanOperator,
    _BinaryLessThanOrEqualToOperator,
    _BinaryNotEqualToOperator,
]
_operation_kinds += [
    _BinaryComparisonExpressionKind(operator)
    for operator in _binary_comparison_operators
]
_unary_arithmetic_operators: list[_UnaryArithmeticOperator] = [
    _UnaryNegationOperator
]
_operation_kinds += [
    _UnaryArithmeticOperationExpressionKind(operator)
    for operator in _unary_arithmetic_operators
]
_rest_internal_node_expression_kinds: list[
    _InternalNodeExpressionKind[_Expression, _NoPrecedence]
] = [
    _BidirectionalConditionalExpressionKind(),
    _BlockExpressionKind(),
    _GroupingExpressionKind(),
    _TupleExpressionKind(),
    _UnidirectionalConditionalExpressionKind(),
    _WhileLoopExpressionKind(),
]

assert all(
    (
        any(isinstance(kind, kind_cls) for kind in _operation_kinds)
        is not any(
            isinstance(kind, kind_cls)
            for kind in _rest_internal_node_expression_kinds
        )
    )
    for kind_cls in _InternalNodeExpressionKind.__subclasses__()
), [
    kind_cls
    for kind_cls in _InternalNodeExpressionKind.__subclasses__()
    if (
        any(isinstance(kind, kind_cls) for kind in _operation_kinds)
        is any(
            isinstance(kind, kind_cls)
            for kind in _rest_internal_node_expression_kinds
        )
    )
]
assert all(
    (
        any(expression_cls is kind.cls for kind in _operation_kinds)
        is not any(
            expression_cls is kind.cls
            for kind in _rest_internal_node_expression_kinds
        )
    )
    for expression_cls in _Expression.__subclasses__()
    if expression_cls not in _leaf_expression_strategies.values()
), [
    expression_cls
    for expression_cls in _Expression.__subclasses__()
    if (
        expression_cls not in _leaf_expression_strategies.values()
        and (
            any(expression_cls is kind.cls for kind in _operation_kinds)
            is any(
                expression_cls is kind.cls
                for kind in _rest_internal_node_expression_kinds
            )
        )
    )
]

_operator_precedences = [
    precedence
    for precedence, _ in _groupby(
        sorted([kind.precedence for kind in _operation_kinds])
    )
]
_precedence_indices = range(len(_operator_precedences))
_to_precedence_index = _operator_precedences.index
_grouped_operator_kinds: _Mapping[
    int, _Sequence[_InternalNodeExpressionKind[_Expression, _Precedence]]
] = _MappingProxyType(
    {
        _to_precedence_index(precedence): list(group)
        for precedence, group in _groupby(
            sorted(_operation_kinds, key=lambda factory: factory.precedence),
            key=lambda factory: factory.precedence,
        )
    }
)


def _precedence_index_to_expression_strategy(
    precedence_index: int, /
) -> _st.SearchStrategy[_Expression]:
    return _st.one_of(
        [
            kind.factory(
                [
                    _st.one_of(
                        [
                            _leaf_expression_strategy,
                            *[
                                _internal_node_expressions_strategies[
                                    next_precedence_index
                                ]
                                for next_precedence_index in (
                                    _precedence_indices[
                                        argument_precedence_index:
                                    ]
                                )
                            ],
                            _no_precedence_expression_strategy,
                        ]
                    )
                    for argument_precedence_index in (
                        kind.factory_argument_min_precedence_indices
                    )
                ]
            )
            for kind in _grouped_operator_kinds[precedence_index]
        ]
    )


def _precedence_index_to_expression_strategy_factory(
    precedence_index: int, /
) -> _Callable[[], _st.SearchStrategy[_Expression]]:
    return lambda: _precedence_index_to_expression_strategy(precedence_index)


_T = _TypeVar('_T')


def _to_pairs(
    coordinate_strategy: _st.SearchStrategy[_T], /
) -> _st.SearchStrategy[tuple[_T, _T]]:
    return _st.tuples(coordinate_strategy, coordinate_strategy)


_internal_node_expressions_strategies: dict[
    int, _st.SearchStrategy[_Expression]
] = {
    precedence_index: _st.deferred(
        _precedence_index_to_expression_strategy_factory(precedence_index)
    )
    for precedence_index in _precedence_indices
}
_no_precedence_expression_strategy: _st.SearchStrategy[_Expression] = (
    _st.deferred(
        lambda: _st.one_of(
            [
                kind.factory(
                    [
                        _st.one_of(
                            [
                                _leaf_expression_strategy,
                                *[
                                    _internal_node_expressions_strategies[
                                        next_precedence_index
                                    ]
                                    for next_precedence_index in (
                                        _precedence_indices[
                                            argument_precedence_index:
                                        ]
                                    )
                                ],
                                _no_precedence_expression_strategy,
                            ]
                        )
                        for argument_precedence_index in (
                            kind.factory_argument_min_precedence_indices
                        )
                    ]
                )
                for kind in _rest_internal_node_expression_kinds
            ]
        )
    )
)

expression_strategy = _st.one_of(
    _leaf_expression_strategy, *_internal_node_expressions_strategies.values()
)
optional_expression_strategy = _st.none() | expression_strategy
expression_list_strategy = _st.lists(
    expression_strategy, max_size=_factories.MAX_EXPRESSIONS_SIZE
)


def _precedence_to_child_expression_strategy(
    precedence: _Precedence, /
) -> _st.SearchStrategy[_Expression]:
    return _st.one_of(
        *[
            _precedence_index_to_expression_strategy(precedence_index)
            for precedence_index in _precedence_indices[
                _to_precedence_index(precedence) + 1 :
            ]
        ],
        _no_precedence_expression_strategy,
    )


annotation_strategy = _precedence_to_child_expression_strategy(
    _AnnotationOperator.PRECEDENCE
)
annotated_identifier_strategy = _factories.to_annotated_identifier_strategy(
    annotation_strategy
)
assignment_target_strategy, assignment_value_strategy = (
    _precedence_index_to_expression_strategy(operand_precedence_index)
    for operand_precedence_index in _binary_operator_to_precedence_indices(
        _AssignmentOperator
    )
)
assignment_strategy = _factories.to_assignment_strategy(
    assignment_target_strategy, assignment_value_strategy
)
binary_arithmetic_operation_strategy = _st.one_of(
    [
        _factories.to_binary_arithmetic_operation_strategy(
            operator,
            *[
                _precedence_index_to_expression_strategy(
                    operand_precedence_index
                )
                for operand_precedence_index in (
                    _binary_operator_to_precedence_indices(operator)
                )
            ],
        )
        for operator in _binary_arithmetic_operators
    ]
)


def _to_binary_arithmetic_operation_operand_pair_strategy(
    operator: _BinaryArithmeticOperator, /
) -> _st.SearchStrategy[tuple[_Expression, _Expression]]:
    operand_strategy = _precedence_to_child_expression_strategy(
        operator.PRECEDENCE
    )
    return _st.tuples(
        operand_strategy,
        (
            _st.builds(
                _factories.to_valid_divisor,
                operand_strategy,
                _non_comment_filler_strategy,
            )
            if operator is _BinaryDivisionOperator
            else operand_strategy
        ),
    )


associativity_strategy = _st.sampled_from(
    [_Associativity.LEFT_TO_RIGHT, _Associativity.RIGHT_TO_LEFT]
)
binary_arithmetic_operator_with_operand_pair_strategy = _st.one_of(
    [
        _st.tuples(
            _st.just(operator),
            _to_binary_arithmetic_operation_operand_pair_strategy(operator),
        )
        for operator in _binary_arithmetic_operators
    ]
)
binary_comparison_strategy = _st.one_of(
    [
        _factories.to_binary_comparison_strategy(
            operator,
            *[
                _precedence_index_to_expression_strategy(
                    operand_precedence_index
                )
                for operand_precedence_index in (
                    _binary_operator_to_precedence_indices(operator)
                )
            ],
        )
        for operator in _binary_comparison_operators
    ]
)
binary_arithmetic_operator_strategy = _st.sampled_from(
    _binary_arithmetic_operators
)
binary_comparison_operator_strategy = _st.sampled_from(
    _binary_comparison_operators
)
binary_comparison_operator_with_operands_pair_strategy = _st.one_of(
    [
        _st.tuples(
            _st.just(operator),
            _to_pairs(
                _precedence_to_child_expression_strategy(operator.PRECEDENCE)
            ),
        )
        for operator in _binary_comparison_operators
    ]
)
block_strategy = _factories.to_block_strategy(
    _st.one_of(
        _leaf_expression_strategy,
        *_internal_node_expressions_strategies.values(),
    )
)
callable_strategy = _precedence_to_child_expression_strategy(
    _CallOperator.PRECEDENCE
)
call_strategy = _factories.to_call_strategy(
    callable_strategy, expression_strategy
)
unary_arithmetic_operator_strategy = _st.sampled_from(
    _unary_arithmetic_operators
)
