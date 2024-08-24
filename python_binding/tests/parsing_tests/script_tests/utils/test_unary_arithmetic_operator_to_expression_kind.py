from __future__ import annotations

from az.parsing import UnaryArithmeticOperator
from hypothesis import given

from tests.strategies import unary_arithmetic_operator_strategy
from tests.utils import equivalence

from . import ExpressionKind, unary_arithmetic_operator_to_expression_kind


@given(unary_arithmetic_operator_strategy)
def test_basic(unary_arithmetic_operator: UnaryArithmeticOperator) -> None:
    result = unary_arithmetic_operator_to_expression_kind(
        unary_arithmetic_operator
    )

    assert isinstance(result, ExpressionKind)


@given(unary_arithmetic_operator_strategy, unary_arithmetic_operator_strategy)
def test_homomorphism_preserving_is_operator(
    first: UnaryArithmeticOperator, second: UnaryArithmeticOperator
) -> None:
    first_result, second_result = (
        unary_arithmetic_operator_to_expression_kind(first),
        unary_arithmetic_operator_to_expression_kind(second),
    )

    assert equivalence(first is second, first_result is second_result)
