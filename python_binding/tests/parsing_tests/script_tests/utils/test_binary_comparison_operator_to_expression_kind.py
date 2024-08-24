from __future__ import annotations

from az.parsing import BinaryComparisonOperator
from hypothesis import given

from tests.strategies import binary_comparison_operator_strategy
from tests.utils import equivalence

from . import ExpressionKind, binary_comparison_operator_to_expression_kind


@given(binary_comparison_operator_strategy)
def test_basic(binary_comparison_operator: BinaryComparisonOperator) -> None:
    result = binary_comparison_operator_to_expression_kind(
        binary_comparison_operator
    )

    assert isinstance(result, ExpressionKind)


@given(
    binary_comparison_operator_strategy, binary_comparison_operator_strategy
)
def test_homomorphism_preserving_is_operator(
    first: BinaryComparisonOperator, second: BinaryComparisonOperator
) -> None:
    first_result, second_result = (
        binary_comparison_operator_to_expression_kind(first),
        binary_comparison_operator_to_expression_kind(second),
    )

    assert equivalence(first is second, first_result is second_result)
