from az.parsing import NumericLiteralType
from hypothesis import given

from tests.strategies import numeric_literal_type_strategy


@given(numeric_literal_type_strategy)
def test_round_trip(numeric_literal_type: NumericLiteralType) -> None:
    result = repr(numeric_literal_type)

    assert eval(result) is numeric_literal_type
