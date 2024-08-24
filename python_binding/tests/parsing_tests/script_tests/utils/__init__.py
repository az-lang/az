from __future__ import annotations

import enum
from functools import singledispatch
from typing import Any

from az.parsing import (
    AnnotatedIdentifier,
    Assignment,
    BidirectionalConditional,
    BinaryAdditionOperator,
    BinaryArithmeticOperation,
    BinaryArithmeticOperator,
    BinaryComparison,
    BinaryComparisonOperator,
    BinaryDivisionOperator,
    BinaryEqualToOperator,
    BinaryGreaterThanOperator,
    BinaryGreaterThanOrEqualToOperator,
    BinaryLessThanOperator,
    BinaryLessThanOrEqualToOperator,
    BinaryMultiplicationOperator,
    BinaryNotEqualToOperator,
    BinarySubtractionOperator,
    Block,
    Call,
    Expression,
    ExpressionStatement,
    FunctionDefinition,
    FunctionType,
    Grouping,
    Identifier,
    MemberAccess,
    NumericLiteral,
    NumericLiteralType,
    Return,
    Script,
    Statement,
    Tuple,
    UnaryArithmeticOperation,
    UnaryArithmeticOperator,
    UnaryNegationOperator,
    UnidirectionalConditional,
    WhileLoop,
)


class AST:
    def __init__(self, statement_nodes: list[StatementNode]) -> None:
        self.statement_nodes = statement_nodes

    def __eq__(self, other: Any) -> Any:
        return (
            self.statement_nodes == other.statement_nodes
            if isinstance(other, AST)
            else NotImplemented
        )


def script_to_ast(script: Script) -> AST:
    return AST(
        [statement_to_node(statement) for statement in script.statements]
    )


class ExpressionKind(enum.Enum):
    ANNOTATED_IDENTIFIER = enum.auto()
    ASSIGNMENT = enum.auto()
    BIDIRECTIONAL_CONDITIONAL = enum.auto()

    BINARY_ARITHMETIC_OPERATION_ADDITION = enum.auto()
    BINARY_ARITHMETIC_OPERATION_DIVISION = enum.auto()
    BINARY_ARITHMETIC_OPERATION_MULTIPLICATION = enum.auto()
    BINARY_ARITHMETIC_OPERATION_SUBTRACTION = enum.auto()

    BINARY_COMPARISON_EQUAL_TO = enum.auto()
    BINARY_COMPARISON_GREATER_THAN = enum.auto()
    BINARY_COMPARISON_GREATER_THAN_OR_EQUAL_TO = enum.auto()
    BINARY_COMPARISON_LESS_THAN = enum.auto()
    BINARY_COMPARISON_LESS_THAN_OR_EQUAL_TO = enum.auto()
    BINARY_COMPARISON_NOT_EQUAL_TO = enum.auto()

    BLOCK = enum.auto()
    CALL = enum.auto()
    FUNCTION_DEFINITION = enum.auto()
    FUNCTION_TYPE = enum.auto()
    GROUPING = enum.auto()
    IDENTIFIER = enum.auto()
    MEMBER_ACCESS = enum.auto()

    NUMERIC_LITERAL_F32 = enum.auto()
    NUMERIC_LITERAL_F64 = enum.auto()
    NUMERIC_LITERAL_I16 = enum.auto()
    NUMERIC_LITERAL_I32 = enum.auto()
    NUMERIC_LITERAL_I64 = enum.auto()
    NUMERIC_LITERAL_I8 = enum.auto()
    NUMERIC_LITERAL_ISIZE = enum.auto()
    NUMERIC_LITERAL_U16 = enum.auto()
    NUMERIC_LITERAL_U32 = enum.auto()
    NUMERIC_LITERAL_U64 = enum.auto()
    NUMERIC_LITERAL_U8 = enum.auto()
    NUMERIC_LITERAL_USIZE = enum.auto()

    RETURN = enum.auto()
    TUPLE = enum.auto()
    UNARY_ARITHMETIC_OPERATION_NEGATION = enum.auto()
    UNIDIRECTIONAL_CONDITIONAL = enum.auto()


class ExpressionNode:
    def __init__(
        self,
        /,
        *,
        content: str | None = None,
        expression_nodes: list[ExpressionNode],
        kind: ExpressionKind,
        statement_nodes: list[StatementNode],
    ) -> None:
        (
            self.content,
            self.expression_nodes,
            self.kind,
            self.statement_nodes,
        ) = (content, expression_nodes, kind, statement_nodes)

    def __eq__(self, other: Any) -> Any:
        return (
            (
                self.kind is other.kind
                and self.content == other.content
                and self.expression_nodes == other.expression_nodes
                and self.statement_nodes == other.statement_nodes
            )
            if isinstance(other, ExpressionNode)
            else NotImplemented
        )


class StatementKind(enum.Enum):
    EXPRESSION = enum.auto()


class StatementNode:
    def __init__(
        self, /, *, kind: StatementKind, expression_nodes: list[ExpressionNode]
    ) -> None:
        self.expression_nodes, self.kind = expression_nodes, kind

    def __eq__(self, other: Any) -> Any:
        return (
            (
                self.kind is other.kind
                and self.expression_nodes == other.expression_nodes
            )
            if isinstance(other, StatementNode)
            else NotImplemented
        )


@singledispatch
def statement_to_node(statement: Statement, /) -> StatementNode:
    raise TypeError(type(statement))


@statement_to_node.register(ExpressionStatement)
def _(statement: ExpressionStatement, /) -> StatementNode:
    return StatementNode(
        kind=StatementKind.EXPRESSION,
        expression_nodes=[expression_to_node(statement.expression)],
    )


@singledispatch
def expression_to_node(expression: Expression, /) -> ExpressionNode:
    raise TypeError(type(expression))


@expression_to_node.register(AnnotatedIdentifier)
def annotated_identifier_to_node(
    expression: AnnotatedIdentifier, /
) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.ANNOTATED_IDENTIFIER,
        statement_nodes=[],
        expression_nodes=[
            identifier_to_node(expression.identifier),
            expression_to_node(expression.annotation),
        ],
    )


@expression_to_node.register(Assignment)
def _(expression: Assignment, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.ASSIGNMENT,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.target),
            expression_to_node(expression.value),
        ],
    )


@expression_to_node.register(BidirectionalConditional)
def _(expression: BidirectionalConditional, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.BIDIRECTIONAL_CONDITIONAL,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.antecedent),
            expression_to_node(expression.consequent),
            expression_to_node(expression.alternative),
        ],
    )


@expression_to_node.register(BinaryArithmeticOperation)
def _(expression: BinaryArithmeticOperation, /) -> ExpressionNode:
    return ExpressionNode(
        kind=binary_arithmetic_operator_to_expression_kind(
            expression.operator
        ),
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.left),
            expression_to_node(expression.right),
        ],
    )


@expression_to_node.register(BinaryComparison)
def _(expression: BinaryComparison, /) -> ExpressionNode:
    return ExpressionNode(
        kind=binary_comparison_operator_to_expression_kind(
            expression.operator
        ),
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.left),
            expression_to_node(expression.right),
        ],
    )


@expression_to_node.register(Block)
def block_to_node(expression: Block, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.BLOCK,
        statement_nodes=[*map(statement_to_node, expression.statements)],
        expression_nodes=[
            *(
                ()
                if expression.expression is None
                else (expression_to_node(expression.expression),)
            )
        ],
    )


@expression_to_node.register(Call)
def _(expression: Call, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.CALL,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.callable),
            *map(expression_to_node, expression.arguments),
        ],
    )


@expression_to_node.register(FunctionDefinition)
def _(expression: FunctionDefinition, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.FUNCTION_DEFINITION,
        statement_nodes=[],
        expression_nodes=[
            *map(annotated_identifier_to_node, expression.parameters),
            expression_to_node(expression.return_type),
            block_to_node(expression.body),
        ],
    )


@expression_to_node.register(FunctionType)
def _(expression: FunctionType, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.FUNCTION_TYPE,
        statement_nodes=[],
        expression_nodes=[
            *map(expression_to_node, expression.parameters),
            expression_to_node(expression.return_type),
        ],
    )


@expression_to_node.register(Grouping)
def _(expression: Grouping, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.GROUPING,
        statement_nodes=[],
        expression_nodes=[expression_to_node(expression.expression)],
    )


@expression_to_node.register(Identifier)
def identifier_to_node(expression: Identifier, /) -> ExpressionNode:
    return ExpressionNode(
        content=expression.string,
        kind=ExpressionKind.IDENTIFIER,
        statement_nodes=[],
        expression_nodes=[],
    )


@expression_to_node.register(MemberAccess)
def _(expression: MemberAccess, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.MEMBER_ACCESS,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.object),
            identifier_to_node(expression.member),
        ],
    )


@expression_to_node.register(NumericLiteral)
def _(expression: NumericLiteral, /) -> ExpressionNode:
    return ExpressionNode(
        content=expression.value,
        kind=numeric_literal_type_to_expression_kind(expression.type_),
        statement_nodes=[],
        expression_nodes=[],
    )


@expression_to_node.register(Return)
def _(expression: Return, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.RETURN,
        statement_nodes=[],
        expression_nodes=[expression_to_node(expression.expression)],
    )


@expression_to_node.register(Tuple)
def _(expression: Tuple, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.TUPLE,
        statement_nodes=[],
        expression_nodes=[*map(expression_to_node, expression.elements)],
    )


@expression_to_node.register(UnaryArithmeticOperation)
def _(expression: UnaryArithmeticOperation, /) -> ExpressionNode:
    return ExpressionNode(
        kind=unary_arithmetic_operator_to_expression_kind(expression.operator),
        statement_nodes=[],
        expression_nodes=[expression_to_node(expression.operand)],
    )


@expression_to_node.register(UnidirectionalConditional)
def _(expression: UnidirectionalConditional, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.UNIDIRECTIONAL_CONDITIONAL,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.antecedent),
            block_to_node(expression.consequent),
        ],
    )


@expression_to_node.register(WhileLoop)
def _(expression: WhileLoop, /) -> ExpressionNode:
    return ExpressionNode(
        kind=ExpressionKind.UNIDIRECTIONAL_CONDITIONAL,
        statement_nodes=[],
        expression_nodes=[
            expression_to_node(expression.condition),
            block_to_node(expression.body),
        ],
    )


assert all(
    (
        expression_to_node.dispatch(cls)
        is not expression_to_node.dispatch(Expression)
    )
    for cls in Expression.__subclasses__()
), [
    cls
    for cls in Expression.__subclasses__()
    if (
        expression_to_node.dispatch(cls)
        is expression_to_node.dispatch(Expression)
    )
]


def binary_arithmetic_operator_to_expression_kind(
    operator: BinaryArithmeticOperator, /
) -> ExpressionKind:
    if operator is BinaryAdditionOperator:
        return ExpressionKind.BINARY_ARITHMETIC_OPERATION_ADDITION
    elif operator is BinaryDivisionOperator:
        return ExpressionKind.BINARY_ARITHMETIC_OPERATION_DIVISION
    elif operator is BinaryMultiplicationOperator:
        return ExpressionKind.BINARY_ARITHMETIC_OPERATION_MULTIPLICATION
    else:
        assert operator is BinarySubtractionOperator, operator
        return ExpressionKind.BINARY_ARITHMETIC_OPERATION_SUBTRACTION


def binary_comparison_operator_to_expression_kind(
    operator: BinaryComparisonOperator, /
) -> ExpressionKind:
    if operator is BinaryEqualToOperator:
        return ExpressionKind.BINARY_COMPARISON_EQUAL_TO
    elif operator is BinaryGreaterThanOperator:
        return ExpressionKind.BINARY_COMPARISON_GREATER_THAN
    elif operator is BinaryGreaterThanOrEqualToOperator:
        return ExpressionKind.BINARY_COMPARISON_GREATER_THAN_OR_EQUAL_TO
    elif operator is BinaryLessThanOperator:
        return ExpressionKind.BINARY_COMPARISON_LESS_THAN
    elif operator is BinaryLessThanOrEqualToOperator:
        return ExpressionKind.BINARY_COMPARISON_LESS_THAN_OR_EQUAL_TO
    else:
        assert operator is BinaryNotEqualToOperator, operator
        return ExpressionKind.BINARY_COMPARISON_NOT_EQUAL_TO


def numeric_literal_type_to_expression_kind(
    type_: NumericLiteralType, /
) -> ExpressionKind:
    if type_ == NumericLiteralType.F32:
        return ExpressionKind.NUMERIC_LITERAL_F32
    elif type_ == NumericLiteralType.F64:
        return ExpressionKind.NUMERIC_LITERAL_F64
    elif type_ == NumericLiteralType.I16:
        return ExpressionKind.NUMERIC_LITERAL_I16
    elif type_ == NumericLiteralType.I32:
        return ExpressionKind.NUMERIC_LITERAL_I32
    elif type_ == NumericLiteralType.I64:
        return ExpressionKind.NUMERIC_LITERAL_I64
    elif type_ == NumericLiteralType.I8:
        return ExpressionKind.NUMERIC_LITERAL_I8
    elif type_ == NumericLiteralType.ISIZE:
        return ExpressionKind.NUMERIC_LITERAL_ISIZE
    elif type_ == NumericLiteralType.U16:
        return ExpressionKind.NUMERIC_LITERAL_U16
    elif type_ == NumericLiteralType.U32:
        return ExpressionKind.NUMERIC_LITERAL_U32
    elif type_ == NumericLiteralType.U64:
        return ExpressionKind.NUMERIC_LITERAL_U64
    elif type_ == NumericLiteralType.U8:
        return ExpressionKind.NUMERIC_LITERAL_U8
    else:
        assert type_ == NumericLiteralType.USIZE, type_
        return ExpressionKind.NUMERIC_LITERAL_USIZE


def unary_arithmetic_operator_to_expression_kind(
    operator: UnaryArithmeticOperator, /
) -> ExpressionKind:
    assert operator is UnaryNegationOperator, operator
    return ExpressionKind.UNARY_ARITHMETIC_OPERATION_NEGATION
