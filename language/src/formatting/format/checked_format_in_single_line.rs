use crate::parsing::{
    AnnotatedIdentifier, AnnotationOperator, Assignment, AssignmentOperator,
    BidirectionalConditional, BinaryArithmeticOperation, BinaryComparison,
    Block, Call, Expression, ExpressionStatement, Filler, FillerContent,
    FunctionDefinition, FunctionType, FunctionTypeOperator, Grouping,
    Identifier, MemberAccess, MemberAccessOperator, NumericLiteral, Return,
    ReturnOperator, Statement, Tuple, UnaryArithmeticOperation,
    UnidirectionalConditional, WhileLoop, CONDITIONAL_ALTERNATIVE_OPENER,
    CONDITIONAL_ANTECEDENT_OPENER, FUNCTION_DEFINITION_OPENER,
    WHILE_LOOP_OPENER,
};
use crate::tokenization::{
    ByteSize, SubstringPosition, TokenContent, Utf8Size,
};

use super::character_offset::CharacterOffset;
use super::format_context::FormatContext;
use super::format_local_context::FormatLocalContext;
use super::line_offset::LineOffset;
use super::offset::Offset;

pub trait CheckedFormatInSingleLine {
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext>;
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for AnnotatedIdentifier<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: CheckedFormatInSingleLine,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        let operator_offset =
            if self.operator_fillers.iter().all(Filler::is_non_comment) {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
        checked_format_binary_operation_in_single_line(
            &mut self.identifier,
            self.annotation.as_mut(),
            AnnotationOperator,
            &mut self.operator_position,
            &mut self.operator_fillers,
            local_context,
            global_context,
            offset,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
            operator_offset,
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Assignment<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        checked_format_binary_operation_in_single_line(
            self.target.as_mut(),
            self.value.as_mut(),
            AssignmentOperator,
            &mut self.operator_position,
            &mut self.operator_fillers,
            local_context,
            global_context,
            offset,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for BidirectionalConditional<StringType>
where
    Block<StringType>: CheckedFormatInSingleLine,
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = checked_format_unidirectional_conditional_components_in_single_line(
            &mut self.antecedent,
            &mut self.consequent,
            &mut self.antecedent_opener_position,
            &mut self.antecedent_opener_fillers,
            local_context,
            global_context,
            offset,
        )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.alternative_opener_fillers,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                CONDITIONAL_ALTERNATIVE_OPENER,
                &mut self.alternative_opener_position,
                global_context,
            )?;
        local_context = self.alternative.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for BinaryArithmeticOperation<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        checked_format_binary_operation_in_single_line(
            self.left.as_mut(),
            self.right.as_mut(),
            self.operator,
            &mut self.operator_position,
            &mut self.operator_fillers,
            local_context,
            global_context,
            offset,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for BinaryComparison<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        checked_format_binary_operation_in_single_line(
            self.left.as_mut(),
            self.right.as_mut(),
            self.operator,
            &mut self.operator_position,
            &mut self.operator_fillers,
            local_context,
            global_context,
            offset,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Block<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    Statement<StringType>: CheckedFormatInSingleLine,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.open_brace_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenBrace.to_string(),
                &mut self.open_brace_position,
                global_context,
            )?;
        if !self.statements.is_empty() {
            return None;
        }
        if let Some(expression) = &mut self.expression {
            local_context = expression.checked_format_in_single_line(
                local_context,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )?;
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_brace_fillers,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )?;
        } else {
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_brace_fillers,
                    global_context,
                    Offset::default(),
                )?;
        }
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseBrace.to_string(),
                &mut self.close_brace_position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Call<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = self.callable.checked_format_in_single_line(
            local_context,
            global_context,
            offset,
        )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.open_parenthesis_fillers,
                global_context,
                Offset::default(),
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenParenthesis.to_string(),
                &mut self.open_parenthesis_position,
                global_context,
            )?;
        local_context = checked_format_comma_separated_values_in_single_line(
            &mut self.arguments,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            Offset::default(),
        )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.close_parenthesis_fillers,
                global_context,
                Offset::default(),
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseParenthesis.to_string(),
                &mut self.close_parenthesis_position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Expression<StringType>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        match self {
            Expression::AnnotatedIdentifier(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::Assignment(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::BidirectionalConditional(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::BinaryArithmeticOperation(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::BinaryComparison(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::Block(value) => value.checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            ),
            Expression::Call(value) => value.checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            ),
            Expression::FunctionDefinition(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::FunctionType(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::Grouping(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::Identifier(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::MemberAccess(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::NumericLiteral(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::Return(value) => value.checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            ),
            Expression::Tuple(value) => value.checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            ),
            Expression::UnaryArithmeticOperation(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::UnidirectionalConditional(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
            Expression::WhileLoop(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for ExpressionStatement<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = self.expression.checked_format_in_single_line(
            local_context,
            global_context,
            offset,
        )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.semicolon_fillers,
                global_context,
                Offset::default(),
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::Semicolon.to_string(),
                &mut self.semicolon_position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for FunctionDefinition<StringType>
where
    AnnotatedIdentifier<StringType>: CheckedFormatInSingleLine,
    Block<StringType>: CheckedFormatInSingleLine,
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.opener_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                FUNCTION_DEFINITION_OPENER,
                &mut self.opener_position,
                global_context,
            )?;
        {
            let offset = if self
                .open_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.open_parenthesis_fillers,
                    global_context,
                    offset,
                )?;
        }
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenParenthesis.to_string(),
                &mut self.open_parenthesis_position,
                global_context,
            )?;
        local_context = checked_format_comma_separated_values_in_single_line(
            &mut self.parameters,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            Offset::default(),
        )?;
        {
            let offset = if self
                .close_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_parenthesis_fillers,
                    global_context,
                    offset,
                )?
        };
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseParenthesis.to_string(),
                &mut self.close_parenthesis_position,
                global_context,
            )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.arrow_fillers,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::Arrow.to_string(),
                &mut self.arrow_position,
                global_context,
            )?;

        local_context = self.return_type.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        local_context = self.body.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for FunctionType<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.open_parenthesis_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenParenthesis.to_string(),
                &mut self.open_parenthesis_position,
                global_context,
            )?;
        local_context = checked_format_comma_separated_values_in_single_line(
            &mut self.parameters,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            Offset::default(),
        )?;
        {
            let offset = if self
                .close_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_parenthesis_fillers,
                    global_context,
                    offset,
                )?
        };
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseParenthesis.to_string(),
                &mut self.close_parenthesis_position,
                global_context,
            )?;
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.operator_fillers,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::from(FunctionTypeOperator).to_string(),
                &mut self.operator_position,
                global_context,
            )?;
        local_context = self.return_type.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Grouping<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.open_parenthesis_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenParenthesis.to_string(),
                &mut self.open_parenthesis_position,
                global_context,
            )?;
        local_context = self.expression.checked_format_in_single_line(
            local_context,
            global_context,
            Offset::default(),
        )?;
        {
            let offset = if self
                .close_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_parenthesis_fillers,
                    global_context,
                    offset,
                )?
        };
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseParenthesis.to_string(),
                &mut self.close_parenthesis_position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Identifier<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &self.string,
                &mut self.position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for MemberAccess<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: CheckedFormatInSingleLine,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        checked_format_binary_operation_in_single_line(
            self.object.as_mut(),
            &mut self.member,
            MemberAccessOperator,
            &mut self.operator_position,
            &mut self.operator_fillers,
            local_context,
            global_context,
            offset,
            Offset::default(),
            Offset::default(),
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for NumericLiteral<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::NumericLiteral {
                    value: self.value.as_ref(),
                    type_: self.type_,
                }
                .to_string(),
                &mut self.position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType> CheckedFormatInSingleLine for Statement<StringType>
where
    ExpressionStatement<StringType>: CheckedFormatInSingleLine,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        match self {
            Statement::Expression(value) => value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                ),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Return<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.operator_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::from(ReturnOperator).to_string(),
                &mut self.operator_position,
                global_context,
            )?;
        local_context = self.expression.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for Tuple<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.open_parenthesis_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::OpenParenthesis.to_string(),
                &mut self.open_parenthesis_position,
                global_context,
            )?;
        local_context = checked_format_comma_separated_values_in_single_line(
            &mut self.elements,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            Offset::default(),
        )?;
        {
            let offset = if self
                .close_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            };
            local_context = local_context
                .checked_format_filler_vec_in_single_line(
                    &mut self.close_parenthesis_fillers,
                    global_context,
                    offset,
                )?
        };
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::CloseParenthesis.to_string(),
                &mut self.close_parenthesis_position,
                global_context,
            )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.operator_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::from(self.operator).to_string(),
                &mut self.operator_position,
                global_context,
            )?;
        local_context = self.operand.checked_format_in_single_line(
            local_context,
            global_context,
            Offset::default(),
        )?;
        Some(local_context)
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for UnidirectionalConditional<StringType>
where
    Block<StringType>: CheckedFormatInSingleLine,
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        checked_format_unidirectional_conditional_components_in_single_line(
            &mut self.antecedent,
            &mut self.consequent,
            &mut self.opener_position,
            &mut self.opener_fillers,
            local_context,
            global_context,
            offset,
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    CheckedFormatInSingleLine for WhileLoop<StringType>
where
    Block<StringType>: CheckedFormatInSingleLine,
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
{
    fn checked_format_in_single_line(
        &mut self,
        mut local_context: FormatLocalContext,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<FormatLocalContext> {
        local_context = local_context
            .checked_format_filler_vec_in_single_line(
                &mut self.opener_fillers,
                global_context,
                offset,
            )?;
        local_context = local_context
            .checked_format_non_empty_string_without_newline_in_single_line(
                WHILE_LOOP_OPENER,
                &mut self.opener_position,
                global_context,
            )?;
        local_context = self.condition.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        local_context = self.body.checked_format_in_single_line(
            local_context,
            global_context,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        )?;
        Some(local_context)
    }
}

pub(super) fn checked_format_comma_separated_values_with_trailing_comma_in_single_line<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    values: &mut [impl CheckedFormatInSingleLine],
    comma_positions: &mut [SubstringPosition],
    comma_fillers: &mut [Vec<Filler<StringType>>],
    local_context: FormatLocalContext,
    global_context: &FormatContext,
    offset: Offset,
) -> Option<FormatLocalContext>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    debug_assert_eq!(values.len(), comma_positions.len());
    let comma_string = TokenContent::Comma.to_string();
    let mut values_with_comma_positions_and_comma_fillers = values
        .iter_mut()
        .zip(comma_positions.iter_mut().zip(comma_fillers.iter_mut()));
    if let Some((value, (comma_position, comma_fillers))) =
        values_with_comma_positions_and_comma_fillers.next()
    {
        value
            .checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            )
            .and_then(|local_context| {
                let offset = if comma_fillers
                    .iter()
                    .all(Filler::is_non_comment)
                {
                    Offset::default()
                } else {
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    }
                };
                local_context
                    .checked_format_filler_vec_in_single_line(
                        comma_fillers,
                        global_context,
                        offset,
                    )
            })
            .and_then(|local_context| {
                local_context.checked_format_non_empty_string_without_newline_in_single_line(
                    &comma_string,
                    comma_position,
                    global_context,
                )
            })
            .and_then(|local_context| {
                values_with_comma_positions_and_comma_fillers
                    .try_fold(
                        local_context,
                        |local_context, (value, (comma_position, comma_fillers))| {
                            value
                                .checked_format_in_single_line(
                                    local_context,
                                    global_context,
                                    Offset {
                                        line: LineOffset::default(),
                                        character: CharacterOffset::from(1usize),
                                    },
                                )
                                .and_then(|local_context| {
                                    let offset = if comma_fillers
                                        .iter()
                                        .all(Filler::is_non_comment)
                                    {
                                        Offset::default()
                                    } else {
                                        Offset {
                                            line: LineOffset::default(),
                                            character: CharacterOffset::from(1usize),
                                        }
                                    };
                                    local_context
                                        .checked_format_filler_vec_in_single_line(
                                            comma_fillers,
                                            global_context,
                                            offset,
                                        )
                                }).and_then(|local_context| {
                                local_context.checked_format_non_empty_string_without_newline_in_single_line(
                                    &comma_string,
                                    comma_position,
                                    global_context,
                                )
                            })
                        },
                    )
            })
    } else {
        Some(local_context)
    }
}

pub(super) fn checked_format_comma_separated_values_without_trailing_comma_in_single_line<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    values: &mut [impl CheckedFormatInSingleLine],
    comma_positions: &mut [SubstringPosition],
    comma_fillers: &mut [Vec<Filler<StringType>>],
    local_context: FormatLocalContext,
    global_context: &FormatContext,
    offset: Offset,
) -> Option<FormatLocalContext>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    debug_assert_eq!(values.len(), comma_positions.len() + 1usize);
    if let [head_values @ .., last_value] = values {
        let comma_string = TokenContent::Comma.to_string();
        let mut head_values_with_comma_positions_and_comma_fillers =
            head_values
                .iter_mut()
                .zip(comma_positions.iter_mut().zip(comma_fillers.iter_mut()));
        if let Some((value, (comma_position, comma_fillers))) =
            head_values_with_comma_positions_and_comma_fillers.next()
        {
            value
                .checked_format_in_single_line(
                    local_context,
                    global_context,
                    offset,
                )
                .and_then(|local_context| {
                    let offset = if comma_fillers
                        .iter()
                        .all(Filler::is_non_comment)
                    {
                        Offset::default()
                    } else {
                        Offset {
                            line: LineOffset::default(),
                            character: CharacterOffset::from(1usize),
                        }
                    };
                    local_context
                        .checked_format_filler_vec_in_single_line(
                            comma_fillers,
                            global_context,
                            offset,
                        )
                })
                .and_then(|local_context| {
                    local_context.checked_format_non_empty_string_without_newline_in_single_line(
                        &comma_string,
                        comma_position,
                        global_context,
                    )
                })
                .and_then(|local_context| {
                    head_values_with_comma_positions_and_comma_fillers
                        .try_fold(
                            local_context,
                            |local_context, (value, (comma_position, comma_fillers))| {
                                value
                                    .checked_format_in_single_line(
                                        local_context,
                                        global_context,
                                        Offset {
                                            line: LineOffset::default(),
                                            character: CharacterOffset::from(1usize),
                                        },
                                    )
                                    .and_then(|local_context| {
                                        let offset = if comma_fillers
                                            .iter()
                                            .all(Filler::is_non_comment)
                                        {
                                            Offset::default()
                                        } else {
                                            Offset {
                                                line: LineOffset::default(),
                                                character: CharacterOffset::from(1usize),
                                            }
                                        };
                                        local_context
                                            .checked_format_filler_vec_in_single_line(
                                                comma_fillers,
                                                global_context,
                                                offset,
                                            )
                                    }).and_then(|local_context| {
                                    local_context.checked_format_non_empty_string_without_newline_in_single_line(
                                        &comma_string,
                                        comma_position,
                                        global_context,
                                    )
                                })
                            },
                        )
                })
                .and_then(|local_context| {
                    last_value.checked_format_in_single_line(
                        local_context,
                        global_context,
                        Offset {
                            line: LineOffset::default(),
                            character: CharacterOffset::from(1usize),
                        },
                    )
                })
        } else {
            last_value.checked_format_in_single_line(
                local_context,
                global_context,
                offset,
            )
        }
    } else {
        unreachable!("There should be at least one value");
    }
}

#[allow(clippy::too_many_arguments)]
fn checked_format_binary_operation_in_single_line<
    Operator,
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    left_operand: &mut impl CheckedFormatInSingleLine,
    right_operand: &mut impl CheckedFormatInSingleLine,
    operator: Operator,
    operator_position: &mut SubstringPosition,
    operator_fillers: &mut Vec<Filler<StringType>>,
    mut local_context: FormatLocalContext,
    global_context: &FormatContext,
    left_operand_offset: Offset,
    right_operand_offset: Offset,
    operator_offset: Offset,
) -> Option<FormatLocalContext>
where
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: From<Operator> + ToString,
{
    local_context = left_operand.checked_format_in_single_line(
        local_context,
        global_context,
        left_operand_offset,
    )?;
    local_context = local_context.checked_format_filler_vec_in_single_line(
        operator_fillers,
        global_context,
        operator_offset,
    )?;
    local_context = local_context
        .checked_format_non_empty_string_without_newline_in_single_line(
            &TokenContent::from(operator).to_string(),
            operator_position,
            global_context,
        )?;
    local_context = right_operand.checked_format_in_single_line(
        local_context,
        global_context,
        right_operand_offset,
    )?;
    Some(local_context)
}

fn checked_format_comma_separated_values_in_single_line<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    values: &mut [impl CheckedFormatInSingleLine],
    comma_positions: &mut [SubstringPosition],
    comma_fillers: &mut [Vec<Filler<StringType>>],
    local_context: FormatLocalContext,
    global_context: &FormatContext,
    offset: Offset,
) -> Option<FormatLocalContext>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    debug_assert_eq!(comma_positions.len(), comma_fillers.len());
    if values.len() > comma_positions.len() {
        checked_format_comma_separated_values_without_trailing_comma_in_single_line(
            values,
            comma_positions,
            comma_fillers,
            local_context,
            global_context,
            offset,
        )
    } else {
        checked_format_comma_separated_values_with_trailing_comma_in_single_line(
            values,
            comma_positions,
            comma_fillers,
            local_context,
            global_context,
            offset
        )
    }
}

fn checked_format_unidirectional_conditional_components_in_single_line<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    antecedent: &mut Box<Expression<StringType>>,
    consequent: &mut Block<StringType>,
    opener_position: &mut SubstringPosition,
    opener_fillers: &mut Vec<Filler<StringType>>,
    mut local_context: FormatLocalContext,
    global_context: &FormatContext,
    offset: Offset,
) -> Option<FormatLocalContext>
where
    Block<StringType>: CheckedFormatInSingleLine,
    Expression<StringType>: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
{
    local_context = local_context.checked_format_filler_vec_in_single_line(
        opener_fillers,
        global_context,
        offset,
    )?;
    local_context = local_context
        .checked_format_non_empty_string_without_newline_in_single_line(
            CONDITIONAL_ANTECEDENT_OPENER,
            opener_position,
            global_context,
        )?;
    local_context = antecedent.checked_format_in_single_line(
        local_context,
        global_context,
        Offset {
            line: LineOffset::default(),
            character: CharacterOffset::from(1usize),
        },
    )?;
    local_context = consequent.checked_format_in_single_line(
        local_context,
        global_context,
        Offset {
            line: LineOffset::default(),
            character: CharacterOffset::from(1usize),
        },
    )?;
    Some(local_context)
}
