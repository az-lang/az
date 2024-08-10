from az.parsing import FillerContent, FillerKind
from hypothesis import given

from tests.strategies import (
    filler_content_strategy,
    filler_kind_with_state_strategy,
)


@given(filler_kind_with_state_strategy)
def test_basic(filler_kind_with_state: tuple[FillerKind, str | None]) -> None:
    filler_kind, state = filler_kind_with_state

    result = FillerContent(filler_kind, state)

    assert isinstance(result, FillerContent)
    assert result.kind is filler_kind
    assert result.state == state


@given(filler_content_strategy)
def test_round_trip(filler_content: FillerContent) -> None:
    result = FillerContent(filler_content.kind, filler_content.state)

    assert result == filler_content
