from hypothesis import given

from az.tokenization import Utf8Index

from tests.utils import equivalence
from . import strategies


@given(strategies.utf8_indices)
def test_irreflexivity(utf8_index: Utf8Index) -> None:
    assert utf8_index == utf8_index


@given(strategies.utf8_indices, strategies.utf8_indices)
def test_symmetry(first: Utf8Index, second: Utf8Index) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.utf8_indices, strategies.utf8_indices)
def test_equivalents(first: Utf8Index, second: Utf8Index) -> None:
    assert equivalence(first != second, first != second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)
