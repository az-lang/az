import inspect
import typing as t
from functools import singledispatch
from itertools import chain

from az.parsing import (
    AnnotatedIdentifier,
    Assignment,
    BinaryArithmeticOperation,
    BinaryComparison,
    Block,
    Call,
    Conditional,
    Expression,
    ExpressionStatement,
    Filler,
    FunctionDefinition,
    Grouping,
    Identifier,
    MemberAccess,
    NumericLiteral,
    Script,
    Statement,
    Tuple,
    UnaryArithmeticOperation,
)
from az.tokenization import Token


def equivalence(left: bool, right: bool, /) -> bool:
    return left is right


Fillers = list[Filler]


@singledispatch
def expression_to_fillers(expression: Expression) -> Fillers:
    raise TypeError(type(expression))


@expression_to_fillers.register(AnnotatedIdentifier)
def _(expression: AnnotatedIdentifier) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.identifier),
            expression.operator_fillers,
            expression_to_fillers(expression.annotation),
        )
    )


@expression_to_fillers.register(Assignment)
def _(expression: Assignment) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.target),
            expression.operator_fillers,
            expression_to_fillers(expression.value),
        )
    )


@expression_to_fillers.register(BinaryArithmeticOperation)
def _(expression: BinaryArithmeticOperation) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.left),
            expression.operator_fillers,
            expression_to_fillers(expression.right),
        )
    )


@expression_to_fillers.register(BinaryComparison)
def _(expression: BinaryComparison) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.left),
            expression.operator_fillers,
            expression_to_fillers(expression.right),
        )
    )


@expression_to_fillers.register(Block)
def _(expression: Block) -> Fillers:
    return list(
        chain(
            expression.open_brace_fillers,
            chain.from_iterable(
                statement_to_fillers(statement)
                for statement in expression.statements
            ),
            (
                []
                if expression.expression is None
                else expression_to_fillers(expression.expression)
            ),
            expression.close_brace_fillers,
        )
    )


@expression_to_fillers.register(Call)
def _(expression: Call) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.callable),
            expression.open_parenthesis_fillers,
            (
                chain.from_iterable(
                    expression_to_fillers(argument) + comma_fillers
                    for argument, comma_fillers in zip(
                        expression.arguments,
                        expression.commas_fillers,
                        strict=True,
                    )
                )
                if len(expression.arguments) == len(expression.commas_fillers)
                else chain(
                    chain.from_iterable(
                        expression_to_fillers(argument) + comma_fillers
                        for argument, comma_fillers in zip(
                            expression.arguments[:-1],
                            expression.commas_fillers,
                            strict=True,
                        )
                    ),
                    expression_to_fillers(expression.arguments[-1]),
                )
            ),
            expression.close_parenthesis_fillers,
        )
    )


@expression_to_fillers.register(Conditional)
def _(expression: Conditional) -> Fillers:
    return list(
        chain(
            expression.opener_fillers,
            expression_to_fillers(expression.antecedent),
            expression_to_fillers(expression.consequent),
            expression.alternative_opener_fillers,
            (
                []
                if expression.alternative is None
                else expression_to_fillers(expression.alternative)
            ),
        )
    )


@expression_to_fillers.register(FunctionDefinition)
def _(expression: FunctionDefinition) -> Fillers:
    return list(
        chain(
            expression.opener_fillers,
            expression.open_parenthesis_fillers,
            (
                chain.from_iterable(
                    expression_to_fillers(parameter) + comma_fillers
                    for parameter, comma_fillers in zip(
                        expression.parameters,
                        expression.commas_fillers,
                        strict=True,
                    )
                )
                if len(expression.parameters) == len(expression.commas_fillers)
                else chain(
                    chain.from_iterable(
                        expression_to_fillers(parameter) + comma_fillers
                        for parameter, comma_fillers in zip(
                            expression.parameters[:-1],
                            expression.commas_fillers,
                            strict=True,
                        )
                    ),
                    expression_to_fillers(expression.parameters[-1]),
                )
            ),
            expression.close_parenthesis_fillers,
            expression.arrow_fillers,
            expression_to_fillers(expression.return_type),
            expression_to_fillers(expression.body),
        )
    )


@expression_to_fillers.register(Grouping)
def _(expression: Grouping) -> Fillers:
    return list(
        chain(
            expression.open_parenthesis_fillers,
            expression_to_fillers(expression.expression),
            expression.close_parenthesis_fillers,
        )
    )


@expression_to_fillers.register(Identifier)
def _(expression: Identifier) -> Fillers:
    return expression.fillers


@expression_to_fillers.register(MemberAccess)
def _(expression: MemberAccess) -> Fillers:
    return list(
        chain(
            expression_to_fillers(expression.object),
            expression.operator_fillers,
            expression_to_fillers(expression.member),
        )
    )


@expression_to_fillers.register(NumericLiteral)
def _(expression: NumericLiteral) -> Fillers:
    return expression.fillers


@expression_to_fillers.register(Tuple)
def _(expression: Tuple) -> Fillers:
    return list(
        chain(
            expression.open_parenthesis_fillers,
            chain.from_iterable(
                expression_to_fillers(element) + comma_fillers
                for element, comma_fillers in zip(
                    expression.elements, expression.commas_fillers, strict=True
                )
            ),
            expression.close_parenthesis_fillers,
        )
    )


@expression_to_fillers.register(UnaryArithmeticOperation)
def _(expression: UnaryArithmeticOperation) -> Fillers:
    return expression.operator_fillers + expression_to_fillers(
        expression.operand
    )


def implication(antecedent: bool, consequent: bool, /) -> bool:
    return not antecedent or consequent


def to_typeless_signature(
    signature: inspect.Signature,
) -> inspect.Signature | None:
    try:
        return signature.replace(
            parameters=[
                parameter.replace(annotation=inspect.Parameter.empty)
                for parameter in signature.parameters.values()
            ],
            return_annotation=inspect.Parameter.empty,
        )
    except ValueError:
        return None


def to_class_signature(cls: type[t.Any]) -> inspect.Signature | None:
    try:
        return inspect.signature(cls)
    except (TypeError, ValueError):
        return None


# See Include/object.h
TPFLAGS_BASETYPE: t.Final[int] = 1 << 10


def is_class_final(cls: type[t.Any]) -> bool:
    return not (cls.__flags__ & TPFLAGS_BASETYPE)


def is_private_object_name(value: str) -> bool:
    return value.startswith('_')


def script_to_fillers(script: Script) -> list[Filler]:
    return (
        list(chain.from_iterable(map(statement_to_fillers, script.statements)))
        + script.fillers
    )


def split_any_string_keeping_separators(
    value: t.AnyStr, separator: t.AnyStr
) -> list[t.AnyStr]:
    lines_without_ends = value.split(separator)
    return (
        [line + separator for line in lines_without_ends]
        if len(lines_without_ends) > 1
        else lines_without_ends
    )


@singledispatch
def statement_to_fillers(statement: Statement) -> Fillers:
    raise TypeError(type(statement))


@statement_to_fillers.register(ExpressionStatement)
def _(statement: ExpressionStatement) -> Fillers:
    return (
        expression_to_fillers(statement.expression)
        + statement.semicolon_fillers
    )


@singledispatch
def statement_to_expression(statement: Statement) -> Expression:
    raise TypeError(type(statement))


@statement_to_expression.register(ExpressionStatement)
def _(statement: ExpressionStatement) -> Expression:
    return statement.expression


def to_base_annotation(annotation: t.Any) -> t.Any | None:
    result = t.get_origin(annotation)
    if result is t.Annotated:
        result = t.get_origin(t.get_args(annotation)[0])
    return result


def tokens_to_string(tokens: list[Token]) -> str:
    return ''.join(token.content.string for token in tokens)
