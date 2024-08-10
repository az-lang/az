import pytest
from az.tokenization import Utf8Count
from hypothesis import given

from .strategies import (
    overflowing_subtraction_utf_8_count_pair_strategy,
    subtractable_utf_8_count_pair_strategy,
    utf_8_count_strategy,
    zero_utf_8_count_strategy,
)


@given(subtractable_utf_8_count_pair_strategy)
def test_basic(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    result = first - second

    assert isinstance(result, Utf8Count)


@given(subtractable_utf_8_count_pair_strategy)
def test_commutative_case(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    if first == second:
        assert first - second == second - first
    else:
        with pytest.raises(OverflowError):
            first - second
            second - first


@given(utf_8_count_strategy, zero_utf_8_count_strategy)
def test_diagonal(utf_8_count: Utf8Count, zero_utf_8_count: Utf8Count) -> None:
    assert utf_8_count - utf_8_count == zero_utf_8_count


@given(utf_8_count_strategy, zero_utf_8_count_strategy)
def test_right_neutral_element(
    utf_8_count: Utf8Count, zero_utf_8_count: Utf8Count
) -> None:
    assert utf_8_count - zero_utf_8_count == utf_8_count


@given(overflowing_subtraction_utf_8_count_pair_strategy)
def test_overflow(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first - second
