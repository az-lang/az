from az import parsing, tokenization
from az.parsing import Expression
from hypothesis import given

from tests.strategies import expression_strategy


@given(expression_strategy)
def test_round_trip(expression: Expression) -> None:
    result = repr(expression)

    assert eval(result, {**vars(parsing), **vars(tokenization)}) == expression
