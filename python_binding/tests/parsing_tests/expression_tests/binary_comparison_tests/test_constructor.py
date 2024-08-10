from az.parsing import (
    BinaryComparison,
    BinaryComparisonOperator,
    Expression,
    Filler,
)
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    binary_comparison_operator_with_operands_pair_strategy,
    binary_comparison_strategy,
    filler_list_strategy,
    substring_position_strategy,
)


@given(
    binary_comparison_operator_with_operands_pair_strategy,
    substring_position_strategy,
    filler_list_strategy,
)
def test_basic(
    operator_with_operands: tuple[
        BinaryComparisonOperator, tuple[Expression, Expression]
    ],
    operator_position: SubstringPosition,
    operator_fillers: list[Filler],
) -> None:
    operator, (left, right) = operator_with_operands

    result = BinaryComparison(
        left,
        right,
        operator,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )

    assert isinstance(result, BinaryComparison)
    assert result.left == left
    assert result.right == right
    assert result.operator is operator
    assert result.operator_position == operator_position
    assert result.operator_fillers == operator_fillers


@given(binary_comparison_strategy)
def test_round_trip(binary_comparison: BinaryComparison) -> None:
    result = BinaryComparison(
        binary_comparison.left,
        binary_comparison.right,
        binary_comparison.operator,
        operator_position=binary_comparison.operator_position,
        operator_fillers=binary_comparison.operator_fillers,
    )

    assert result == binary_comparison
