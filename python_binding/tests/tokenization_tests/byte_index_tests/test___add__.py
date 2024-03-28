import pytest
from hypothesis import given

from az.tokenization import ByteIndex

from . import strategies


@given(strategies.addable_byte_indices_pairs)
def test_basic(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    result = first + second

    assert isinstance(result, ByteIndex)


@given(strategies.addable_byte_indices_pairs)
def test_commutativity(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    assert first + second == second + first


@given(strategies.byte_indices, strategies.zero_byte_indices)
def test_neutral_element(first: ByteIndex, second: ByteIndex) -> None:
    assert first + second == first == second + first


@given(strategies.addable_byte_indices_triplets)
def test_associativity(
    triplet: tuple[ByteIndex, ByteIndex, ByteIndex],
) -> None:
    first, second, third = triplet

    assert (first + second) + third == first + (second + third)


@given(strategies.overflowing_addition_byte_indices_pairs)
def test_overflow(pair: tuple[ByteIndex, ByteIndex]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first + second
