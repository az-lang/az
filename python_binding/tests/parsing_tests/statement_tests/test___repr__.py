from az import parsing, tokenization
from az.parsing import Statement
from hypothesis import given

from .strategies import statement_strategy


@given(statement_strategy)
def test_round_trip(statement: Statement) -> None:
    result = repr(statement)

    assert eval(result, {**vars(parsing), **vars(tokenization)}) == statement
