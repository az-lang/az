import pytest
from hypothesis import given

from az.tokenization import ByteIndex

from . import strategies


@given(strategies.byte_indices_values)
def test_basic(value: int) -> None:
    result = ByteIndex(value)

    assert int(result) == value


@given(strategies.invalid_byte_indices_values)
def test_invalid_value(value: int) -> None:
    with pytest.raises(OverflowError):
        ByteIndex(value)
