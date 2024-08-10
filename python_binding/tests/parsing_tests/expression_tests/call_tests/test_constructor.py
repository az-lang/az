from az.parsing import Call, Expression, Filler
from az.tokenization import SubstringPosition
from hypothesis import given

from .strategies import (
    argument_list_with_comma_position_list_and_comma_filler_list_strategy,
    call_strategy,
    callable_strategy,
    filler_list_strategy,
    substring_position_strategy,
)


@given(
    callable_strategy,
    argument_list_with_comma_position_list_and_comma_filler_list_strategy,
    substring_position_strategy,
    substring_position_strategy,
    filler_list_strategy,
    filler_list_strategy,
)
def test_basic(
    callable_: Expression,
    arguments_with_comma_positions_and_comma_fillers: tuple[
        list[Expression], list[SubstringPosition], list[list[Filler]]
    ],
    open_parenthesis_position: SubstringPosition,
    close_parenthesis_position: SubstringPosition,
    open_parenthesis_fillers: list[Filler],
    close_parenthesis_fillers: list[Filler],
) -> None:
    arguments, comma_positions, comma_fillers = (
        arguments_with_comma_positions_and_comma_fillers
    )

    result = Call(
        callable_,
        arguments,
        open_parenthesis_position=open_parenthesis_position,
        comma_positions=comma_positions,
        close_parenthesis_position=close_parenthesis_position,
        open_parenthesis_fillers=open_parenthesis_fillers,
        comma_fillers=comma_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
    )

    assert isinstance(result, Call)
    assert result.callable == callable_
    assert result.arguments == arguments
    assert result.open_parenthesis_position == open_parenthesis_position
    assert result.comma_positions == comma_positions
    assert result.close_parenthesis_position == close_parenthesis_position
    assert result.open_parenthesis_fillers == open_parenthesis_fillers
    assert result.comma_fillers == comma_fillers
    assert result.close_parenthesis_fillers == close_parenthesis_fillers


@given(call_strategy)
def test_round_trip(call: Call) -> None:
    result = Call(
        call.callable,
        call.arguments,
        open_parenthesis_position=call.open_parenthesis_position,
        comma_positions=call.comma_positions,
        close_parenthesis_position=call.close_parenthesis_position,
        open_parenthesis_fillers=call.open_parenthesis_fillers,
        comma_fillers=call.comma_fillers,
        close_parenthesis_fillers=call.close_parenthesis_fillers,
    )

    assert result == call
