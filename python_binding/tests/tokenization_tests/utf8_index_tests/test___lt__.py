from hypothesis import given

from az.tokenization import Utf8Index

from tests.utils import equivalence, implication
from . import strategies


@given(strategies.utf8_indices)
def test_irreflexivity(utf8_index: Utf8Index) -> None:
    assert not utf8_index < utf8_index


@given(strategies.utf8_indices, strategies.utf8_indices)
def test_asymmetry(first: Utf8Index, second: Utf8Index) -> None:
    assert implication(first < second, not second < first)


@given(
    strategies.utf8_indices, strategies.utf8_indices, strategies.utf8_indices
)
def test_transitivity(
    first: Utf8Index, second: Utf8Index, third: Utf8Index
) -> None:
    assert implication(first < second < third, first < third)


@given(strategies.utf8_indices, strategies.utf8_indices)
def test_alternatives(first: Utf8Index, second: Utf8Index) -> None:
    assert equivalence(first < second, first <= second and first != second)
    assert equivalence(first < second, first <= second and first != second)
    assert equivalence(first < second, second >= first and first != second)
    assert equivalence(first < second, second >= first and first != second)
    assert equivalence(first < second, second > first)
    assert equivalence(first < second, not second <= first)
    assert equivalence(first < second, not first >= second)
