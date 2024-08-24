from az.parsing import Expression
from hypothesis import given

from tests.strategies import expression_strategy
from tests.utils import equivalence, implication


@given(expression_strategy)
def test_reflexivity(expression: Expression) -> None:
    assert expression == expression


@given(expression_strategy, expression_strategy)
def test_symmetry(first: Expression, second: Expression) -> None:
    assert equivalence(first == second, second == first)


@given(expression_strategy, expression_strategy, expression_strategy)
def test_transitivity(
    first: Expression, second: Expression, third: Expression
) -> None:
    assert implication(first == second and second == third, first == third)


@given(expression_strategy, expression_strategy)
def test_alternatives(first: Expression, second: Expression) -> None:
    assert equivalence(first == second, first == second)
