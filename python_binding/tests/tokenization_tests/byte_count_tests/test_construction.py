import pytest
from az.tokenization import ByteCount
from hypothesis import given

from .strategies import (
    byte_count_value_strategy,
    invalid_byte_count_value_strategy,
)


@given(byte_count_value_strategy)
def test_basic(value: int) -> None:
    result = ByteCount(value)

    assert isinstance(result, ByteCount)
    assert int(result) == value


@given(invalid_byte_count_value_strategy)
def test_invalid_value(value: int) -> None:
    with pytest.raises(OverflowError):
        ByteCount(value)
