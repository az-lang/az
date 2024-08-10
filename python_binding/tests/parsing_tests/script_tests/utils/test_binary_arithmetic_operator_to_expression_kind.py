from __future__ import annotations

from az.parsing import BinaryArithmeticOperator
from hypothesis import given

from tests.strategies import binary_arithmetic_operator_strategy
from tests.utils import equivalence

from . import ExpressionKind, binary_arithmetic_operator_to_expression_kind


@given(binary_arithmetic_operator_strategy)
def test_basic(binary_arithmetic_operator: BinaryArithmeticOperator) -> None:
    result = binary_arithmetic_operator_to_expression_kind(
        binary_arithmetic_operator
    )

    assert isinstance(result, ExpressionKind)


@given(
    binary_arithmetic_operator_strategy, binary_arithmetic_operator_strategy
)
def test_homomorphism_preserving_is_operator(
    first: BinaryArithmeticOperator, second: BinaryArithmeticOperator
) -> None:
    first_result, second_result = (
        binary_arithmetic_operator_to_expression_kind(first),
        binary_arithmetic_operator_to_expression_kind(second),
    )

    assert equivalence(first is second, first_result is second_result)
