from az.parsing import Filler, FillerContent
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    filler_content_strategy,
    filler_strategy,
    substring_position_strategy,
)


@given(filler_content_strategy, substring_position_strategy)
def test_basic(content: FillerContent, position: SubstringPosition) -> None:
    result = Filler(content, position=position)

    assert isinstance(result, Filler)
    assert result.content == content
    assert result.position == position


@given(filler_strategy)
def test_round_trip(filler: Filler) -> None:
    result = Filler(filler.content, position=filler.position)

    assert result == filler
