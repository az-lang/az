from __future__ import annotations

import copy

from az.parsing import Script
from hypothesis import given

from tests.strategies import script_strategy

from .utils import script_to_ast


@given(script_strategy)
def test_basic(script: Script) -> None:
    result = script.format()  # type: ignore[func-returns-value]

    assert result is None


@given(script_strategy)
def test_idempotence(script: Script) -> None:
    script.format()
    script_copy = copy.deepcopy(script)

    script.format()

    assert script == script_copy


@given(script_strategy)
def test_preserving_of_ast(script: Script) -> None:
    ast_before = script_to_ast(script)

    script.format()

    ast_after = script_to_ast(script)

    assert ast_after == ast_before
