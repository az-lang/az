from az.parsing import Assignment, Expression, Filler
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    assignment_strategy,
    assignment_target_strategy,
    assignment_value_strategy,
    filler_list_strategy,
    substring_position_strategy,
)


@given(
    assignment_target_strategy,
    assignment_value_strategy,
    substring_position_strategy,
    filler_list_strategy,
)
def test_basic(
    target: Expression,
    value: Expression,
    operator_position: SubstringPosition,
    operator_fillers: list[Filler],
) -> None:
    result = Assignment(
        target,
        value,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )

    assert isinstance(result, Assignment)
    assert result.target == target
    assert result.value == value
    assert result.operator_position == operator_position
    assert result.operator_fillers == operator_fillers


@given(assignment_strategy)
def test_round_trip(assignment: Assignment) -> None:
    result = Assignment(
        assignment.target,
        assignment.value,
        operator_position=assignment.operator_position,
        operator_fillers=assignment.operator_fillers,
    )

    assert result == assignment
