from __future__ import annotations

from az.parsing import NumericLiteralType
from hypothesis import given

from tests.strategies import numeric_literal_type_strategy
from tests.utils import equivalence

from . import ExpressionKind, numeric_literal_type_to_expression_kind


@given(numeric_literal_type_strategy)
def test_basic(numeric_literal_type: NumericLiteralType) -> None:
    result = numeric_literal_type_to_expression_kind(numeric_literal_type)

    assert isinstance(result, ExpressionKind)


@given(numeric_literal_type_strategy, numeric_literal_type_strategy)
def test_homomorphism_preserving_is_operator(
    first: NumericLiteralType, second: NumericLiteralType
) -> None:
    first_result, second_result = (
        numeric_literal_type_to_expression_kind(first),
        numeric_literal_type_to_expression_kind(second),
    )

    assert equivalence(first is second, first_result is second_result)
