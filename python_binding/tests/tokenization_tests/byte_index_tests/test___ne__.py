from hypothesis import given

from az.tokenization import ByteIndex

from tests.utils import equivalence
from . import strategies


@given(strategies.byte_indices)
def test_irreflexivity(byte_index: ByteIndex) -> None:
    assert byte_index == byte_index


@given(strategies.byte_indices, strategies.byte_indices)
def test_symmetry(first: ByteIndex, second: ByteIndex) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.byte_indices, strategies.byte_indices)
def test_equivalents(first: ByteIndex, second: ByteIndex) -> None:
    assert equivalence(first != second, first != second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)
