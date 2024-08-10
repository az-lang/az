from az.parsing import Block, Expression, Filler, Statement
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    block_strategy,
    expression_strategy,
    filler_list_strategy,
    statement_list_strategy,
    substring_position_strategy,
)


@given(
    statement_list_strategy,
    expression_strategy,
    substring_position_strategy,
    substring_position_strategy,
    filler_list_strategy,
    filler_list_strategy,
)
def test_basic(
    statements: list[Statement],
    expression: Expression | None,
    open_brace_position: SubstringPosition,
    close_brace_position: SubstringPosition,
    open_brace_fillers: list[Filler],
    close_brace_fillers: list[Filler],
) -> None:
    result = Block(
        statements,
        expression,
        open_brace_position=open_brace_position,
        close_brace_position=close_brace_position,
        open_brace_fillers=open_brace_fillers,
        close_brace_fillers=close_brace_fillers,
    )

    assert isinstance(result, Block)
    assert result.statements == statements
    assert result.expression == expression
    assert result.open_brace_position == open_brace_position
    assert result.close_brace_position == close_brace_position
    assert result.open_brace_fillers == open_brace_fillers
    assert result.close_brace_fillers == close_brace_fillers


@given(block_strategy)
def test_round_trip(block: Block) -> None:
    result = Block(
        block.statements,
        block.expression,
        open_brace_position=block.open_brace_position,
        close_brace_position=block.close_brace_position,
        open_brace_fillers=block.open_brace_fillers,
        close_brace_fillers=block.close_brace_fillers,
    )

    assert result == block
