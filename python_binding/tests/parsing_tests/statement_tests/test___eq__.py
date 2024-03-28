from hypothesis import given

from az.parsing import Statement

from tests.utils import equivalence, implication
from . import strategies


@given(strategies.statements)
def test_reflexivity(statement: Statement) -> None:
    assert statement == statement


@given(strategies.statements, strategies.statements)
def test_symmetry(first: Statement, second: Statement) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.statements, strategies.statements, strategies.statements)
def test_transitivity(
    first: Statement, second: Statement, third: Statement
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.statements, strategies.statements)
def test_alternatives(first: Statement, second: Statement) -> None:
    assert equivalence(first == second, first == second)
