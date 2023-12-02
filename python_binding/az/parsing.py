from __future__ import annotations

import typing as _t
from abc import ABC as _ABC
from abc import abstractmethod as _abstractmethod

from .tokenization import SubstringPosition as _SubstringPosition
from .tokenization import Token as _Token

if _t.TYPE_CHECKING:
    # classes

    @_t.final
    class Precedence:
        MIN: _t.ClassVar[int]

        @_t.overload
        def __eq__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __eq__(self, other: _t.Any) -> _t.Any: ...

        def __eq__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __ge__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __ge__(self, other: _t.Any) -> _t.Any: ...

        def __ge__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __gt__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __gt__(self, other: _t.Any) -> _t.Any: ...

        def __gt__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __le__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __le__(self, other: _t.Any) -> _t.Any: ...

        def __le__(self, other: _t.Any) -> _t.Any: ...

        @_t.overload
        def __lt__(self, other: _t.Self) -> bool: ...

        @_t.overload
        def __lt__(self, other: _t.Any) -> _t.Any: ...

        def __lt__(self, other: _t.Any) -> _t.Any: ...

    @_t.final
    class BinaryAdditionOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryDivisionOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryMultiplicationOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinarySubtractionOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    BinaryArithmeticOperator = (
        type[BinaryAdditionOperator]
        | type[BinaryDivisionOperator]
        | type[BinaryMultiplicationOperator]
        | type[BinarySubtractionOperator]
    )

    @_t.final
    class BinaryAnnotationOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryAssignmentOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class CallOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class MemberAccessOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryEqualToOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryGreaterThanOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryGreaterThanOrEqualToOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryLowerThanOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryLowerThanOrEqualToOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    @_t.final
    class BinaryNotEqualToOperator:
        ASSOCIATIVITY: _t.ClassVar[Associativity]
        PRECEDENCE: _t.ClassVar[Precedence]

    BinaryComparisonOperator = (
        type[BinaryEqualToOperator]
        | type[BinaryGreaterThanOperator]
        | type[BinaryGreaterThanOrEqualToOperator]
        | type[BinaryLowerThanOperator]
        | type[BinaryLowerThanOrEqualToOperator]
        | type[BinaryNotEqualToOperator]
    )

    @_t.final
    class UnaryNegationOperator:
        PRECEDENCE: _t.ClassVar[Precedence]

    UnaryArithmeticOperator = type[UnaryNegationOperator]

    @_t.final
    class Filler:
        content: FillerContent
        position: _SubstringPosition

        def __init__(
            self, *, content: FillerContent, position: _SubstringPosition
        ) -> None: ...

    @_t.final
    class FillerContent:
        kind: FillerKind
        string: str

        def __init__(self, kind: FillerKind, string: str, /) -> None: ...

    class Expression(_ABC):
        @_abstractmethod
        def __init__(self) -> None: ...

    @_t.final
    class AnnotatedIdentifier(Expression):
        identifier: Identifier
        annotation: Expression
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            identifier: Identifier,
            annotation: Expression,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Assignment(Expression):
        target: Expression
        value: Expression
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            target: Expression,
            value: Expression,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class BinaryArithmeticOperation(Expression):
        left: Expression
        right: Expression
        operator: BinaryArithmeticOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            left: Expression,
            right: Expression,
            operator: BinaryArithmeticOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Block(Expression):
        statements: list[Statement]
        expression: Expression | None
        open_brace_position: _SubstringPosition
        close_brace_position: _SubstringPosition
        open_brace_fillers: list[Filler]
        close_brace_fillers: list[Filler]

        def __init__(
            self,
            statements: list[Statement],
            expression: Expression | None,
            /,
            *,
            open_brace_position: _SubstringPosition,
            close_brace_position: _SubstringPosition,
            open_brace_fillers: list[Filler],
            close_brace_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Call(Expression):
        callable: Expression
        arguments: list[Expression]
        open_parenthesis_position: _SubstringPosition
        commas_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        commas_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]

        def __init__(
            self,
            callable: Expression,
            arguments: list[Expression],
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            commas_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            commas_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class BinaryComparison(Expression):
        left: Expression
        right: Expression
        operator: BinaryComparisonOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            left: Expression,
            right: Expression,
            operator: BinaryComparisonOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Conditional(Expression):
        antecedent: Expression
        consequent: Block
        alternative: Expression | None
        opener_position: _SubstringPosition
        alternative_opener_position: _SubstringPosition | None
        opener_fillers: list[Filler]
        alternative_opener_fillers: list[Filler]

        def __init__(
            self,
            antecedent: Expression,
            consequent: Block,
            alternative: Expression | None,
            /,
            *,
            opener_position: _SubstringPosition,
            alternative_opener_position: _SubstringPosition | None,
            opener_fillers: list[Filler],
            alternative_opener_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class FunctionDefinition(Expression):
        parameters: list[AnnotatedIdentifier]
        return_type: Expression
        body: Block
        opener_position: _SubstringPosition
        open_parenthesis_position: _SubstringPosition
        commas_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        arrow_position: _SubstringPosition
        opener_fillers: list[Filler]
        open_parenthesis_fillers: list[Filler]
        commas_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]
        arrow_fillers: list[Filler]

        def __init__(
            self,
            parameters: list[AnnotatedIdentifier],
            return_type: Expression,
            body: Block,
            /,
            *,
            opener_position: _SubstringPosition,
            open_parenthesis_position: _SubstringPosition,
            commas_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            arrow_position: _SubstringPosition,
            opener_fillers: list[Filler],
            open_parenthesis_fillers: list[Filler],
            commas_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
            arrow_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Grouping(Expression):
        expression: Expression
        open_parenthesis_position: _SubstringPosition
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        close_parenthesis_fillers: list[Filler]

        def __init__(
            self,
            expression: Expression,
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Identifier(Expression):
        string: str
        position: _SubstringPosition
        fillers: list[Filler]

        def __init__(
            self,
            string: str,
            /,
            *,
            position: _SubstringPosition,
            fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class MemberAccess(Expression):
        object: Expression
        member: Identifier
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            object: Expression,
            member: Identifier,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class NumericLiteral(Expression):
        value: str
        type_: NumericLiteralType
        position: _SubstringPosition
        fillers: list[Filler]

        def __init__(
            self,
            value: str,
            type_: NumericLiteralType,
            /,
            *,
            position: _SubstringPosition,
            fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Tuple(Expression):
        elements: list[Expression]
        open_parenthesis_position: _SubstringPosition
        commas_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        commas_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]

        def __init__(
            self,
            elements: list[Expression],
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            commas_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            commas_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class UnaryArithmeticOperation(Expression):
        operand: Expression
        operator: UnaryArithmeticOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def __init__(
            self,
            operand: Expression,
            operator: UnaryArithmeticOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_t.final
    class Script:
        statements: list[Statement]
        fillers: list[Filler]

        @classmethod
        def from_tokens(cls, tokens: list[_Token]) -> _t.Self: ...

        def tokenize(self) -> list[_Token]: ...

        def __init__(
            self, statements: list[Statement], /, *, fillers: list[Filler]
        ) -> None: ...

    class Statement(_ABC):
        @_abstractmethod
        def __init__(self) -> None: ...

    @_t.final
    class ExpressionStatement(Statement):
        expression: Expression
        semicolon_position: _SubstringPosition
        semicolon_fillers: list[Filler]

        def __init__(
            self,
            expression: Expression,
            /,
            *,
            semicolon_position: _SubstringPosition,
            semicolon_fillers: list[Filler],
        ) -> None: ...

    # enumerations

    @_t.final
    class Associativity:
        LEFT_TO_RIGHT: _t.ClassVar[_t.Self]
        RIGHT_TO_LEFT: _t.ClassVar[_t.Self]

    @_t.final
    class FillerKind:
        COMMENT_BLOCK: _t.ClassVar[_t.Self]
        COMMENT_LINE: _t.ClassVar[_t.Self]
        NEWLINE: _t.ClassVar[_t.Self]
        WHITESPACE: _t.ClassVar[_t.Self]

    @_t.final
    class NumericLiteralType:
        F32: _t.ClassVar[_t.Self]
        F64: _t.ClassVar[_t.Self]
        I8: _t.ClassVar[_t.Self]
        I16: _t.ClassVar[_t.Self]
        I32: _t.ClassVar[_t.Self]
        I64: _t.ClassVar[_t.Self]
        ISIZE: _t.ClassVar[_t.Self]
        U8: _t.ClassVar[_t.Self]
        U16: _t.ClassVar[_t.Self]
        U32: _t.ClassVar[_t.Self]
        U64: _t.ClassVar[_t.Self]
        USIZE: _t.ClassVar[_t.Self]

    # exceptions

    class ParsingError(Exception):
        pass

    @_t.final
    class MismatchedOpenBrace(ParsingError):
        position: _SubstringPosition

        def __init__(self, position: _SubstringPosition, /) -> None: ...

    @_t.final
    class MismatchedOpenParenthesis(ParsingError):
        position: _SubstringPosition

        def __init__(self, position: _SubstringPosition, /) -> None: ...

    @_t.final
    class MissingSemicolon(ParsingError):
        token: _Token

        def __init__(self, token: _Token, /) -> None: ...

    @_t.final
    class OutOfTokens(ParsingError):
        def __init__(self, /) -> None: ...

    @_t.final
    class UnexpectedExpression(ParsingError):
        expression: Expression

        def __init__(self, expression: Expression, /) -> None: ...

    @_t.final
    class UnexpectedToken(ParsingError):
        token: _Token

        def __init__(self, token: _Token, /) -> None: ...

else:
    from . import _az

    # classes
    Associativity = _az.Associativity
    Filler = _az.Filler
    FillerContent = _az.FillerContent
    Precedence = _az.Precedence

    BinaryAdditionOperator = _az.BinaryAdditionOperator
    BinaryDivisionOperator = _az.BinaryDivisionOperator
    BinaryMultiplicationOperator = _az.BinaryMultiplicationOperator
    BinarySubtractionOperator = _az.BinarySubtractionOperator

    BinaryArithmeticOperator = (
        type[BinaryAdditionOperator]
        | type[BinaryDivisionOperator]
        | type[BinaryMultiplicationOperator]
        | type[BinarySubtractionOperator]
    )

    BinaryAnnotationOperator = _az.BinaryAnnotationOperator
    BinaryAssignmentOperator = _az.BinaryAssignmentOperator
    CallOperator = _az.CallOperator
    MemberAccessOperator = _az.MemberAccessOperator

    BinaryEqualToOperator = _az.BinaryEqualToOperator
    BinaryGreaterThanOperator = _az.BinaryGreaterThanOperator
    BinaryGreaterThanOrEqualToOperator = _az.BinaryGreaterThanOrEqualToOperator
    BinaryLowerThanOperator = _az.BinaryLowerThanOperator
    BinaryLowerThanOrEqualToOperator = _az.BinaryLowerThanOrEqualToOperator
    BinaryNotEqualToOperator = _az.BinaryNotEqualToOperator

    BinaryComparisonOperator = (
        type[BinaryEqualToOperator]
        | type[BinaryGreaterThanOperator]
        | type[BinaryGreaterThanOrEqualToOperator]
        | type[BinaryLowerThanOperator]
        | type[BinaryLowerThanOrEqualToOperator]
        | type[BinaryNotEqualToOperator]
    )

    UnaryNegationOperator = _az.UnaryNegationOperator

    UnaryArithmeticOperator = type[UnaryNegationOperator]

    Expression = _az.Expression
    AnnotatedIdentifier = _az.AnnotatedIdentifier
    Assignment = _az.Assignment
    BinaryArithmeticOperation = _az.BinaryArithmeticOperation
    BinaryComparison = _az.BinaryComparison
    Block = _az.Block
    Call = _az.Call
    Conditional = _az.Conditional
    FunctionDefinition = _az.FunctionDefinition
    Grouping = _az.Grouping
    Identifier = _az.Identifier
    MemberAccess = _az.MemberAccess
    NumericLiteral = _az.NumericLiteral
    Tuple = _az.Tuple
    UnaryArithmeticOperation = _az.UnaryArithmeticOperation

    Script = _az.Script

    Statement = _az.Statement
    ExpressionStatement = _az.ExpressionStatement

    # enumerations
    FillerKind = _az.FillerKind
    NumericLiteralType = _az.NumericLiteralType
    UnaryNegationOperator = _az.UnaryNegationOperator

    # exceptions
    ParsingError = _az.ParsingError
    MismatchedOpenBrace = _az.MismatchedOpenBrace
    MismatchedOpenParenthesis = _az.MismatchedOpenParenthesis
    MissingSemicolon = _az.MissingSemicolon
    OutOfTokens = _az.OutOfTokens
    UnexpectedExpression = _az.UnexpectedExpression
    UnexpectedToken = _az.UnexpectedToken

    del _az
