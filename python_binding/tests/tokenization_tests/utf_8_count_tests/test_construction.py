import pytest
from az.tokenization import Utf8Count
from hypothesis import given

from .strategies import (
    invalid_utf_8_count_value_strategy,
    utf_8_count_value_strategy,
)


@given(utf_8_count_value_strategy)
def test_basic(value: int) -> None:
    result = Utf8Count(value)

    assert isinstance(result, Utf8Count)
    assert int(result) == value


@given(invalid_utf_8_count_value_strategy)
def test_invalid_value(value: int) -> None:
    with pytest.raises(OverflowError):
        Utf8Count(value)
