from az.parsing import Statement
from hypothesis import given

from tests.utils import equivalence, implication

from .strategies import statement_strategy


@given(statement_strategy)
def test_reflexivity(statement: Statement) -> None:
    assert statement == statement


@given(statement_strategy, statement_strategy)
def test_symmetry(first: Statement, second: Statement) -> None:
    assert equivalence(first == second, second == first)


@given(statement_strategy, statement_strategy, statement_strategy)
def test_transitivity(
    first: Statement, second: Statement, third: Statement
) -> None:
    assert implication(first == second and second == third, first == third)


@given(statement_strategy, statement_strategy)
def test_alternatives(first: Statement, second: Statement) -> None:
    assert equivalence(first == second, first == second)
