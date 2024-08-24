from az import tokenization
from az.tokenization import ByteCount
from hypothesis import given

from .strategies import byte_count_strategy


@given(byte_count_strategy)
def test_round_trip(byte_count: ByteCount) -> None:
    result = repr(byte_count)

    round_tripped = eval(result, dict(vars(tokenization)))
    assert round_tripped is not byte_count
    assert round_tripped == byte_count
