from hypothesis import given

from az import tokenization
from az.tokenization import ByteIndex

from . import strategies


@given(strategies.byte_indices)
def test_round_trip(byte_index: ByteIndex) -> None:
    result = repr(byte_index)

    round_tripped = eval(result, dict(vars(tokenization)))
    assert round_tripped is not byte_index
    assert round_tripped == byte_index
