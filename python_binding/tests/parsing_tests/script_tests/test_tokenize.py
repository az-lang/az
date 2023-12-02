from hypothesis import given

from az.parsing import Script
from az.tokenization import Token

from . import strategies


@given(strategies.scripts)
def test_basic(script: Script) -> None:
    result = script.tokenize()

    assert isinstance(result, list)
    assert all(isinstance(element, Token) for element in result)


@given(strategies.scripts)
def test_round_trip(script: Script) -> None:
    result = script.tokenize()

    assert Script.from_tokens(result) == script
