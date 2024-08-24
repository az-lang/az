from az.parsing import (
    BinaryArithmeticOperation,
    BinaryArithmeticOperator,
    Expression,
    Filler,
)
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    binary_arithmetic_operation_strategy,
    binary_arithmetic_operator_with_operands_pair_strategy,
    filler_list_strategy,
    substring_position_strategy,
)


@given(
    binary_arithmetic_operator_with_operands_pair_strategy,
    substring_position_strategy,
    filler_list_strategy,
)
def test_basic(
    operator_with_operands_pair: tuple[
        BinaryArithmeticOperator, tuple[Expression, Expression]
    ],
    operator_position: SubstringPosition,
    operator_fillers: list[Filler],
) -> None:
    operator, (left, right) = operator_with_operands_pair

    result = BinaryArithmeticOperation(
        left,
        right,
        operator,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )

    assert isinstance(result, BinaryArithmeticOperation)
    assert result.left == left
    assert result.right == right
    assert result.operator is operator
    assert result.operator_position == operator_position
    assert result.operator_fillers == operator_fillers


@given(binary_arithmetic_operation_strategy)
def test_round_trip(
    binary_arithmetic_operation: BinaryArithmeticOperation,
) -> None:
    result = BinaryArithmeticOperation(
        binary_arithmetic_operation.left,
        binary_arithmetic_operation.right,
        binary_arithmetic_operation.operator,
        operator_position=binary_arithmetic_operation.operator_position,
        operator_fillers=binary_arithmetic_operation.operator_fillers,
    )

    assert result == binary_arithmetic_operation
