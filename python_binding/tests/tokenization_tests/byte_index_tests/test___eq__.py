from hypothesis import given

from az.tokenization import ByteIndex

from tests.utils import equivalence, implication
from . import strategies


@given(strategies.byte_indices)
def test_reflexivity(byte_index: ByteIndex) -> None:
    assert byte_index == byte_index


@given(strategies.byte_indices, strategies.byte_indices)
def test_symmetry(first: ByteIndex, second: ByteIndex) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.byte_indices, strategies.byte_indices, strategies.byte_indices
)
def test_transitivity(
    first: ByteIndex, second: ByteIndex, third: ByteIndex
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.byte_indices, strategies.byte_indices)
def test_alternatives(first: ByteIndex, second: ByteIndex) -> None:
    assert equivalence(first == second, first == second)
    assert equivalence(first == second, first >= second and first <= second)
    assert equivalence(first == second, first >= second and second >= first)
    assert equivalence(first == second, second <= first and second >= first)
    assert equivalence(first == second, second <= first and first <= second)
