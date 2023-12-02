import pytest
from hypothesis import given

from az.tokenization import ByteIndex

from . import strategies


@given(strategies.subtractable_byte_indices_pairs)
def test_basic(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    result = first - second

    assert isinstance(result, ByteIndex)


@given(strategies.subtractable_byte_indices_pairs)
def test_commutative_case(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    if first == second:
        assert first - second == second - first
    else:
        with pytest.raises(OverflowError):
            first - second
            second - first


@given(strategies.byte_indices, strategies.zero_byte_indices)
def test_diagonal(byte_index: ByteIndex, zero_byte_index: ByteIndex) -> None:
    assert byte_index - byte_index == zero_byte_index


@given(strategies.byte_indices, strategies.zero_byte_indices)
def test_right_neutral_element(
    byte_index: ByteIndex, zero_byte_index: ByteIndex
) -> None:
    assert byte_index - zero_byte_index == byte_index


@given(strategies.overflowing_subtractition_byte_indices_pairs)
def test_overflow(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first - second
