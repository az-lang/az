from az.parsing import NumericLiteralType
from hypothesis import given

from tests.strategies import numeric_literal_value_kind_strategy
from tests.utils import pickling_round_trip


@given(numeric_literal_value_kind_strategy)
def test_round_trip(numeric_literal_type: NumericLiteralType) -> None:
    assert pickling_round_trip(numeric_literal_type) is numeric_literal_type
