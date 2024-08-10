use crate::parsing::{
    AnnotatedIdentifier, AnnotationOperator, Assignment, AssignmentOperator,
    Associativity, BidirectionalConditional, BinaryArithmeticOperation,
    BinaryComparison, Block, Call, Expression, ExpressionStatement, Filler,
    FillerContent, FunctionDefinition, FunctionType, FunctionTypeOperator,
    Grouping, Identifier, MemberAccess, MemberAccessOperator, NumericLiteral,
    Return, ReturnOperator, Statement, ToFirstFillers, Tuple,
    UnaryArithmeticOperation, UnidirectionalConditional, WhileLoop,
    CONDITIONAL_ALTERNATIVE_OPENER, CONDITIONAL_ANTECEDENT_OPENER,
    FUNCTION_DEFINITION_OPENER, WHILE_LOOP_OPENER,
};
use crate::tokenization::{
    ByteSize, SubstringPosition, TokenContent, Utf8Size,
};

use super::character_offset::CharacterOffset;
use super::checked_format_in_single_line::{
    checked_format_comma_separated_values_with_trailing_comma_in_single_line,
    checked_format_comma_separated_values_without_trailing_comma_in_single_line,
    CheckedFormatInSingleLine,
};
use super::format_context::FormatContext;
use super::format_local_context::FormatLocalContext;
use super::line_offset::LineOffset;
use super::offset::Offset;

pub trait FormatInMultipleLines {
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    );
}

const BINARY_OPERATOR_DEPTH_INCREMENT: usize = 1usize;
const RIGHT_BINARY_OPERAND_DEPTH_INCREMENT: usize =
    BINARY_OPERATOR_DEPTH_INCREMENT + 1usize;

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for AnnotatedIdentifier<StringType>
where
    FillerContent<StringType>: ToString,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    Identifier<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        let annotation_offset = if self
            .annotation
            .to_first_fillers()
            .iter()
            .all(Filler::is_non_comment)
        {
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            }
        } else {
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
                ),
            }
        };
        let operator_offset =
            if self.operator_fillers.iter().all(Filler::is_non_comment) {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::from(1usize),
                    character: global_context.depth_to_character_offset(
                        depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                    ),
                }
            };
        format_binary_operation_in_multiple_lines(
            &mut self.identifier,
            self.annotation.as_mut(),
            AnnotationOperator,
            &mut self.operator_fillers,
            &mut self.operator_position,
            local_context,
            global_context,
            depth,
            offset,
            annotation_offset,
            operator_offset,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Assignment<StringType>
where
    FillerContent<StringType>: ToString,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        let value_offset = if self
            .value
            .to_first_fillers()
            .iter()
            .all(Filler::is_non_comment)
        {
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            }
        } else {
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
                ),
            }
        };
        format_binary_operation_in_multiple_lines(
            self.target.as_mut(),
            self.value.as_mut(),
            AssignmentOperator,
            &mut self.operator_fillers,
            &mut self.operator_position,
            local_context,
            global_context,
            depth,
            offset,
            value_offset,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                ),
            },
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for BidirectionalConditional<StringType>
where
    Block<StringType>: FormatInMultipleLines,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.antecedent_opener_fillers,
            global_context,
            depth,
            offset.clone(),
        );
        local_context
            .clone()
            .checked_format_non_empty_string_without_newline_in_single_line(
                CONDITIONAL_ANTECEDENT_OPENER,
                &mut self.antecedent_opener_position,
                global_context,
            )
            .and_then(|local_context| {
                self.antecedent.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.reset_non_empty_string_position(
                    CONDITIONAL_ANTECEDENT_OPENER,
                    &mut self.antecedent_opener_position,
                );
                self.antecedent.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
        self.consequent.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        );
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.alternative_opener_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        );
        local_context.reset_non_empty_string_position(
            CONDITIONAL_ALTERNATIVE_OPENER,
            &mut self.alternative_opener_position,
        );
        self.alternative.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for BinaryArithmeticOperation<StringType>
where
    FillerContent<StringType>: ToString,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        let right_offset = if self
            .right
            .to_first_fillers()
            .iter()
            .all(Filler::is_non_comment)
        {
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            }
        } else {
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
                ),
            }
        };
        format_binary_operation_in_multiple_lines(
            self.left.as_mut(),
            self.right.as_mut(),
            self.operator,
            &mut self.operator_fillers,
            &mut self.operator_position,
            local_context,
            global_context,
            depth,
            offset,
            right_offset,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                ),
            },
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for BinaryComparison<StringType>
where
    FillerContent<StringType>: ToString,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        let right_offset = if self
            .right
            .to_first_fillers()
            .iter()
            .all(Filler::is_non_comment)
        {
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            }
        } else {
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
                ),
            }
        };
        format_binary_operation_in_multiple_lines(
            self.left.as_mut(),
            self.right.as_mut(),
            self.operator,
            &mut self.operator_fillers,
            &mut self.operator_position,
            local_context,
            global_context,
            depth,
            offset,
            right_offset,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                ),
            },
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Block<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    Statement<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.open_brace_fillers,
            global_context,
            depth,
            offset,
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenBrace.to_string(),
            &mut self.open_brace_position,
        );
        {
            let depth = depth + 1usize;
            for statement in &mut self.statements {
                statement
                    .checked_format_in_single_line(
                        local_context.clone(),
                        global_context,
                        Offset {
                            line: LineOffset::from(1usize),
                            character: global_context
                                .depth_to_character_offset(depth),
                        },
                    )
                    .map(|replacement| {
                        *local_context = replacement;
                    })
                    .unwrap_or_else(|| {
                        statement.format_in_multiple_lines(
                            local_context,
                            global_context,
                            depth,
                            Offset {
                                line: LineOffset::from(1usize),
                                character: global_context
                                    .depth_to_character_offset(depth),
                            },
                        )
                    });
            }
            if let Some(expression) = &mut self.expression {
                expression.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::from(1usize),
                        character: global_context
                            .depth_to_character_offset(depth),
                    },
                );
            }
        }
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_brace_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseBrace.to_string(),
            &mut self.close_brace_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Call<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        self.callable.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            offset,
        );
        {
            let offset = if self
                .open_parenthesis_fillers
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset::default()
            } else {
                Offset {
                    line: LineOffset::from(1usize),
                    character: global_context.depth_to_character_offset(
                        depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                    ),
                }
            };
            local_context.format_filler_vec_in_multiple_lines(
                &mut self.open_parenthesis_fillers,
                global_context,
                depth,
                offset,
            );
        }
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        format_comma_separated_values_in_multiple_lines(
            &mut self.arguments,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
        );
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_parenthesis_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                ),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Expression<StringType>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        match self {
            Expression::AnnotatedIdentifier(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::Assignment(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::BidirectionalConditional(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::BinaryArithmeticOperation(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::BinaryComparison(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::Block(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::Call(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::FunctionDefinition(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::FunctionType(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::Grouping(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::Identifier(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::MemberAccess(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::NumericLiteral(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::Return(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::Tuple(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
            Expression::UnaryArithmeticOperation(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::UnidirectionalConditional(value) => value
                .format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    offset,
                ),
            Expression::WhileLoop(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for ExpressionStatement<StringType>
where
    Expression<StringType>: FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        self.expression.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            offset,
        );
        {
            let offset =
                if self.semicolon_fillers.iter().all(Filler::is_non_comment) {
                    Offset::default()
                } else {
                    Offset {
                        line: LineOffset::from(1usize),
                        character: global_context
                            .depth_to_character_offset(depth),
                    }
                };
            local_context.format_filler_vec_in_multiple_lines(
                &mut self.semicolon_fillers,
                global_context,
                depth,
                offset,
            );
        }
        local_context.reset_non_empty_string_position(
            &TokenContent::Semicolon.to_string(),
            &mut self.semicolon_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for FunctionDefinition<StringType>
where
    AnnotatedIdentifier<StringType>:
        CheckedFormatInSingleLine + FormatInMultipleLines,
    Block<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context
            .clone()
            .checked_format_filler_vec_in_single_line(
                &mut self.opener_fillers,
                global_context,
                offset.clone(),
            )
            .and_then(|local_context| {
                local_context.checked_format_non_empty_string_without_newline_in_single_line(
                    FUNCTION_DEFINITION_OPENER,
                    &mut self.opener_position,
                    global_context
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.format_filler_vec_in_multiple_lines(
                    &mut self.opener_fillers,
                    global_context,
                    depth,
                    offset,
                );
                local_context.reset_non_empty_string_position(
                    FUNCTION_DEFINITION_OPENER,
                    &mut self.opener_position,
                );
            });
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
            local_context
                .clone()
                .checked_format_filler_vec_in_single_line(
                    &mut self.open_parenthesis_fillers,
                    global_context,
                    offset,
                )
        }
        .map(|replacement| {
            *local_context = replacement;
        })
        .unwrap_or_else(|| {
            local_context.format_filler_vec_in_multiple_lines(
                &mut self.open_parenthesis_fillers,
                global_context,
                depth,
                Offset::default(),
            );
        });
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        format_comma_separated_values_in_multiple_lines(
            &mut self.parameters,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            depth + 1usize,
        );
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_parenthesis_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
        local_context
            .clone()
            .checked_format_filler_vec_in_single_line(
                &mut self.arrow_fillers,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )
            .and_then(|local_context| {
                local_context.checked_format_non_empty_string_without_newline_in_single_line(
                    &TokenContent::Arrow.to_string(),
                    &mut self.arrow_position,
                    global_context,
                )
            })
            .and_then(|local_context| {
                self.return_type.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.format_filler_vec_in_multiple_lines(
                    &mut self.arrow_fillers,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
                local_context.reset_non_empty_string_position(
                    &TokenContent::Arrow.to_string(),
                    &mut self.arrow_position,
                );
                self.return_type.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
        {
            let offset = if self
                .body
                .to_first_fillers()
                .iter()
                .all(Filler::is_non_comment)
            {
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                }
            } else {
                Offset {
                    line: LineOffset::from(1usize),
                    character: global_context.depth_to_character_offset(depth),
                }
            };
            self.body
                .checked_format_in_single_line(
                    local_context.clone(),
                    global_context,
                    offset.clone(),
                )
                .map(|replacement| {
                    *local_context = replacement;
                })
                .unwrap_or_else(|| {
                    self.body.format_in_multiple_lines(
                        local_context,
                        global_context,
                        depth,
                        offset,
                    );
                });
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for FunctionType<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context
            .clone()
            .checked_format_filler_vec_in_single_line(
                &mut self.open_parenthesis_fillers,
                global_context,
                offset,
            )
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.format_filler_vec_in_multiple_lines(
                    &mut self.open_parenthesis_fillers,
                    global_context,
                    depth,
                    Offset::default(),
                );
            });
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        format_comma_separated_values_in_multiple_lines(
            &mut self.parameters,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            depth + 1usize,
        );
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_parenthesis_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
        local_context
            .clone()
            .checked_format_filler_vec_in_single_line(
                &mut self.operator_fillers,
                global_context,
                Offset {
                    line: LineOffset::default(),
                    character: CharacterOffset::from(1usize),
                },
            )
            .and_then(|local_context| {
                local_context.checked_format_non_empty_string_without_newline_in_single_line(
                    &TokenContent::from(FunctionTypeOperator).to_string(),
                    &mut self.operator_position,
                    global_context,
                )
            })
            .and_then(|local_context| {
                self.return_type.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.format_filler_vec_in_multiple_lines(
                    &mut self.operator_fillers,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
                local_context.reset_non_empty_string_position(
                    &TokenContent::from(FunctionTypeOperator).to_string(),
                    &mut self.operator_position,
                );
                self.return_type.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Grouping<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.open_parenthesis_fillers,
            global_context,
            depth,
            offset,
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        {
            let depth = depth + 1usize;
            self.expression
                .checked_format_in_single_line(
                    local_context.clone(),
                    global_context,
                    Offset {
                        line: LineOffset::from(1usize),
                        character: global_context
                            .depth_to_character_offset(depth),
                    },
                )
                .map(|replacement| {
                    *local_context = replacement;
                })
                .unwrap_or_else(|| {
                    self.expression.format_in_multiple_lines(
                        local_context,
                        global_context,
                        depth,
                        Offset {
                            line: LineOffset::from(1usize),
                            character: global_context
                                .depth_to_character_offset(depth),
                        },
                    );
                });
        }
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_parenthesis_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Identifier<StringType>
where
    Self: CheckedFormatInSingleLine,
    FillerContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.fillers,
            global_context,
            depth,
            offset,
        );
        local_context.reset_non_empty_string_position(
            self.string.as_ref(),
            &mut self.position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for MemberAccess<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        let member_offset = if self
            .member
            .to_first_fillers()
            .iter()
            .all(Filler::is_non_comment)
        {
            Offset::default()
        } else {
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT,
                ),
            }
        };
        format_binary_operation_in_multiple_lines(
            self.object.as_mut(),
            &mut self.member,
            MemberAccessOperator,
            &mut self.operator_fillers,
            &mut self.operator_position,
            local_context,
            global_context,
            depth,
            offset,
            member_offset,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(
                    depth + BINARY_OPERATOR_DEPTH_INCREMENT,
                ),
            },
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for NumericLiteral<StringType>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.fillers,
            global_context,
            depth,
            offset,
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::NumericLiteral {
                value: self.value.as_ref(),
                type_: self.type_,
            }
            .to_string(),
            &mut self.position,
        );
    }
}

impl<StringType> FormatInMultipleLines for Statement<StringType>
where
    ExpressionStatement<StringType>: FormatInMultipleLines,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        match self {
            Statement::Expression(value) => value.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                offset,
            ),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Return<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.operator_fillers,
            global_context,
            depth,
            offset.clone(),
        );
        local_context
            .clone()
            .checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::from(ReturnOperator).to_string(),
                &mut self.operator_position,
                global_context,
            )
            .and_then(|local_context| {
                self.expression.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.reset_non_empty_string_position(
                    &TokenContent::from(ReturnOperator).to_string(),
                    &mut self.operator_position,
                );
                self.expression.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for Tuple<StringType>
where
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.open_parenthesis_fillers,
            global_context,
            depth,
            offset,
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        format_comma_separated_values_in_multiple_lines(
            &mut self.elements,
            &mut self.comma_positions,
            &mut self.comma_fillers,
            local_context,
            global_context,
            depth + 1usize,
        );
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.close_parenthesis_fillers,
            global_context,
            depth,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            },
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: FormatInMultipleLines,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.clone().checked_format_filler_vec_in_single_line(
            &mut self.operator_fillers,
            global_context,
            offset.clone(),
        ).and_then(|local_context| {
            local_context.checked_format_non_empty_string_without_newline_in_single_line(
                &TokenContent::from(self.operator).to_string(),
                &mut self.operator_position,
                global_context,
            )
        }).map(|replacement| {
            *local_context = replacement;
        }).unwrap_or_else(|| {
            local_context.format_filler_vec_in_multiple_lines(
                &mut self.operator_fillers,
                global_context,
                depth,
                offset,
            );
            local_context.reset_non_empty_string_position(
                &TokenContent::from(self.operator).to_string(),
                &mut self.operator_position,
            );
        });
        self.operand.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            Offset::default(),
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for UnidirectionalConditional<StringType>
where
    Block<StringType>: FormatInMultipleLines,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.opener_fillers,
            global_context,
            depth,
            offset.clone(),
        );
        local_context
            .clone()
            .checked_format_non_empty_string_without_newline_in_single_line(
                CONDITIONAL_ANTECEDENT_OPENER,
                &mut self.opener_position,
                global_context,
            )
            .and_then(|local_context| {
                self.antecedent.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.reset_non_empty_string_position(
                    CONDITIONAL_ANTECEDENT_OPENER,
                    &mut self.opener_position,
                );
                self.antecedent.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
        self.consequent.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>>
    FormatInMultipleLines for WhileLoop<StringType>
where
    Block<StringType>: FormatInMultipleLines,
    Expression<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
    FillerContent<StringType>: ToString,
{
    fn format_in_multiple_lines(
        &mut self,
        local_context: &mut FormatLocalContext,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) {
        local_context.format_filler_vec_in_multiple_lines(
            &mut self.opener_fillers,
            global_context,
            depth,
            offset.clone(),
        );
        local_context
            .clone()
            .checked_format_non_empty_string_without_newline_in_single_line(
                WHILE_LOOP_OPENER,
                &mut self.opener_position,
                global_context,
            )
            .and_then(|local_context| {
                self.condition.checked_format_in_single_line(
                    local_context,
                    global_context,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                )
            })
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                local_context.reset_non_empty_string_position(
                    WHILE_LOOP_OPENER,
                    &mut self.opener_position,
                );
                self.condition.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    Offset {
                        line: LineOffset::default(),
                        character: CharacterOffset::from(1usize),
                    },
                );
            });
        self.body.format_in_multiple_lines(
            local_context,
            global_context,
            depth,
            Offset {
                line: LineOffset::default(),
                character: CharacterOffset::from(1usize),
            },
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn format_binary_operation_in_multiple_lines<
    LeftOperand: CheckedFormatInSingleLine + FormatInMultipleLines,
    RightOperand: CheckedFormatInSingleLine
        + FormatInMultipleLines
        + ToFirstFillers<StringType>,
    Operator: Copy,
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    left_operand: &mut LeftOperand,
    right_operand: &mut RightOperand,
    operator: Operator,
    operator_fillers: &mut Vec<Filler<StringType>>,
    operator_position: &mut SubstringPosition,
    local_context: &mut FormatLocalContext,
    global_context: &FormatContext,
    depth: usize,
    left_operand_offset: Offset,
    right_operand_offset: Offset,
    operator_offset: Offset,
) where
    Associativity: From<Operator>,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: From<Operator> + ToString,
{
    let associativity = Associativity::from(operator);
    match associativity {
        Associativity::LeftToRight => {
            left_operand.format_in_multiple_lines(
                local_context,
                global_context,
                depth,
                left_operand_offset,
            );
        }
        Associativity::RightToLeft => {
            left_operand
                .checked_format_in_single_line(
                    local_context.clone(),
                    global_context,
                    left_operand_offset.clone(),
                )
                .map(|replacement| {
                    *local_context = replacement;
                })
                .unwrap_or_else(|| {
                    left_operand.format_in_multiple_lines(
                        local_context,
                        global_context,
                        depth,
                        left_operand_offset,
                    );
                });
        }
    }
    {
        let depth = depth + BINARY_OPERATOR_DEPTH_INCREMENT;
        local_context.format_filler_vec_in_multiple_lines(
            operator_fillers,
            global_context,
            depth,
            operator_offset,
        );
        local_context.reset_non_empty_string_position(
            &TokenContent::from(operator).to_string(),
            operator_position,
        );
    }
    {
        let depth = depth + RIGHT_BINARY_OPERAND_DEPTH_INCREMENT;
        match associativity {
            Associativity::LeftToRight => {
                right_operand
                    .checked_format_in_single_line(
                        local_context.clone(),
                        global_context,
                        right_operand_offset.clone(),
                    )
                    .map(|replacement| {
                        *local_context = replacement;
                    })
                    .unwrap_or_else(|| {
                        right_operand.format_in_multiple_lines(
                            local_context,
                            global_context,
                            depth,
                            right_operand_offset,
                        );
                    });
            }
            Associativity::RightToLeft => {
                right_operand.format_in_multiple_lines(
                    local_context,
                    global_context,
                    depth,
                    right_operand_offset,
                );
            }
        }
    }
}

fn format_comma_separated_values_in_multiple_lines<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    values: &mut [impl CheckedFormatInSingleLine + FormatInMultipleLines],
    comma_positions: &mut [SubstringPosition],
    comma_fillers: &mut [Vec<Filler<StringType>>],
    local_context: &mut FormatLocalContext,
    global_context: &FormatContext,
    depth: usize,
) where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    debug_assert_eq!(comma_positions.len(), comma_fillers.len());
    if values.len() > comma_positions.len() {
        checked_format_comma_separated_values_without_trailing_comma_in_single_line(
            values,
            comma_positions,
            comma_fillers,
            local_context.clone(),
            global_context,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            }
        )
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                if let [head_values @ .., last_value] = values {
                    let comma_string = TokenContent::Comma.to_string();
                    for (value, (comma_position, comma_fillers)) in head_values
                        .iter_mut()
                        .zip(comma_positions.iter_mut().zip(comma_fillers.iter_mut()))
                    {
                        value
                            .checked_format_in_single_line(
                                local_context.clone(),
                                global_context,
                                Offset {
                                    line: LineOffset::from(1usize),
                                    character: global_context.depth_to_character_offset(depth),
                                },
                            )
                            .map(|replacement| {
                                *local_context = replacement;
                            })
                            .unwrap_or_else(|| {
                                value.format_in_multiple_lines(
                                    local_context,
                                    global_context,
                                    depth,
                                    Offset {
                                        line: LineOffset::from(1usize),
                                        character: global_context
                                            .depth_to_character_offset(depth),
                                    },
                                );
                            });
                        {
                            let offset =
                                if comma_fillers.iter().all(Filler::is_non_comment) {
                                    Offset::default()
                                } else {
                                    Offset {
                                        line: LineOffset::default(),
                                        character: CharacterOffset::from(1usize),
                                    }
                                };
                            local_context
                                .clone()
                                .checked_format_filler_vec_in_single_line(
                                    comma_fillers,
                                    global_context,
                                    offset,
                                )
                                .and_then(|local_context| {
                                    local_context.checked_format_non_empty_string_without_newline_in_single_line(
                                        &comma_string,
                                        comma_position,
                                        global_context,
                                    )
                                })
                                .map(|replacement| {
                                    *local_context = replacement;
                                })
                                .unwrap_or_else(|| {
                                    local_context.format_filler_vec_in_multiple_lines(
                                        comma_fillers,
                                        global_context,
                                        depth,
                                        Offset {
                                            line: LineOffset::from(1usize),
                                            character: global_context
                                                .depth_to_character_offset(depth),
                                        },
                                    );
                                    local_context
                                        .reset_non_empty_string_position(
                                            &comma_string,
                                            comma_position,
                                        );
                                });
                        }
                    }

                    last_value
                        .checked_format_in_single_line(
                            local_context.clone(),
                            global_context,
                            Offset {
                                line: LineOffset::from(1usize),
                                character: global_context.depth_to_character_offset(depth),
                            },
                        )
                        .map(|replacement| {
                            *local_context = replacement;
                        })
                        .unwrap_or_else(|| {
                            last_value.format_in_multiple_lines(
                                local_context,
                                global_context,
                                depth,
                                Offset {
                                    line: LineOffset::from(1usize),
                                    character: global_context
                                        .depth_to_character_offset(depth),
                                },
                            );
                        });
                } else {
                    unreachable!("There should be at least one value");
                }
            });
    } else {
        checked_format_comma_separated_values_with_trailing_comma_in_single_line(
            values,
            comma_positions,
            comma_fillers,
            local_context.clone(),
            global_context,
            Offset {
                line: LineOffset::from(1usize),
                character: global_context.depth_to_character_offset(depth),
            }
        )
            .map(|replacement| {
                *local_context = replacement;
            })
            .unwrap_or_else(|| {
                let comma_string = TokenContent::Comma.to_string();
                for (value, (comma_position, comma_fillers)) in values
                    .iter_mut()
                    .zip(comma_positions.iter_mut().zip(comma_fillers.iter_mut()))
                {
                    value
                        .checked_format_in_single_line(
                            local_context.clone(),
                            global_context,
                            Offset {
                                line: LineOffset::from(1usize),
                                character: global_context.depth_to_character_offset(depth),
                            },
                        )
                        .map(|replacement| {
                            *local_context = replacement;
                        })
                        .unwrap_or_else(|| {
                            value.format_in_multiple_lines(
                                local_context,
                                global_context,
                                depth,
                                Offset {
                                    line: LineOffset::from(1usize),
                                    character: global_context
                                        .depth_to_character_offset(depth),
                                },
                            );
                        });
                    {
                        let offset =
                            if comma_fillers.iter().all(Filler::is_non_comment) {
                                Offset::default()
                            } else {
                                Offset {
                                    line: LineOffset::default(),
                                    character: CharacterOffset::from(1usize),
                                }
                            };
                        local_context
                            .clone()
                            .checked_format_filler_vec_in_single_line(
                                comma_fillers,
                                global_context,
                                offset,
                            )
                            .map(|replacement| {
                                *local_context = replacement;
                            })
                            .unwrap_or_else(|| {
                                local_context.format_filler_vec_in_multiple_lines(
                                    comma_fillers,
                                    global_context,
                                    depth,
                                    Offset {
                                        line: LineOffset::from(1usize),
                                        character: global_context
                                            .depth_to_character_offset(depth),
                                    },
                                );
                            });
                    }
                    local_context.reset_non_empty_string_position(
                        &comma_string,
                        comma_position,
                    );
                }
            });
    }
}
