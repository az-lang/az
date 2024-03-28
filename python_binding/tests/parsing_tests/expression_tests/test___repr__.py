from hypothesis import given

from az import parsing, tokenization
from az.parsing import Expression

from . import strategies


@given(strategies.expressions)
def test_round_trip(expression: Expression) -> None:
    result = repr(expression)

    assert eval(result, {**vars(parsing), **vars(tokenization)}) == expression
