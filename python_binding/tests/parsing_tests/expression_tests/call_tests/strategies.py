from az.parsing import Expression, Filler
from az.tokenization import SubstringPosition
from hypothesis import strategies as _st

from tests import strategies as _strategies


@_st.composite
def _to_argument_lists_with_comma_position_lists_and_comma_filler_lists(
    draw: _st.DrawFn,
    argument_lists: _st.SearchStrategy[list[Expression]],
    filler_lists: _st.SearchStrategy[list[Filler]],
    substring_positions: _st.SearchStrategy[SubstringPosition],
) -> tuple[list[Expression], list[SubstringPosition], list[list[Filler]]]:
    arguments = draw(argument_lists)
    comma_positions = draw(
        _st.lists(
            substring_positions,
            min_size=max(len(arguments) - 1, 0),
            max_size=len(arguments),
        )
    )
    comma_fillers = draw(
        _st.lists(
            filler_lists,
            min_size=len(comma_positions),
            max_size=len(comma_positions),
        )
    )
    return arguments, comma_positions, comma_fillers


argument_list_with_comma_position_list_and_comma_filler_list_strategy = (
    _to_argument_lists_with_comma_position_lists_and_comma_filler_lists(
        _strategies.expression_list_strategy,
        _strategies.filler_list_strategy,
        _strategies.substring_position_strategy,
    )
)
callable_strategy = _strategies.callable_strategy
call_strategy = _strategies.call_strategy
filler_list_strategy = _strategies.filler_list_strategy
substring_position_strategy = _strategies.substring_position_strategy
