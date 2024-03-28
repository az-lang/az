from hypothesis import given

from az import parsing, tokenization
from az.parsing import Statement

from . import strategies


@given(strategies.statements)
def test_round_trip(statement: Statement) -> None:
    result = repr(statement)

    assert eval(result, {**vars(parsing), **vars(tokenization)}) == statement
