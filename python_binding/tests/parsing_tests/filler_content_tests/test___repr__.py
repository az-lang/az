from az import parsing, tokenization
from az.parsing import FillerContent
from hypothesis import given

from tests.strategies import filler_content_strategy


@given(filler_content_strategy)
def test_round_trip(filler_content: FillerContent) -> None:
    result = repr(filler_content)

    assert (
        eval(result, {**vars(parsing), **vars(tokenization)}) == filler_content
    )
