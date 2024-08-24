from __future__ import annotations

from az.parsing import (
    Filler,
    Identifier,
    NumericLiteral,
    NumericLiteralType,
    is_keyword,
)
from az.tokenization import SubstringPosition
from hypothesis import strategies as _st

from ._fillers import filler_list_strategy
from ._positions import substring_position_strategy
from ._token_content_strings import (
    floating_point_literal_value_strategy,
    identifier_token_string_strategy,
    integer_literal_value_strategy,
)


def to_validated_identifier(
    value: str, /, *, position: SubstringPosition, fillers: list[Filler]
) -> Identifier:
    result = Identifier(value, position=position, fillers=fillers)
    result.validate_contents()
    return result


identifier_string_strategy = identifier_token_string_strategy.filter(
    lambda candidate: not is_keyword(candidate)
)
identifier_strategy = _st.builds(
    to_validated_identifier,
    identifier_string_strategy,
    position=substring_position_strategy,
    fillers=filler_list_strategy,
)


def to_validated_numeric_literal(
    value: str,
    type_: NumericLiteralType,
    /,
    *,
    position: SubstringPosition,
    fillers: list[Filler],
) -> NumericLiteral:
    result = NumericLiteral(value, type_, position=position, fillers=fillers)
    result.validate_contents()
    return result


_floating_point_literal_types = [
    NumericLiteralType.F32,
    NumericLiteralType.F64,
]
_integer_literal_types = [
    NumericLiteralType.I16,
    NumericLiteralType.I32,
    NumericLiteralType.I64,
    NumericLiteralType.I8,
    NumericLiteralType.ISIZE,
    NumericLiteralType.U16,
    NumericLiteralType.U32,
    NumericLiteralType.U64,
    NumericLiteralType.U8,
    NumericLiteralType.USIZE,
]

numeric_literal_strategy = _st.builds(
    to_validated_numeric_literal,
    integer_literal_value_strategy,
    _st.sampled_from(_integer_literal_types),
    fillers=filler_list_strategy,
    position=substring_position_strategy,
) | _st.builds(
    to_validated_numeric_literal,
    floating_point_literal_value_strategy,
    _st.sampled_from(_floating_point_literal_types),
    fillers=filler_list_strategy,
    position=substring_position_strategy,
)
_numeric_literal_types = _floating_point_literal_types + _integer_literal_types
numeric_literal_type_strategy = _st.sampled_from(_numeric_literal_types)
