from az.parsing import Expression
from hypothesis import given

from tests.strategies import expression_strategy
from tests.utils import equivalence, implication

from . import are_structurally_equivalent_expressions


@given(expression_strategy, expression_strategy)
def test_basic(first: Expression, second: Expression) -> None:
    result = are_structurally_equivalent_expressions(first, second)

    assert isinstance(result, bool)


@given(expression_strategy)
def test_reflexivity(expression: Expression) -> None:
    assert are_structurally_equivalent_expressions(expression, expression)


@given(expression_strategy, expression_strategy)
def test_symmetry(first: Expression, second: Expression) -> None:
    assert equivalence(
        are_structurally_equivalent_expressions(first, second),
        are_structurally_equivalent_expressions(second, first),
    )


@given(expression_strategy, expression_strategy, expression_strategy)
def test_transitivity(
    first: Expression, second: Expression, third: Expression
) -> None:
    assert implication(
        (
            are_structurally_equivalent_expressions(first, second)
            and are_structurally_equivalent_expressions(second, third)
        ),
        are_structurally_equivalent_expressions(first, third),
    )
