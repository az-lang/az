from az.tokenization import Utf8Count
from hypothesis import given

from tests.utils import equivalence, implication

from .strategies import utf_8_count_strategy


@given(utf_8_count_strategy)
def test_irreflexivity(utf_8_count: Utf8Count) -> None:
    assert not utf_8_count < utf_8_count


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_asymmetry(first: Utf8Count, second: Utf8Count) -> None:
    assert implication(first < second, not second < first)


@given(utf_8_count_strategy, utf_8_count_strategy, utf_8_count_strategy)
def test_transitivity(
    first: Utf8Count, second: Utf8Count, third: Utf8Count
) -> None:
    assert implication(first < second < third, first < third)


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_alternatives(first: Utf8Count, second: Utf8Count) -> None:
    assert equivalence(first < second, first <= second and first != second)
    assert equivalence(first < second, first <= second and first != second)
    assert equivalence(first < second, second >= first and first != second)
    assert equivalence(first < second, second >= first and first != second)
    assert equivalence(first < second, second > first)
    assert equivalence(first < second, not second <= first)
    assert equivalence(first < second, not first >= second)
