from az.parsing import Filler
from hypothesis import given

from tests.strategies import filler_strategy
from tests.utils import equivalence, implication


@given(filler_strategy)
def test_reflexivity(filler: Filler) -> None:
    assert filler == filler


@given(filler_strategy, filler_strategy)
def test_symmetry(first: Filler, second: Filler) -> None:
    assert equivalence(first == second, second == first)


@given(filler_strategy, filler_strategy, filler_strategy)
def test_transitivity(first: Filler, second: Filler, third: Filler) -> None:
    assert implication(first == second and second == third, first == third)


@given(filler_strategy, filler_strategy)
def test_alternatives(first: Filler, second: Filler) -> None:
    assert equivalence(first == second, first == second)
