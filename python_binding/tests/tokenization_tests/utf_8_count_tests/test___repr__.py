from az import tokenization
from az.tokenization import Utf8Count
from hypothesis import given

from .strategies import utf_8_count_strategy


@given(utf_8_count_strategy)
def test_round_trip(utf_8_count: Utf8Count) -> None:
    result = repr(utf_8_count)

    round_tripped = eval(result, dict(vars(tokenization)))
    assert round_tripped is not utf_8_count
    assert round_tripped == utf_8_count
