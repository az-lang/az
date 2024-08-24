from az.parsing import Script
from az.tokenization import TokenCollection
from hypothesis import given

from tests.strategies import script_strategy


@given(script_strategy)
def test_basic(script: Script) -> None:
    result = script.tokenize()

    assert isinstance(result, TokenCollection)


@given(script_strategy)
def test_round_trip(script: Script) -> None:
    result = script.tokenize()

    assert Script.from_tokens(result) == script
