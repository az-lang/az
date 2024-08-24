from az.parsing import Filler, Script, Statement
from hypothesis import given

from tests.strategies import (
    filler_list_strategy,
    script_strategy,
    statement_list_strategy,
)


@given(statement_list_strategy, filler_list_strategy)
def test_basic(statements: list[Statement], fillers: list[Filler]) -> None:
    result = Script(statements, fillers=fillers)

    assert isinstance(result, Script)
    assert result.statements == statements
    assert result.fillers == fillers


@given(script_strategy)
def test_round_trip(script: Script) -> None:
    result = Script(script.statements, fillers=script.fillers)

    assert result == script
