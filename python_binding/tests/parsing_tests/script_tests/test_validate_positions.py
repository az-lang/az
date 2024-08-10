from az.parsing import Script
from az.tokenization import PositionsValidationError, TokenCollection
from hypothesis import given

from tests.strategies import script_strategy
from tests.tokenization_tests.utils import tokens_to_string


@given(script_strategy)
def test_basic(script: Script) -> None:
    try:
        result = script.validate_positions()  # type: ignore[func-returns-value]
    except Exception as error:
        assert isinstance(error, ExceptionGroup)
        assert [
            sub_error
            for sub_error in error.exceptions
            if not isinstance(sub_error, PositionsValidationError)
        ] == []
    else:
        assert result is None


@given(script_strategy)
def test_reset_positions(script: Script) -> None:
    script.reset_positions()

    script.validate_positions()


@given(script_strategy)
def test_round_trip(script: Script) -> None:
    Script.from_tokens(
        TokenCollection.from_string(tokens_to_string(script.tokenize()))
    ).validate_positions()
