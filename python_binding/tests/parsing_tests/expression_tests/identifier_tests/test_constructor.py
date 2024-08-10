from az.parsing import Filler, Identifier
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    filler_list_strategy,
    identifier_strategy,
    identifier_string_strategy,
    substring_position_strategy,
)


@given(
    identifier_string_strategy,
    substring_position_strategy,
    filler_list_strategy,
)
def test_basic(
    string: str, position: SubstringPosition, fillers: list[Filler]
) -> None:
    result = Identifier(string, position=position, fillers=fillers)

    assert isinstance(result, Identifier)
    assert result.string == string
    assert result.position == position
    assert result.fillers == fillers


@given(identifier_strategy)
def test_round_trip(identifier: Identifier) -> None:
    result = Identifier(
        identifier.string,
        position=identifier.position,
        fillers=identifier.fillers,
    )

    assert result == identifier
