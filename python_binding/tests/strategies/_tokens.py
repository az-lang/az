from az.parsing import Script
from az.tokenization import (
    NumericLiteralValueKind,
    Token,
    TokenContent,
    TokenKind,
)
from hypothesis import strategies as _st

from tests.utils import pack

from ._positions import substring_position_strategy
from ._scripts import script_strategy
from ._token_content_strings import (
    comment_block_string_strategy,
    comment_line_string_strategy,
    floating_point_literal_value_strategy,
    identifier_token_string_strategy,
    integer_literal_value_strategy,
    whitespace_string_strategy,
)

numeric_literal_value_kind_strategy = _st.sampled_from(
    [NumericLiteralValueKind.INTEGER, NumericLiteralValueKind.FLOATING_POINT]
)
_floating_point_literal_types = [TokenKind.F32, TokenKind.F64]
_integer_literal_types = [
    TokenKind.I16,
    TokenKind.I32,
    TokenKind.I64,
    TokenKind.I8,
    TokenKind.ISIZE,
    TokenKind.U16,
    TokenKind.U32,
    TokenKind.U64,
    TokenKind.U8,
    TokenKind.USIZE,
]
token_kind_with_state_strategy = (
    _st.tuples(
        _st.sampled_from(
            [
                TokenKind.ARROW,
                TokenKind.ASSIGNMENT,
                TokenKind.ASTERISK,
                TokenKind.CLOSE_BRACE,
                TokenKind.CLOSE_PARENTHESIS,
                TokenKind.COLON,
                TokenKind.COMMA,
                TokenKind.DOT,
                TokenKind.EQUAL_TO,
                TokenKind.GREATER_THAN,
                TokenKind.GREATER_THAN_OR_EQUAL_TO,
                TokenKind.LESS_THAN,
                TokenKind.LESS_THAN_OR_EQUAL_TO,
                TokenKind.MINUS,
                TokenKind.NEWLINE,
                TokenKind.NOT_EQUAL_TO,
                TokenKind.OPEN_BRACE,
                TokenKind.OPEN_PARENTHESIS,
                TokenKind.PLUS,
                TokenKind.SEMICOLON,
                TokenKind.SLASH,
            ]
        ),
        _st.none(),
    )
    | _st.tuples(
        _st.just(TokenKind.COMMENT_LINE), comment_line_string_strategy
    )
    | _st.tuples(
        _st.just(TokenKind.COMMENT_BLOCK), comment_block_string_strategy
    )
    | _st.tuples(_st.just(TokenKind.WHITESPACE), whitespace_string_strategy)
    | _st.tuples(
        _st.just(TokenKind.IDENTIFIER), identifier_token_string_strategy
    )
    | _st.tuples(
        _st.sampled_from(_integer_literal_types),
        integer_literal_value_strategy,
    )
    | _st.tuples(
        _st.sampled_from(_floating_point_literal_types),
        floating_point_literal_value_strategy,
    )
)
token_content_strategy = _st.builds(
    pack(TokenContent), token_kind_with_state_strategy
)
token_kind_strategy = _st.sampled_from(
    [
        TokenKind.ARROW,
        TokenKind.ASSIGNMENT,
        TokenKind.ASTERISK,
        TokenKind.CLOSE_BRACE,
        TokenKind.CLOSE_PARENTHESIS,
        TokenKind.COLON,
        TokenKind.COMMA,
        TokenKind.COMMENT_BLOCK,
        TokenKind.COMMENT_LINE,
        TokenKind.DOT,
        TokenKind.EQUAL_TO,
        TokenKind.F32,
        TokenKind.F64,
        TokenKind.GREATER_THAN,
        TokenKind.GREATER_THAN_OR_EQUAL_TO,
        TokenKind.I8,
        TokenKind.I16,
        TokenKind.I32,
        TokenKind.I64,
        TokenKind.IDENTIFIER,
        TokenKind.ISIZE,
        TokenKind.LESS_THAN,
        TokenKind.LESS_THAN_OR_EQUAL_TO,
        TokenKind.MINUS,
        TokenKind.NEWLINE,
        TokenKind.NOT_EQUAL_TO,
        TokenKind.OPEN_BRACE,
        TokenKind.OPEN_PARENTHESIS,
        TokenKind.PLUS,
        TokenKind.SEMICOLON,
        TokenKind.SLASH,
        TokenKind.U8,
        TokenKind.U16,
        TokenKind.U32,
        TokenKind.U64,
        TokenKind.USIZE,
        TokenKind.WHITESPACE,
    ]
)
token_strategy = _st.builds(
    Token, token_content_strategy, position=substring_position_strategy
)
parseable_token_collection_strategy = script_strategy.map(Script.tokenize)
