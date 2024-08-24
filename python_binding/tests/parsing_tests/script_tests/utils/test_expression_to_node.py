from __future__ import annotations

from az.parsing import Expression
from hypothesis import given

from tests.parsing_tests.utils import are_structurally_equivalent_expressions
from tests.strategies import expression_strategy
from tests.utils import equivalence

from . import ExpressionNode, expression_to_node


@given(expression_strategy)
def test_basic(expression: Expression) -> None:
    result = expression_to_node(expression)

    assert isinstance(result, ExpressionNode)


@given(expression_strategy, expression_strategy)
def test_homomorphism_of_structural_equivalency_and_equal_to_operator(
    first: Expression, second: Expression
) -> None:
    first_result, second_result = (
        expression_to_node(first),
        expression_to_node(second),
    )

    assert equivalence(
        are_structurally_equivalent_expressions(first, second),
        first_result == second_result,
    )
