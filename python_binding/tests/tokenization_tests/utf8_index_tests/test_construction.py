import pytest
from hypothesis import given

from az.tokenization import Utf8Index

from . import strategies


@given(strategies.utf8_indices_values)
def test_basic(value: int) -> None:
    result = Utf8Index(value)

    assert int(result) == value


@given(strategies.invalid_utf8_indices_values)
def test_invalid_value(value: int) -> None:
    with pytest.raises(OverflowError):
        Utf8Index(value)
