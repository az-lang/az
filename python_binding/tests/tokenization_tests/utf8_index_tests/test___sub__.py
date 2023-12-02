import pytest
from hypothesis import given

from az.tokenization import Utf8Index

from . import strategies


@given(strategies.subtractable_utf8_indices_pairs)
def test_basic(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    result = first - second

    assert isinstance(result, Utf8Index)


@given(strategies.subtractable_utf8_indices_pairs)
def test_commutative_case(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    if first == second:
        assert first - second == second - first
    else:
        with pytest.raises(OverflowError):
            first - second
            second - first


@given(strategies.utf8_indices, strategies.zero_utf8_indices)
def test_diagonal(utf8_index: Utf8Index, zero_utf8_index: Utf8Index) -> None:
    assert utf8_index - utf8_index == zero_utf8_index


@given(strategies.utf8_indices, strategies.zero_utf8_indices)
def test_right_neutral_element(
    utf8_index: Utf8Index, zero_utf8_index: Utf8Index
) -> None:
    assert utf8_index - zero_utf8_index == utf8_index


@given(strategies.overflowing_subtractition_utf8_indices_pairs)
def test_overflow(pair: tuple[Utf8Index, Utf8Index]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first - second
