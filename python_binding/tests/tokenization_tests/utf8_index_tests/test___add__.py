import pytest
from hypothesis import given

from az.tokenization import Utf8Index

from . import strategies


@given(strategies.addable_utf8_indices_pairs)
def test_basic(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    result = first + second

    assert isinstance(result, Utf8Index)


@given(strategies.addable_utf8_indices_pairs)
def test_commutativity(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    assert first + second == second + first


@given(strategies.utf8_indices, strategies.zero_utf8_indices)
def test_neutral_element(first: Utf8Index, second: Utf8Index) -> None:
    assert first + second == first == second + first


@given(strategies.addable_utf8_indices_triplets)
def test_associativity(
    triplet: tuple[Utf8Index, Utf8Index, Utf8Index],
) -> None:
    first, second, third = triplet

    assert (first + second) + third == first + (second + third)


@given(strategies.overflowing_addition_utf8_indices_pairs)
def test_overflow(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first + second
