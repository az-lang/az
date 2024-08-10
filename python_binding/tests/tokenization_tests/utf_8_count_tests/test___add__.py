import pytest
from az.tokenization import Utf8Count
from hypothesis import given

from .strategies import (
    addable_utf_8_count_pair_strategy,
    addable_utf_8_count_triplet_strategy,
    overflowing_addition_utf_8_count_pair_strategy,
    utf_8_count_strategy,
    zero_utf_8_count_strategy,
)


@given(addable_utf_8_count_pair_strategy)
def test_basic(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    result = first + second

    assert isinstance(result, Utf8Count)


@given(addable_utf_8_count_pair_strategy)
def test_commutativity(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    assert first + second == second + first


@given(utf_8_count_strategy, zero_utf_8_count_strategy)
def test_neutral_element(first: Utf8Count, second: Utf8Count) -> None:
    assert first + second == first == second + first


@given(addable_utf_8_count_triplet_strategy)
def test_associativity(
    triplet: tuple[Utf8Count, Utf8Count, Utf8Count],
) -> None:
    first, second, third = triplet

    assert (first + second) + third == first + (second + third)


@given(overflowing_addition_utf_8_count_pair_strategy)
def test_overflow(pair: tuple[Utf8Count, Utf8Count]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first + second
