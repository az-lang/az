from az.parsing import FillerContent
from hypothesis import given

from tests.strategies import filler_content_strategy
from tests.utils import equivalence, implication


@given(filler_content_strategy)
def test_reflexivity(filler_content: FillerContent) -> None:
    assert filler_content == filler_content


@given(filler_content_strategy, filler_content_strategy)
def test_symmetry(first: FillerContent, second: FillerContent) -> None:
    assert equivalence(first == second, second == first)


@given(
    filler_content_strategy, filler_content_strategy, filler_content_strategy
)
def test_transitivity(
    first: FillerContent, second: FillerContent, third: FillerContent
) -> None:
    assert implication(first == second and second == third, first == third)


@given(filler_content_strategy, filler_content_strategy)
def test_alternatives(first: FillerContent, second: FillerContent) -> None:
    assert equivalence(first == second, first == second)
