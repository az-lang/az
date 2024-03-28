from __future__ import annotations

import typing as _t

if _t.TYPE_CHECKING:
    # classes

    @_t.final
    class ByteIndex:
        MAX: _t.ClassVar[_t.Self]

        def __init__(self, _value: int | None = None, /) -> None: ...

        @_t.overload
        def __add__(self, other: _t.Self) -> _t.Self: ...

        @_t.overload
        def __add__(self, other: _t.Any) -> _t.Any: ...

        def __add__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __eq__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __eq__(self, other: _t.Any) -> _t.Any: ...

        def __eq__(self, other: _t.Any) -> _t.Any: ...

        def __ge__(self, other: _t.Self) -> bool: ...

        def __gt__(self, other: _t.Self) -> bool: ...

        def __int__(self) -> int: ...

        def __le__(self, other: _t.Self) -> bool: ...

        def __lt__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __sub__(self, other: _t.Self) -> _t.Self: ...

        @_t.overload
        def __sub__(self, other: _t.Any) -> _t.Any: ...

        def __sub__(self, other: _t.Any) -> _t.Any: ...

    @_t.final
    class Utf8Index:
        MAX: _t.ClassVar[_t.Self]

        def __init__(self, _value: int | None = None, /) -> None: ...

        @_t.overload
        def __add__(self, other: _t.Self) -> _t.Self: ...

        @_t.overload
        def __add__(self, other: _t.Any) -> _t.Any: ...

        def __add__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __eq__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __eq__(self, other: _t.Any) -> _t.Any: ...

        def __eq__(self, other: _t.Any) -> _t.Any: ...

        def __ge__(self, other: _t.Self) -> bool: ...

        def __gt__(self, other: _t.Self) -> bool: ...

        def __int__(self) -> int: ...

        def __le__(self, other: _t.Self) -> bool: ...

        def __lt__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __sub__(self, other: _t.Self) -> _t.Self: ...

        @_t.overload
        def __sub__(self, other: _t.Any) -> _t.Any: ...

        def __sub__(self, other: _t.Any) -> _t.Any: ...

    @_t.final
    class CharacterPosition:
        byte: ByteIndex
        utf_8: Utf8Index

        def __init__(self, *, byte: ByteIndex, utf_8: Utf8Index) -> None: ...

    @_t.final
    class SubstringPosition:
        start_line: int
        start_character: CharacterPosition
        end_line: int
        end_character: CharacterPosition

        def __init__(
            self,
            *,
            start_line: int,
            start_character: CharacterPosition,
            end_line: int,
            end_character: CharacterPosition,
        ) -> None: ...

    @_t.final
    class Token:
        content: TokenContent
        position: SubstringPosition

        def __init__(
            self, *, content: TokenContent, position: SubstringPosition
        ) -> None: ...

    @_t.final
    class TokenContent:
        kind: TokenKind
        string: str

        def __init__(self, kind: TokenKind, string: str, /) -> None: ...

    # enumerations

    @_t.final
    class NumericLiteralValueKind:
        FLOATING_POINT: _t.ClassVar[_t.Self]
        INTEGER: _t.ClassVar[_t.Self]

    @_t.final
    class TokenKind:
        ARROW: _t.ClassVar[_t.Self]
        ASSIGNMENT: _t.ClassVar[_t.Self]
        ASTERISK: _t.ClassVar[_t.Self]
        CLOSE_BRACE: _t.ClassVar[_t.Self]
        CLOSE_PARENTHESIS: _t.ClassVar[_t.Self]
        COLON: _t.ClassVar[_t.Self]
        COMMA: _t.ClassVar[_t.Self]
        COMMENT_BLOCK: _t.ClassVar[_t.Self]
        COMMENT_LINE: _t.ClassVar[_t.Self]
        DOT: _t.ClassVar[_t.Self]
        EQUAL_TO: _t.ClassVar[_t.Self]
        F32: _t.ClassVar[_t.Self]
        F64: _t.ClassVar[_t.Self]
        GREATER_THAN: _t.ClassVar[_t.Self]
        GREATER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        I8: _t.ClassVar[_t.Self]
        I16: _t.ClassVar[_t.Self]
        I32: _t.ClassVar[_t.Self]
        I64: _t.ClassVar[_t.Self]
        IDENTIFIER: _t.ClassVar[_t.Self]
        LOWER_THAN: _t.ClassVar[_t.Self]
        LOWER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        MINUS: _t.ClassVar[_t.Self]
        NEWLINE: _t.ClassVar[_t.Self]
        NOT_EQUAL_TO: _t.ClassVar[_t.Self]
        OPEN_BRACE: _t.ClassVar[_t.Self]
        OPEN_PARENTHESIS: _t.ClassVar[_t.Self]
        PLUS: _t.ClassVar[_t.Self]
        SEMICOLON: _t.ClassVar[_t.Self]
        SLASH: _t.ClassVar[_t.Self]
        U8: _t.ClassVar[_t.Self]
        U16: _t.ClassVar[_t.Self]
        U32: _t.ClassVar[_t.Self]
        U64: _t.ClassVar[_t.Self]
        WHITESPACE: _t.ClassVar[_t.Self]

    # exceptions

    class LexicalError(Exception):
        pass

    @_t.final
    class CommentBlockIncomplete(LexicalError):
        position: SubstringPosition
        strings: list[str]

        def __init__(
            self, position: SubstringPosition, strings: list[str], /
        ) -> None: ...

    @_t.final
    class IdentifierIncomplete(LexicalError):
        position: SubstringPosition
        string: str

        def __init__(
            self, position: SubstringPosition, string: str, /
        ) -> None: ...

    @_t.final
    class IdentifierUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str

        def __init__(
            self,
            character: str,
            expected: str,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralTypeSuffixIncomplete(LexicalError):
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralTypeSuffixUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            character: str,
            expected: str,
            position: SubstringPosition,
            string: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralTypeSuffixUnknown(LexicalError):
        position: SubstringPosition
        string: str
        type_suffix: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            type_suffix: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralValueIncomplete(LexicalError):
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

        def __init__(
            self,
            kind: NumericLiteralValueKind,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralValueTypeSuffixConflict(LexicalError):
        position: SubstringPosition
        string: str
        type_suffix: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            type_suffix: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_t.final
    class NumericLiteralValueUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

        def __init__(
            self,
            character: str,
            expected: str,
            kind: NumericLiteralValueKind,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_t.final
    class UnexpectedCharacter(LexicalError):
        character: str
        position: SubstringPosition
        string: str

        def __init__(
            self, character: str, position: SubstringPosition, string: str, /
        ) -> None: ...

    # functions

    def tokenize_string(string: str) -> list[Token]: ...

else:
    from . import _az

    # classes
    ByteIndex = _az.ByteIndex
    CharacterPosition = _az.CharacterPosition
    SubstringPosition = _az.SubstringPosition
    Token = _az.Token
    TokenContent = _az.TokenContent
    Utf8Index = _az.Utf8Index

    # enumerations
    NumericLiteralValueKind = _az.NumericLiteralValueKind
    TokenKind = _az.TokenKind

    # exceptions
    LexicalError = _az.LexicalError
    CommentBlockIncomplete = _az.CommentBlockIncomplete
    IdentifierIncomplete = _az.IdentifierIncomplete
    IdentifierUnexpectedCharacter = _az.IdentifierUnexpectedCharacter
    NumericLiteralTypeSuffixIncomplete = _az.NumericLiteralTypeSuffixIncomplete
    NumericLiteralTypeSuffixUnexpectedCharacter = (
        _az.NumericLiteralTypeSuffixUnexpectedCharacter
    )
    NumericLiteralTypeSuffixUnknown = _az.NumericLiteralTypeSuffixUnknown
    NumericLiteralValueIncomplete = _az.NumericLiteralValueIncomplete
    NumericLiteralValueTypeSuffixConflict = (
        _az.NumericLiteralValueTypeSuffixConflict
    )
    NumericLiteralValueUnexpectedCharacter = (
        _az.NumericLiteralValueUnexpectedCharacter
    )
    UnexpectedCharacter = _az.UnexpectedCharacter

    # functions
    tokenize_string = _az.tokenize_string

    del _az
