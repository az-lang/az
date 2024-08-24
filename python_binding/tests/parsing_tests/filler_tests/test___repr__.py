from az import parsing, tokenization
from az.parsing import Filler
from hypothesis import given

from tests.strategies import filler_strategy


@given(filler_strategy)
def test_round_trip(filler: Filler) -> None:
    result = repr(filler)

    assert eval(result, {**vars(parsing), **vars(tokenization)}) == filler
