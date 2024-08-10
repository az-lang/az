from az.tokenization import NumericLiteralValueKind
from hypothesis import given

from tests.strategies import numeric_literal_value_kind_strategy


@given(numeric_literal_value_kind_strategy)
def test_round_trip(
    numeric_literal_value_kind: NumericLiteralValueKind,
) -> None:
    result = repr(numeric_literal_value_kind)

    assert eval(result) is numeric_literal_value_kind
