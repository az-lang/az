from hypothesis import given

from az import tokenization
from az.tokenization import Utf8Index

from . import strategies


@given(strategies.utf8_indices)
def test_round_trip(utf8_index: Utf8Index) -> None:
    result = repr(utf8_index)

    round_tripped = eval(result, dict(vars(tokenization)))
    assert round_tripped is not utf8_index
    assert round_tripped == utf8_index
