from functools import singledispatch as _singledispatch

from az.parsing import (
    AnnotatedIdentifier as _AnnotatedIdentifier,
    Assignment as _Assignment,
    BidirectionalConditional as _BidirectionalConditional,
    BinaryArithmeticOperation as _BinaryArithmeticOperation,
    BinaryArithmeticOperator as _BinaryArithmeticOperator,
    BinaryComparison as _BinaryComparison,
    BinaryComparisonOperator as _BinaryComparisonOperator,
    BinaryDivisionOperator as _BinaryDivisionOperator,
    Block as _Block,
    Call as _Call,
    Expression as _Expression,
    ExpressionStatement as _ExpressionStatement,
    Filler as _Filler,
    FillerKind as _FillerKind,
    FunctionDefinition as _FunctionDefinition,
    FunctionType as _FunctionType,
    Grouping as _Grouping,
    Identifier as _Identifier,
    MemberAccess as _MemberAccess,
    NumericLiteral as _NumericLiteral,
    Return as _Return,
    Statement as _Statement,
    Tuple as _Tuple,
    UnaryArithmeticOperation as _UnaryArithmeticOperation,
    UnaryArithmeticOperator as _UnaryArithmeticOperator,
    UnidirectionalConditional as _UnidirectionalConditional,
    WhileLoop as _WhileLoop,
)
from az.tokenization import SubstringPosition as _SubstringPosition
from hypothesis import strategies as _st

from ._fillers import (
    filler_list_strategy as _filler_list_strategy,
    non_comment_filler_strategy as _non_comment_filler_strategy,
)
from ._leaf_expressions import (
    identifier_strategy as _identifier_strategy,
    to_validated_identifier as _to_validated_identifier,
    to_validated_numeric_literal as _to_validated_numeric_literal,
)
from ._positions import (
    substring_position_strategy as _substring_position_strategy,
)

MAX_EXPRESSIONS_SIZE = 4
MAX_STATEMENTS_SIZE = 4


def to_annotated_identifier_strategy(
    annotation_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_AnnotatedIdentifier]:
    return _st.builds(
        to_validated_annotated_identifier,
        _identifier_strategy,
        annotation_strategy,
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_assignment_strategy(
    target_strategy: _st.SearchStrategy[_Expression],
    value_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_Assignment]:
    return _st.builds(
        to_validated_assignment,
        target_strategy,
        value_strategy,
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_bidirectional_conditional_strategy(
    antecedent_base_strategy: _st.SearchStrategy[_Expression],
    consequent_base_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_BidirectionalConditional]:
    antecedent_strategy = _to_antecedent_strategy(antecedent_base_strategy)
    block_strategy = to_block_strategy(antecedent_strategy)
    unidirectional_conditional_alternative_strategy = _st.builds(
        to_non_lexically_conflicting_unidirectional_conditional,
        to_unidirectional_conditional_strategy(
            antecedent_strategy, consequent_base_strategy
        ),
        _non_comment_filler_strategy,
    )
    return _st.recursive(
        _st.builds(
            to_validated_bidirectional_conditional,
            antecedent_strategy,
            block_strategy,
            block_strategy | unidirectional_conditional_alternative_strategy,
            antecedent_opener_position=_substring_position_strategy,
            alternative_opener_position=_substring_position_strategy,
            antecedent_opener_fillers=_filler_list_strategy,
            alternative_opener_fillers=_filler_list_strategy,
        ),
        lambda step: _st.builds(
            to_validated_bidirectional_conditional,
            antecedent_strategy,
            block_strategy,
            block_strategy
            | unidirectional_conditional_alternative_strategy
            | _st.builds(
                to_non_lexically_conflicting_bidirectional_conditional,
                step,
                _non_comment_filler_strategy,
            ),
            antecedent_opener_position=_substring_position_strategy,
            alternative_opener_position=_substring_position_strategy,
            antecedent_opener_fillers=_filler_list_strategy,
            alternative_opener_fillers=_filler_list_strategy,
        ),
        max_leaves=4,
    )


def to_binary_arithmetic_operation_strategy(
    operator: _BinaryArithmeticOperator,
    left_operand_strategy: _st.SearchStrategy[_Expression],
    right_operand_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_BinaryArithmeticOperation]:
    return _st.builds(
        to_validated_binary_arithmetic_operation,
        left_operand_strategy,
        (
            _st.builds(
                to_valid_divisor,
                right_operand_strategy,
                _non_comment_filler_strategy,
            )
            if operator is _BinaryDivisionOperator
            else right_operand_strategy
        ),
        _st.just(operator),
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_binary_comparison_strategy(
    operator: _BinaryComparisonOperator,
    left_operand_strategy: _st.SearchStrategy[_Expression],
    right_operand_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_BinaryComparison]:
    return _st.builds(
        to_validated_binary_comparison,
        left_operand_strategy,
        right_operand_strategy,
        _st.just(operator),
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_block_strategy(
    expression_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_Block]:
    return _st.builds(
        to_validated_block,
        _st.lists(
            to_statement_strategy(expression_strategy),
            max_size=MAX_STATEMENTS_SIZE,
        ),
        _st.none() | expression_strategy,
        open_brace_position=_substring_position_strategy,
        close_brace_position=_substring_position_strategy,
        open_brace_fillers=_filler_list_strategy,
        close_brace_fillers=_filler_list_strategy,
    )


def to_call_strategy(
    callable_strategy: _st.SearchStrategy[_Expression],
    argument_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_Call]:
    return (
        _st.lists(argument_strategy, max_size=MAX_EXPRESSIONS_SIZE)
        .flatmap(
            lambda arguments: _st.tuples(
                _st.just(arguments),
                _st.lists(
                    _substring_position_strategy,
                    min_size=max(len(arguments) - 1, 0),
                    max_size=len(arguments),
                ),
            )
        )
        .flatmap(
            lambda arguments_with_comma_positions: _st.builds(
                to_validated_call,
                callable_strategy,
                _st.just(arguments_with_comma_positions[0]),
                open_parenthesis_position=_substring_position_strategy,
                comma_positions=_st.just(arguments_with_comma_positions[1]),
                close_parenthesis_position=_substring_position_strategy,
                open_parenthesis_fillers=_filler_list_strategy,
                comma_fillers=_st.lists(
                    _filler_list_strategy,
                    min_size=len(arguments_with_comma_positions[1]),
                    max_size=len(arguments_with_comma_positions[1]),
                ),
                close_parenthesis_fillers=_filler_list_strategy,
            )
        )
    )


def to_expression_statement_strategy(
    expression_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_ExpressionStatement]:
    return _st.builds(
        _ExpressionStatement,
        expression_strategy,
        semicolon_position=_substring_position_strategy,
        semicolon_fillers=_filler_list_strategy,
    )


@_st.composite
def to_function_definition_strategy(
    draw: _st.DrawFn,
    parameter_annotation_strategy: _st.SearchStrategy[_Expression],
    return_type_strategy: _st.SearchStrategy[_Expression],
    body_expression_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _FunctionDefinition:
    parameters = draw(
        _st.lists(
            to_annotated_identifier_strategy(parameter_annotation_strategy),
            max_size=MAX_EXPRESSIONS_SIZE,
        )
    )
    comma_positions = draw(
        _st.lists(
            _substring_position_strategy,
            min_size=max(len(parameters) - 1, 0),
            max_size=len(parameters),
        )
    )
    return to_validated_function_definition(
        parameters,
        draw(return_type_strategy),
        draw(to_block_strategy(body_expression_strategy)),
        opener_position=draw(_substring_position_strategy),
        open_parenthesis_position=draw(_substring_position_strategy),
        comma_positions=comma_positions,
        close_parenthesis_position=draw(_substring_position_strategy),
        arrow_position=draw(_substring_position_strategy),
        opener_fillers=draw(_filler_list_strategy),
        open_parenthesis_fillers=draw(_filler_list_strategy),
        comma_fillers=draw(
            _st.lists(
                _filler_list_strategy,
                min_size=len(comma_positions),
                max_size=len(comma_positions),
            )
        ),
        close_parenthesis_fillers=draw(_filler_list_strategy),
        arrow_fillers=draw(_filler_list_strategy),
    )


@_st.composite
def to_function_type_strategy(
    draw: _st.DrawFn,
    parameter_strategy: _st.SearchStrategy[_Expression],
    return_type_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _FunctionType:
    parameters = draw(
        _st.lists(parameter_strategy, max_size=MAX_EXPRESSIONS_SIZE)
    )
    comma_positions = draw(
        _st.lists(
            _substring_position_strategy,
            min_size=max(len(parameters) - 1, 0),
            max_size=len(parameters),
        )
    )
    return to_validated_function_type(
        parameters,
        draw(return_type_strategy),
        open_parenthesis_position=draw(_substring_position_strategy),
        comma_positions=comma_positions,
        close_parenthesis_position=draw(_substring_position_strategy),
        operator_position=draw(_substring_position_strategy),
        open_parenthesis_fillers=draw(_filler_list_strategy),
        comma_fillers=draw(
            _st.lists(
                _filler_list_strategy,
                min_size=len(comma_positions),
                max_size=len(comma_positions),
            )
        ),
        close_parenthesis_fillers=draw(_filler_list_strategy),
        operator_fillers=draw(_filler_list_strategy),
    )


def to_grouping_strategy(
    expression_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_Grouping]:
    return _st.builds(
        to_validated_grouping,
        expression_strategy,
        open_parenthesis_position=_substring_position_strategy,
        close_parenthesis_position=_substring_position_strategy,
        open_parenthesis_fillers=_filler_list_strategy,
        close_parenthesis_fillers=_filler_list_strategy,
    )


def to_member_access_strategy(
    object_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_MemberAccess]:
    return _st.builds(
        to_validated_member_access,
        object_strategy,
        _identifier_strategy,
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_return_strategy(
    expression_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_Return]:
    return _st.builds(
        to_validated_return,
        _st.builds(
            to_non_lexically_conflicting_expression,
            expression_strategy,
            _non_comment_filler_strategy,
        ),
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


@_st.composite
def to_tuple_strategy(
    draw: _st.DrawFn, element_strategy: _st.SearchStrategy[_Expression]
) -> _Tuple:
    elements = draw(_st.lists(element_strategy))
    comma_positions = draw(
        _st.lists(
            _substring_position_strategy,
            min_size=(1 if len(elements) == 1 else max(len(elements) - 1, 0)),
            max_size=len(elements),
        )
    )
    return to_validated_tuple(
        elements,
        open_parenthesis_position=draw(_substring_position_strategy),
        comma_positions=comma_positions,
        close_parenthesis_position=draw(_substring_position_strategy),
        open_parenthesis_fillers=draw(_filler_list_strategy),
        comma_fillers=draw(
            _st.lists(
                _filler_list_strategy,
                min_size=len(comma_positions),
                max_size=len(comma_positions),
            )
        ),
        close_parenthesis_fillers=draw(_filler_list_strategy),
    )


def to_statement_strategy(
    expression_strategy: _st.SearchStrategy[_Expression], /
) -> _st.SearchStrategy[_Statement]:
    return to_expression_statement_strategy(expression_strategy)


def to_unary_arithmetic_operation_strategy(
    operator: _UnaryArithmeticOperator,
    operand_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_UnaryArithmeticOperation]:
    return _st.builds(
        to_validated_unary_arithmetic_operation,
        operand_strategy,
        _st.just(operator),
        operator_position=_substring_position_strategy,
        operator_fillers=_filler_list_strategy,
    )


def to_unidirectional_conditional_strategy(
    antecedent_base_strategy: _st.SearchStrategy[_Expression],
    consequent_base_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_UnidirectionalConditional]:
    antecedent_strategy = _to_antecedent_strategy(antecedent_base_strategy)
    consequent_strategy = to_block_strategy(consequent_base_strategy)
    return _st.builds(
        to_validated_unidirectional_conditional,
        antecedent_strategy,
        consequent_strategy,
        opener_position=_substring_position_strategy,
        opener_fillers=_filler_list_strategy,
    )


def to_while_loop_strategy(
    condition_base_strategy: _st.SearchStrategy[_Expression],
    body_base_strategy: _st.SearchStrategy[_Expression],
    /,
) -> _st.SearchStrategy[_WhileLoop]:
    condition_strategy = _st.builds(
        to_non_lexically_conflicting_expression,
        condition_base_strategy,
        _non_comment_filler_strategy,
    )
    body_strategy = to_block_strategy(body_base_strategy)
    return _st.builds(
        to_validated_while_loop,
        condition_strategy,
        body_strategy,
        opener_position=_substring_position_strategy,
        opener_fillers=_filler_list_strategy,
    )


def to_validated_annotated_identifier(
    identifier: _Identifier,
    annotation: _Expression,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _AnnotatedIdentifier:
    result = _AnnotatedIdentifier(
        identifier,
        annotation,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_assignment(
    target: _Expression,
    value: _Expression,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _Assignment:
    result = _Assignment(
        target,
        value,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_bidirectional_conditional(
    antecedent: _Expression,
    consequent: _Block,
    alternative: _Expression,
    /,
    *,
    antecedent_opener_position: _SubstringPosition,
    alternative_opener_position: _SubstringPosition,
    antecedent_opener_fillers: list[_Filler],
    alternative_opener_fillers: list[_Filler],
) -> _BidirectionalConditional:
    result = _BidirectionalConditional(
        antecedent,
        consequent,
        alternative,
        antecedent_opener_position=antecedent_opener_position,
        alternative_opener_position=alternative_opener_position,
        antecedent_opener_fillers=antecedent_opener_fillers,
        alternative_opener_fillers=alternative_opener_fillers,
    )
    result.validate_contents()
    return result


def to_validated_binary_arithmetic_operation(
    left: _Expression,
    right: _Expression,
    operator: _BinaryArithmeticOperator,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _BinaryArithmeticOperation:
    result = _BinaryArithmeticOperation(
        left,
        right,
        operator,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_binary_comparison(
    left: _Expression,
    right: _Expression,
    operator: _BinaryComparisonOperator,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _BinaryComparison:
    result = _BinaryComparison(
        left,
        right,
        operator,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_block(
    statements: list[_Statement],
    expression: _Expression | None,
    /,
    *,
    open_brace_position: _SubstringPosition,
    close_brace_position: _SubstringPosition,
    open_brace_fillers: list[_Filler],
    close_brace_fillers: list[_Filler],
) -> _Block:
    result = _Block(
        statements,
        expression,
        open_brace_position=open_brace_position,
        close_brace_position=close_brace_position,
        open_brace_fillers=open_brace_fillers,
        close_brace_fillers=close_brace_fillers,
    )
    result.validate_contents()
    return result


def to_validated_call(
    callable_: _Expression,
    arguments: list[_Expression],
    /,
    *,
    open_parenthesis_position: _SubstringPosition,
    comma_positions: list[_SubstringPosition],
    close_parenthesis_position: _SubstringPosition,
    open_parenthesis_fillers: list[_Filler],
    comma_fillers: list[list[_Filler]],
    close_parenthesis_fillers: list[_Filler],
) -> _Call:
    result = _Call(
        callable_,
        arguments,
        open_parenthesis_position=open_parenthesis_position,
        comma_positions=comma_positions,
        close_parenthesis_position=close_parenthesis_position,
        open_parenthesis_fillers=open_parenthesis_fillers,
        comma_fillers=comma_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
    )
    result.validate_contents()
    return result


def to_validated_function_definition(
    parameters: list[_AnnotatedIdentifier],
    return_type: _Expression,
    body: _Block,
    /,
    *,
    opener_position: _SubstringPosition,
    open_parenthesis_position: _SubstringPosition,
    comma_positions: list[_SubstringPosition],
    close_parenthesis_position: _SubstringPosition,
    arrow_position: _SubstringPosition,
    opener_fillers: list[_Filler],
    open_parenthesis_fillers: list[_Filler],
    comma_fillers: list[list[_Filler]],
    close_parenthesis_fillers: list[_Filler],
    arrow_fillers: list[_Filler],
) -> _FunctionDefinition:
    result = _FunctionDefinition(
        parameters,
        return_type,
        body,
        opener_position=opener_position,
        open_parenthesis_position=open_parenthesis_position,
        comma_positions=comma_positions,
        close_parenthesis_position=close_parenthesis_position,
        arrow_position=arrow_position,
        opener_fillers=opener_fillers,
        open_parenthesis_fillers=open_parenthesis_fillers,
        comma_fillers=comma_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
        arrow_fillers=arrow_fillers,
    )
    result.validate_contents()
    return result


def to_validated_function_type(
    parameters: list[_Expression],
    return_type: _Expression,
    /,
    *,
    open_parenthesis_position: _SubstringPosition,
    comma_positions: list[_SubstringPosition],
    close_parenthesis_position: _SubstringPosition,
    operator_position: _SubstringPosition,
    open_parenthesis_fillers: list[_Filler],
    comma_fillers: list[list[_Filler]],
    close_parenthesis_fillers: list[_Filler],
    operator_fillers: list[_Filler],
) -> _FunctionType:
    result = _FunctionType(
        parameters,
        return_type,
        open_parenthesis_position=open_parenthesis_position,
        comma_positions=comma_positions,
        close_parenthesis_position=close_parenthesis_position,
        operator_position=operator_position,
        open_parenthesis_fillers=open_parenthesis_fillers,
        comma_fillers=comma_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_grouping(
    expression: _Expression,
    /,
    *,
    open_parenthesis_position: _SubstringPosition,
    close_parenthesis_position: _SubstringPosition,
    open_parenthesis_fillers: list[_Filler],
    close_parenthesis_fillers: list[_Filler],
) -> _Grouping:
    result = _Grouping(
        expression,
        open_parenthesis_position=open_parenthesis_position,
        close_parenthesis_position=close_parenthesis_position,
        open_parenthesis_fillers=open_parenthesis_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
    )
    result.validate_contents()
    return result


def to_validated_member_access(
    object_: _Expression,
    member: _Identifier,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _MemberAccess:
    result = _MemberAccess(
        object_,
        member,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_return(
    expression: _Expression,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _Return:
    result = _Return(
        expression,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_tuple(
    elements: list[_Expression],
    /,
    *,
    open_parenthesis_position: _SubstringPosition,
    comma_positions: list[_SubstringPosition],
    close_parenthesis_position: _SubstringPosition,
    open_parenthesis_fillers: list[_Filler],
    comma_fillers: list[list[_Filler]],
    close_parenthesis_fillers: list[_Filler],
) -> _Tuple:
    result = _Tuple(
        elements,
        open_parenthesis_position=open_parenthesis_position,
        comma_positions=comma_positions,
        close_parenthesis_position=close_parenthesis_position,
        open_parenthesis_fillers=open_parenthesis_fillers,
        comma_fillers=comma_fillers,
        close_parenthesis_fillers=close_parenthesis_fillers,
    )
    result.validate_contents()
    return result


def to_validated_unary_arithmetic_operation(
    operand: _Expression,
    operator: _UnaryArithmeticOperator,
    /,
    *,
    operator_position: _SubstringPosition,
    operator_fillers: list[_Filler],
) -> _UnaryArithmeticOperation:
    result = _UnaryArithmeticOperation(
        operand,
        operator,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )
    result.validate_contents()
    return result


def to_validated_unidirectional_conditional(
    antecedent: _Expression,
    consequent: _Block,
    /,
    *,
    opener_position: _SubstringPosition,
    opener_fillers: list[_Filler],
) -> _UnidirectionalConditional:
    result = _UnidirectionalConditional(
        antecedent,
        consequent,
        opener_position=opener_position,
        opener_fillers=opener_fillers,
    )
    result.validate_contents()
    return result


def to_validated_while_loop(
    condition: _Expression,
    body: _Block,
    /,
    *,
    opener_position: _SubstringPosition,
    opener_fillers: list[_Filler],
) -> _WhileLoop:
    result = _WhileLoop(
        condition,
        body,
        opener_position=opener_position,
        opener_fillers=opener_fillers,
    )
    result.validate_contents()
    return result


def _is_comment_filler(filler: _Filler) -> bool:
    return filler.content.kind in (
        _FillerKind.COMMENT_BLOCK,
        _FillerKind.COMMENT_LINE,
    )


@_singledispatch
def to_valid_divisor(
    expression: _Expression, _filler: _Filler, /
) -> _Expression:
    raise TypeError(type(expression))


@to_valid_divisor.register(_AnnotatedIdentifier)
def _(
    expression: _AnnotatedIdentifier, filler: _Filler, /
) -> _AnnotatedIdentifier:
    identifier = to_valid_divisor(expression.identifier, filler)
    assert isinstance(identifier, _Identifier), identifier
    return to_validated_annotated_identifier(
        identifier,
        expression.annotation,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_Assignment)
def _(expression: _Assignment, filler: _Filler, /) -> _Assignment:
    return to_validated_assignment(
        to_valid_divisor(expression.target, filler),
        expression.value,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_BidirectionalConditional)
def _(
    expression: _BidirectionalConditional, filler: _Filler, /
) -> _BidirectionalConditional:
    return to_validated_bidirectional_conditional(
        expression.antecedent,
        expression.consequent,
        expression.alternative,
        antecedent_opener_position=expression.antecedent_opener_position,
        alternative_opener_position=expression.alternative_opener_position,
        antecedent_opener_fillers=_to_divisor_fillers(
            expression.antecedent_opener_fillers, filler
        ),
        alternative_opener_fillers=expression.alternative_opener_fillers,
    )


@to_valid_divisor.register(_BinaryArithmeticOperation)
def _(
    expression: _BinaryArithmeticOperation, filler: _Filler, /
) -> _BinaryArithmeticOperation:
    return to_validated_binary_arithmetic_operation(
        to_valid_divisor(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_BinaryComparison)
def _(expression: _BinaryComparison, filler: _Filler, /) -> _BinaryComparison:
    return to_validated_binary_comparison(
        to_valid_divisor(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_Block)
def _(expression: _Block, filler: _Filler, /) -> _Block:
    return to_validated_block(
        expression.statements,
        expression.expression,
        open_brace_position=expression.open_brace_position,
        close_brace_position=expression.close_brace_position,
        open_brace_fillers=_to_divisor_fillers(
            expression.open_brace_fillers, filler
        ),
        close_brace_fillers=expression.close_brace_fillers,
    )


@to_valid_divisor.register(_Call)
def _(expression: _Call, filler: _Filler, /) -> _Call:
    return to_validated_call(
        to_valid_divisor(expression.callable, filler),
        expression.arguments,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@to_valid_divisor.register(_FunctionDefinition)
def _(
    expression: _FunctionDefinition, filler: _Filler, /
) -> _FunctionDefinition:
    return to_validated_function_definition(
        expression.parameters,
        expression.return_type,
        expression.body,
        opener_position=expression.opener_position,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        arrow_position=expression.arrow_position,
        opener_fillers=_to_divisor_fillers(expression.opener_fillers, filler),
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        arrow_fillers=expression.arrow_fillers,
    )


@to_valid_divisor.register(_FunctionType)
def _(expression: _FunctionType, filler: _Filler, /) -> _FunctionType:
    return to_validated_function_type(
        expression.parameters,
        expression.return_type,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        operator_position=expression.operator_position,
        open_parenthesis_fillers=_to_divisor_fillers(
            expression.open_parenthesis_fillers, filler
        ),
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_Grouping)
def _(expression: _Grouping, filler: _Filler, /) -> _Grouping:
    return to_validated_grouping(
        expression.expression,
        open_parenthesis_position=expression.open_parenthesis_position,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=_to_divisor_fillers(
            expression.open_parenthesis_fillers, filler
        ),
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@to_valid_divisor.register(_Identifier)
def _(expression: _Identifier, filler: _Filler, /) -> _Identifier:
    fillers = expression.fillers
    return _to_validated_identifier(
        expression.string,
        position=expression.position,
        fillers=_to_divisor_fillers(fillers, filler),
    )


@to_valid_divisor.register(_MemberAccess)
def _(expression: _MemberAccess, filler: _Filler, /) -> _MemberAccess:
    return to_validated_member_access(
        to_valid_divisor(expression.object, filler),
        expression.member,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_valid_divisor.register(_NumericLiteral)
def _(expression: _NumericLiteral, filler: _Filler, /) -> _NumericLiteral:
    return _to_validated_numeric_literal(
        expression.value,
        expression.type_,
        position=expression.position,
        fillers=_to_divisor_fillers(expression.fillers, filler),
    )


@to_valid_divisor.register(_Return)
def _(expression: _Return, filler: _Filler, /) -> _Return:
    return to_validated_return(
        expression.expression,
        operator_position=expression.operator_position,
        operator_fillers=_to_divisor_fillers(
            expression.operator_fillers, filler
        ),
    )


@to_valid_divisor.register(_Tuple)
def _(expression: _Tuple, filler: _Filler, /) -> _Tuple:
    return to_validated_tuple(
        expression.elements,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=_to_divisor_fillers(
            expression.open_parenthesis_fillers, filler
        ),
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@to_valid_divisor.register(_UnaryArithmeticOperation)
def _(
    expression: _UnaryArithmeticOperation, filler: _Filler, /
) -> _UnaryArithmeticOperation:
    return to_validated_unary_arithmetic_operation(
        expression.operand,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=_to_divisor_fillers(
            expression.operator_fillers, filler
        ),
    )


@to_valid_divisor.register(_UnidirectionalConditional)
def _(
    expression: _UnidirectionalConditional, filler: _Filler, /
) -> _UnidirectionalConditional:
    return to_validated_unidirectional_conditional(
        expression.antecedent,
        expression.consequent,
        opener_position=expression.opener_position,
        opener_fillers=_to_divisor_fillers(expression.opener_fillers, filler),
    )


@to_valid_divisor.register(_WhileLoop)
def _(expression: _WhileLoop, filler: _Filler, /) -> _WhileLoop:
    return to_validated_while_loop(
        expression.condition,
        expression.body,
        opener_position=expression.opener_position,
        opener_fillers=_to_divisor_fillers(expression.opener_fillers, filler),
    )


assert all(
    (
        to_valid_divisor.dispatch(cls)
        is not to_valid_divisor.dispatch(_Expression)
    )
    for cls in _Expression.__subclasses__()
), [
    cls
    for cls in _Expression.__subclasses__()
    if (
        to_valid_divisor.dispatch(cls)
        is to_valid_divisor.dispatch(_Expression)
    )
]


def _to_divisor_fillers(
    fillers: list[_Filler], non_comment_filler: _Filler
) -> list[_Filler]:
    assert not _is_comment_filler(non_comment_filler), non_comment_filler
    return (
        [non_comment_filler] + fillers
        if fillers and _is_comment_filler(fillers[0])
        else fillers
    )


@_singledispatch
def to_non_lexically_conflicting_expression(
    expression: _Expression, _non_comment_filler: _Filler, /
) -> _Expression:
    raise TypeError(type(expression))


@to_non_lexically_conflicting_expression.register(_AnnotatedIdentifier)
def _(
    expression: _AnnotatedIdentifier, non_comment_filler: _Filler, /
) -> _AnnotatedIdentifier:
    identifier = to_non_lexically_conflicting_expression(
        expression.identifier, non_comment_filler
    )
    assert isinstance(identifier, _Identifier), identifier
    return to_validated_annotated_identifier(
        identifier,
        expression.annotation,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_Assignment)
def _(expression: _Assignment, non_comment_filler: _Filler, /) -> _Assignment:
    return to_validated_assignment(
        to_non_lexically_conflicting_expression(
            expression.target, non_comment_filler
        ),
        expression.value,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_BidirectionalConditional)
def to_non_lexically_conflicting_bidirectional_conditional(
    expression: _BidirectionalConditional, non_comment_filler: _Filler, /
) -> _BidirectionalConditional:
    return to_validated_bidirectional_conditional(
        expression.antecedent,
        expression.consequent,
        expression.alternative,
        antecedent_opener_position=expression.antecedent_opener_position,
        alternative_opener_position=expression.alternative_opener_position,
        antecedent_opener_fillers=(
            expression.antecedent_opener_fillers or [non_comment_filler]
        ),
        alternative_opener_fillers=expression.alternative_opener_fillers,
    )


@to_non_lexically_conflicting_expression.register(_BinaryArithmeticOperation)
def _(
    expression: _BinaryArithmeticOperation, non_comment_filler: _Filler, /
) -> _BinaryArithmeticOperation:
    return to_validated_binary_arithmetic_operation(
        to_non_lexically_conflicting_expression(
            expression.left, non_comment_filler
        ),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_BinaryComparison)
def _(
    expression: _BinaryComparison, non_comment_filler: _Filler, /
) -> _BinaryComparison:
    return to_validated_binary_comparison(
        to_non_lexically_conflicting_expression(
            expression.left, non_comment_filler
        ),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_Block)
def _(expression: _Block, _non_comment_filler: _Filler, /) -> _Block:
    return expression


@to_non_lexically_conflicting_expression.register(_Call)
def _(expression: _Call, non_comment_filler: _Filler, /) -> _Call:
    return to_validated_call(
        to_non_lexically_conflicting_expression(
            expression.callable, non_comment_filler
        ),
        expression.arguments,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@to_non_lexically_conflicting_expression.register(_FunctionDefinition)
def _(
    expression: _FunctionDefinition, non_comment_filler: _Filler, /
) -> _FunctionDefinition:
    return to_validated_function_definition(
        expression.parameters,
        expression.return_type,
        expression.body,
        opener_position=expression.opener_position,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        arrow_position=expression.arrow_position,
        opener_fillers=expression.opener_fillers or [non_comment_filler],
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        arrow_fillers=expression.arrow_fillers,
    )


@to_non_lexically_conflicting_expression.register(_FunctionType)
def _(
    expression: _FunctionType, non_comment_filler: _Filler, /
) -> _FunctionType:
    return to_validated_function_type(
        expression.parameters,
        expression.return_type,
        open_parenthesis_position=expression.open_parenthesis_position,
        comma_positions=expression.comma_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        operator_position=expression.operator_position,
        open_parenthesis_fillers=(
            expression.open_parenthesis_fillers or [non_comment_filler]
        ),
        comma_fillers=expression.comma_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_Grouping)
def _(expression: _Grouping, _non_comment_filler: _Filler, /) -> _Grouping:
    return expression


@to_non_lexically_conflicting_expression.register(_Identifier)
def _(expression: _Identifier, non_comment_filler: _Filler, /) -> _Identifier:
    return _to_validated_identifier(
        expression.string,
        position=expression.position,
        fillers=expression.fillers or [non_comment_filler],
    )


@to_non_lexically_conflicting_expression.register(_MemberAccess)
def _(
    expression: _MemberAccess, non_comment_filler: _Filler, /
) -> _MemberAccess:
    return to_validated_member_access(
        to_non_lexically_conflicting_expression(
            expression.object, non_comment_filler
        ),
        expression.member,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@to_non_lexically_conflicting_expression.register(_NumericLiteral)
def _(
    expression: _NumericLiteral, non_comment_filler: _Filler, /
) -> _NumericLiteral:
    return _to_validated_numeric_literal(
        expression.value,
        expression.type_,
        position=expression.position,
        fillers=expression.fillers or [non_comment_filler],
    )


@to_non_lexically_conflicting_expression.register(_Return)
def to_non_lexically_conflicting_return(
    expression: _Return, _non_comment_filler: _Filler, /
) -> _Return:
    return to_validated_return(
        expression.expression,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers or [_non_comment_filler],
    )


@to_non_lexically_conflicting_expression.register(_Tuple)
def _(expression: _Tuple, _non_comment_filler: _Filler, /) -> _Tuple:
    return expression


@to_non_lexically_conflicting_expression.register(_UnaryArithmeticOperation)
def _(
    expression: _UnaryArithmeticOperation, _non_comment_filler: _Filler, /
) -> _UnaryArithmeticOperation:
    return expression


@to_non_lexically_conflicting_expression.register(_UnidirectionalConditional)
def to_non_lexically_conflicting_unidirectional_conditional(
    expression: _UnidirectionalConditional, non_comment_filler: _Filler, /
) -> _UnidirectionalConditional:
    return to_validated_unidirectional_conditional(
        expression.antecedent,
        expression.consequent,
        opener_position=expression.opener_position,
        opener_fillers=(expression.opener_fillers or [non_comment_filler]),
    )


@to_non_lexically_conflicting_expression.register(_WhileLoop)
def _(expression: _WhileLoop, non_comment_filler: _Filler, /) -> _WhileLoop:
    return to_validated_while_loop(
        expression.condition,
        expression.body,
        opener_position=expression.opener_position,
        opener_fillers=(expression.opener_fillers or [non_comment_filler]),
    )


assert all(
    (
        to_non_lexically_conflicting_expression.dispatch(cls)
        is not to_non_lexically_conflicting_expression.dispatch(_Expression)
    )
    for cls in _Expression.__subclasses__()
), [
    cls
    for cls in _Expression.__subclasses__()
    if (
        to_non_lexically_conflicting_expression.dispatch(cls)
        is to_non_lexically_conflicting_expression.dispatch(_Expression)
    )
]


def _to_antecedent_strategy(
    expressions: _st.SearchStrategy[_Expression],
) -> _st.SearchStrategy[_Expression]:
    return _st.builds(
        to_non_lexically_conflicting_expression,
        expressions,
        _non_comment_filler_strategy,
    )
