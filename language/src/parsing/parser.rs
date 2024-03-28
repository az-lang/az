use crate::parsing::UnaryArithmeticOperation;
use crate::tokenization::{SubstringPosition, Token, TokenContent};

use super::associativity::Associativity;
use super::binary_operator::BinaryOperator;
use super::expressions::{
    AnnotatedIdentifier, Assignment, BinaryArithmeticOperation,
    BinaryComparison, Block, Call, Conditional, Expression,
    FunctionDefinition, Grouping, Identifier, MemberAccess, NumericLiteral,
    Tuple,
};
use super::filler::{Filler, Fillers};
use super::filler_content::FillerContent;
use super::keywords::{
    CONDITIONAL_ALTERNATIVE_OPENER, CONDITIONAL_ANTECEDENT_OPENER,
    FUNCTION_OPENER,
};
use super::operators::{
    BinaryArithmeticOperator, BinaryComparisonOperator,
    UnaryArithmeticOperator,
};
use super::parsing_error::{
    MismatchedOpenBrace, MismatchedOpenParenthesis, MissingSemicolon,
    OutOfTokens, ParsingError, UnexpectedExpression, UnexpectedToken,
};
use super::precedence::Precedence;
use super::statement::{ExpressionStatement, Statement};
use super::unary_operator::UnaryOperator;

pub(super) struct Parser<
    StringType,
    TokenStringType,
    Tokens: Iterator<Item = Token<TokenStringType>>,
> {
    cursor: Option<Token<TokenStringType>>,
    fillers: Vec<Filler<StringType>>,
    tokens: Tokens,
}

impl<
        StringType,
        TokenStringType: Into<StringType>,
        Tokens: Iterator<Item = Token<TokenStringType>>,
    > Parser<StringType, TokenStringType, Tokens>
{
    pub(super) fn new(tokens: Tokens) -> Self {
        let mut result = Self {
            cursor: None,
            fillers: vec![],
            tokens,
        };
        result.update_cursor();
        result
    }

    pub(super) fn peek(&self) -> Option<&Token<TokenStringType>> {
        self.cursor.as_ref()
    }

    pub(super) fn into_fillers(self) -> Fillers<StringType> {
        debug_assert!(self.cursor.is_none());
        self.fillers
    }

    fn update_cursor(&mut self) {
        debug_assert!(self.cursor.is_none());
        for token in self.tokens.by_ref() {
            match token.content {
                TokenContent::CommentBlock(strings) => {
                    self.fillers.push(Filler {
                        content: FillerContent::CommentBlock(
                            strings.into_iter().map(Into::into).collect(),
                        ),
                        position: token.position,
                    });
                }
                TokenContent::CommentLine(string) => {
                    self.fillers.push(Filler {
                        content: FillerContent::CommentLine(string.into()),
                        position: token.position,
                    });
                }
                TokenContent::Newline => {
                    self.fillers.push(Filler {
                        content: FillerContent::Newline,
                        position: token.position,
                    });
                }
                TokenContent::Whitespace(string) => {
                    self.fillers.push(Filler {
                        content: FillerContent::Whitespace(string.into()),
                        position: token.position,
                    });
                }
                _ => {
                    self.cursor = Some(token);
                    break;
                }
            }
        }
    }
}

impl<
        StringType,
        TokenStringType: AsRef<str> + Into<StringType> + PartialEq,
        Tokens: Iterator<Item = Token<TokenStringType>>,
    > Parser<StringType, TokenStringType, Tokens>
{
    pub(super) fn parse_statement(
        self,
        token: Token<TokenStringType>,
        fillers: Fillers<StringType>,
    ) -> Result<
        (Statement<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (expression, mut parser) =
            self.parse_expression(token, fillers)?;
        let (token, fillers) = parser
            .next()
            .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
        match token.content {
            TokenContent::Semicolon => Ok((
                Statement::Expression(ExpressionStatement {
                    expression,
                    semicolon_position: token.position,
                    semicolon_fillers: fillers,
                }),
                parser,
            )),
            _ => {
                Err(ParsingError::MissingSemicolon(MissingSemicolon { token }))
            }
        }
    }

    fn parse_expression(
        self,
        token: Token<TokenStringType>,
        fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (result, parser) = self.parse_term(token, fillers)?;
        parser.parse_sub_expression(result, Precedence::minimum())
    }

    fn parse_sub_expression(
        self,
        mut result: Expression<StringType>,
        min_precedence: Precedence,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let mut parser = self;
        while let Some(next_token) = parser.peek() {
            if let Ok(operator) = BinaryOperator::try_from(&next_token.content)
            {
                let precedence = Precedence::from(operator);
                if precedence < min_precedence {
                    break;
                }
                result = if matches!(operator, BinaryOperator::Call) {
                    let (operand, next_parser) =
                        parser.parse_operand(precedence)?;
                    parser = next_parser;
                    match operand {
                        Expression::Tuple(Tuple {
                            elements,
                            open_parenthesis_position,
                            commas_positions,
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            commas_fillers,
                            close_parenthesis_fillers,
                        }) => Expression::Call(Call {
                            callable: Box::new(result),
                            arguments: elements,
                            open_parenthesis_position,
                            commas_positions,
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            commas_fillers,
                            close_parenthesis_fillers,
                        }),
                        Expression::Grouping(Grouping {
                            expression: argument,
                            open_parenthesis_position,
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            close_parenthesis_fillers,
                        }) => Expression::Call(Call {
                            callable: Box::new(result),
                            arguments: vec![*argument],
                            open_parenthesis_position,
                            commas_positions: vec![],
                            close_parenthesis_position,
                            open_parenthesis_fillers,
                            commas_fillers: vec![],
                            close_parenthesis_fillers,
                        }),
                        _ => {
                            return Err(ParsingError::UnexpectedExpression(
                                UnexpectedExpression {
                                    expression: operand,
                                },
                            ));
                        }
                    }
                } else {
                    let (token, operator_fillers) =
                        unsafe { parser.next().unwrap_unchecked() };
                    let operator_position = token.position;
                    let (operand, next_parser) =
                        parser.parse_operand(precedence)?;
                    parser = next_parser;
                    match operator {
                        BinaryOperator::Addition => {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left: Box::new(result),
                                    right: Box::new(operand),
                                    operator: BinaryArithmeticOperator::Addition,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        }
                        BinaryOperator::Annotation => match result {
                            Expression::Identifier(identifier) => {
                                Expression::AnnotatedIdentifier(
                                    AnnotatedIdentifier {
                                        identifier,
                                        annotation: Box::new(operand),
                                        operator_position,
                                        operator_fillers,
                                    },
                                )
                            }
                            _ => {
                                return Err(
                                    ParsingError::UnexpectedExpression(UnexpectedExpression {
                                        expression: result,
                                    }),
                                );
                            }
                        },
                        BinaryOperator::Assignment => {
                            Expression::Assignment(Assignment {
                                target: Box::new(result),
                                value: Box::new(operand),
                                operator_position,
                                operator_fillers,
                            })
                        }
                        BinaryOperator::Call => {
                            unreachable!(
                                "Call operator is handled separately."
                            )
                        }
                        BinaryOperator::Division => {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left: Box::new(result),
                                    right: Box::new(operand),
                                    operator: BinaryArithmeticOperator::Division,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        }
                        BinaryOperator::EqualTo => Expression::BinaryComparison(BinaryComparison {
                            left: Box::new(result),
                            right: Box::new(operand),
                            operator: BinaryComparisonOperator::EqualTo,
                            operator_position,
                            operator_fillers,
                        }),
                        BinaryOperator::GreaterThan => {
                            Expression::BinaryComparison(BinaryComparison {
                                left: Box::new(result),
                                right: Box::new(operand),
                                operator: BinaryComparisonOperator::GreaterThan,
                                operator_position,
                                operator_fillers,
                            })
                        }
                        BinaryOperator::GreaterThanOrEqualTo => {
                            Expression::BinaryComparison(BinaryComparison {
                                left: Box::new(result),
                                right: Box::new(operand),
                                operator:
                                    BinaryComparisonOperator::GreaterThanOrEqualTo,
                                operator_position,
                                operator_fillers,
                            })
                        }
                        BinaryOperator::LowerThan => Expression::BinaryComparison(BinaryComparison {
                            left: Box::new(result),
                            right: Box::new(operand),
                            operator: BinaryComparisonOperator::LowerThan,
                            operator_position,
                            operator_fillers,
                        }),
                        BinaryOperator::LowerThanOrEqualTo => {
                            Expression::BinaryComparison(BinaryComparison {
                                left: Box::new(result),
                                right: Box::new(operand),
                                operator:
                                    BinaryComparisonOperator::LowerThanOrEqualTo,
                                operator_position,
                                operator_fillers,
                            })
                        }
                        BinaryOperator::MemberAccess => match operand {
                            Expression::Identifier(identifier) => {
                                Expression::MemberAccess(MemberAccess {
                                    object: Box::new(result),
                                    member: identifier,
                                    operator_position,
                                    operator_fillers,
                                })
                            }
                            _ => {
                                return Err(
                                    ParsingError::UnexpectedExpression(UnexpectedExpression {
                                        expression: operand,
                                    }),
                                );
                            }
                        },
                        BinaryOperator::Multiplication => {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left: Box::new(result),
                                    right: Box::new(operand),
                                    operator: BinaryArithmeticOperator::Multiplication,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        }
                        BinaryOperator::NotEqualTo => Expression::BinaryComparison(BinaryComparison {
                            left: Box::new(result),
                            right: Box::new(operand),
                            operator: BinaryComparisonOperator::NotEqualTo,
                            operator_position,
                            operator_fillers,
                        }),
                        BinaryOperator::Subtraction => {
                            Expression::BinaryArithmeticOperation(
                                BinaryArithmeticOperation {
                                    left: Box::new(result),
                                    right: Box::new(operand),
                                    operator: BinaryArithmeticOperator::Subtraction,
                                    operator_position,
                                    operator_fillers,
                                },
                            )
                        }
                    }
                };
            } else {
                break;
            }
        }
        Ok((result, parser))
    }

    fn parse_term(
        self,
        token: Token<TokenStringType>,
        fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        if let Ok(operator) = UnaryOperator::try_from(&token.content) {
            let operator_position = token.position;
            let precedence = Precedence::from(operator);
            let (operand, parser) = self.parse_operand(precedence)?;
            match operator {
                UnaryOperator::Negation => Ok((
                    Expression::UnaryArithmeticOperation(
                        UnaryArithmeticOperation {
                            operand: Box::new(operand),
                            operator: UnaryArithmeticOperator::Negation,
                            operator_position,
                            operator_fillers: fillers,
                        },
                    ),
                    parser,
                )),
            }
        } else {
            match token.content {
                TokenContent::Identifier(string) => match string.as_ref() {
                    FUNCTION_OPENER => {
                        self.parse_function_definition(token.position, fillers)
                    }
                    CONDITIONAL_ANTECEDENT_OPENER => {
                        self.parse_conditional(token.position, fillers)
                    }
                    CONDITIONAL_ALTERNATIVE_OPENER => {
                        Err(ParsingError::UnexpectedToken(UnexpectedToken {
                            token: Token {
                                content: TokenContent::Identifier(string),
                                position: token.position,
                            },
                        }))
                    }
                    _ => Ok((
                        Expression::Identifier(Identifier {
                            string: string.into(),
                            position: token.position,
                            fillers,
                        }),
                        self,
                    )),
                },
                TokenContent::NumericLiteral { value, type_ } => Ok((
                    Expression::NumericLiteral(NumericLiteral {
                        value: value.into(),
                        type_,
                        position: token.position,
                        fillers,
                    }),
                    self,
                )),
                TokenContent::OpenBrace => {
                    self.parse_block(token.position, fillers)
                }
                TokenContent::OpenParenthesis => {
                    self.parse_tuple_or_grouping(token.position, fillers)
                }
                _ => Err(ParsingError::UnexpectedToken(UnexpectedToken {
                    token,
                })),
            }
        }
    }

    fn parse_block(
        self,
        open_brace_position: SubstringPosition,
        open_brace_fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let mut statements = vec![];
        let mut expression = None;
        let mut parser = self;
        let (close_brace_position, close_brace_fillers) = loop {
            match parser.next() {
                Some((next_token, fillers)) => match next_token.content {
                    TokenContent::CloseBrace => {
                        break (next_token.position, fillers);
                    }
                    _ => {
                        let (next_expression, next_parser) =
                            parser.parse_expression(next_token, fillers)?;
                        parser = next_parser;
                        match parser.next() {
                            Some((next_token, next_fillers)) => {
                                match next_token.content {
                                    TokenContent::CloseBrace => {
                                        expression =
                                            Some(Box::new(next_expression));
                                        break (
                                            next_token.position,
                                            next_fillers,
                                        );
                                    }
                                    TokenContent::Semicolon => {
                                        statements.push(
                                            Statement::Expression(
                                                ExpressionStatement {
                                                    expression:
                                                        next_expression,
                                                    semicolon_position:
                                                        next_token.position,
                                                    semicolon_fillers:
                                                        next_fillers,
                                                },
                                            ),
                                        );
                                    }
                                    _ => {
                                        return Err(
                                            ParsingError::MismatchedOpenBrace(
                                                MismatchedOpenBrace {
                                                    position:
                                                        open_brace_position,
                                                },
                                            ),
                                        );
                                    }
                                }
                            }
                            None => {
                                return Err(
                                    ParsingError::MismatchedOpenBrace(
                                        MismatchedOpenBrace {
                                            position: open_brace_position,
                                        },
                                    ),
                                );
                            }
                        }
                    }
                },
                None => {
                    return Err(ParsingError::MismatchedOpenBrace(
                        MismatchedOpenBrace {
                            position: open_brace_position,
                        },
                    ));
                }
            }
        };
        Ok((
            Expression::Block(Block {
                statements,
                expression,
                open_brace_position,
                close_brace_position,
                open_brace_fillers,
                close_brace_fillers,
            }),
            parser,
        ))
    }

    fn parse_conditional(
        mut self,
        opener_position: SubstringPosition,
        opener_fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (antecedent, mut parser) = {
            let (token, fillers) = self
                .next()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
            self.parse_expression(token, fillers)?
        };
        let (expression, mut parser) = {
            let (token, fillers) = parser
                .next()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
            parser.parse_term(token, fillers)?
        };
        let consequent = match expression {
            Expression::Block(value) => value,
            value => {
                return Err(ParsingError::UnexpectedExpression(
                    UnexpectedExpression { expression: value },
                ));
            }
        };
        let (
            alternative,
            alternative_opener_position,
            alternative_opener_fillers,
        ) = if matches!(
            parser
                .peek()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?
                .content,
            TokenContent::Identifier(ref string)
            if CONDITIONAL_ALTERNATIVE_OPENER == string.as_ref()
        ) {
            let (alternative_opener, alternative_opener_fillers) =
                unsafe { parser.next().unwrap_unchecked() };
            let alternative_opener_position = alternative_opener.position;
            let (next_token, next_fillers) = parser
                .next()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
            let (alternative, next_parser) =
                parser.parse_term(next_token, next_fillers)?;
            if !matches!(
                alternative,
                Expression::Block(_) | Expression::Conditional { .. }
            ) {
                return Err(ParsingError::UnexpectedExpression(
                    UnexpectedExpression {
                        expression: alternative,
                    },
                ));
            }
            parser = next_parser;
            (
                Some(Box::new(alternative)),
                Some(alternative_opener_position),
                alternative_opener_fillers,
            )
        } else {
            (None, None, Fillers::default())
        };
        Ok((
            Expression::Conditional(Conditional {
                antecedent: Box::new(antecedent),
                consequent,
                alternative,
                opener_position,
                alternative_opener_position,
                opener_fillers,
                alternative_opener_fillers,
            }),
            parser,
        ))
    }

    fn parse_function_definition(
        mut self,
        opener_position: SubstringPosition,
        opener_fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (token, signature_opening_fillers) = self
            .next()
            .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
        if !matches!(token.content, TokenContent::OpenParenthesis) {
            return Err(ParsingError::UnexpectedToken(UnexpectedToken {
                token,
            }));
        }
        let open_parenthesis = token;
        let mut commas_positions = vec![];
        let mut parameters = vec![];
        let mut parameters_fillers = vec![];
        let (mut token, mut fillers) = match self.next() {
            Some(value) => value,
            None => {
                return Err(ParsingError::MismatchedOpenParenthesis(
                    MismatchedOpenParenthesis {
                        position: open_parenthesis.position,
                    },
                ));
            }
        };
        let mut parser = self;
        let (close_parenthesis_position, signature_closing_fillers) = loop {
            match token.content {
                TokenContent::CloseParenthesis => {
                    break (token.position, fillers);
                }
                _ => {
                    let (expression, next_parser) =
                        parser.parse_expression(token, fillers)?;
                    if !matches!(
                        expression,
                        Expression::AnnotatedIdentifier { .. }
                    ) {
                        return Err(ParsingError::UnexpectedExpression(
                            UnexpectedExpression { expression },
                        ));
                    }
                    parser = next_parser;
                    parameters.push(expression);
                    let (next_token, next_fillers) = match parser.next() {
                        Some(value) => value,
                        None => {
                            return Err(
                                ParsingError::MismatchedOpenParenthesis(
                                    MismatchedOpenParenthesis {
                                        position: open_parenthesis.position,
                                    },
                                ),
                            );
                        }
                    };
                    match next_token.content {
                        TokenContent::Comma => {
                            commas_positions.push(next_token.position);
                            parameters_fillers.push(next_fillers);
                            (token, fillers) = match parser.next() {
                                Some(value) => value,
                                None => {
                                    return Err(
                                        ParsingError::MismatchedOpenParenthesis(
                                            MismatchedOpenParenthesis {
                                                position: open_parenthesis
                                                    .position,
                                            },
                                        ),
                                    );
                                }
                            };
                        }
                        TokenContent::CloseParenthesis => {
                            break (next_token.position, next_fillers);
                        }
                        _ => {
                            return Err(ParsingError::UnexpectedToken(
                                UnexpectedToken { token: next_token },
                            ));
                        }
                    }
                }
            }
        };
        let (next_token, return_type_fillers) = parser
            .next()
            .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
        let (arrow_position, (return_type, mut parser)) =
            match next_token.content {
                TokenContent::Arrow => {
                    let arrow_position = next_token.position;
                    let (token, fillers) = parser.next().ok_or_else(|| {
                        ParsingError::OutOfTokens(OutOfTokens)
                    })?;
                    (arrow_position, parser.parse_expression(token, fillers)?)
                }
                _ => {
                    return Err(ParsingError::UnexpectedToken(
                        UnexpectedToken { token: next_token },
                    ));
                }
            };
        let (expression, parser) = {
            let (token, fillers) = parser
                .next()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
            parser.parse_term(token, fillers)?
        };
        match expression {
            Expression::Block(body) => Ok((
                Expression::FunctionDefinition(FunctionDefinition {
                    parameters,
                    return_type: Box::new(return_type),
                    body,
                    opener_position,
                    open_parenthesis_position: open_parenthesis.position,
                    commas_positions,
                    close_parenthesis_position,
                    arrow_position,
                    opener_fillers,
                    open_parenthesis_fillers: signature_opening_fillers,
                    commas_fillers: parameters_fillers,
                    close_parenthesis_fillers: signature_closing_fillers,
                    arrow_fillers: return_type_fillers,
                }),
                parser,
            )),
            _ => {
                Err(ParsingError::UnexpectedExpression(UnexpectedExpression {
                    expression,
                }))
            }
        }
    }

    fn parse_tuple_or_grouping(
        mut self,
        open_parenthesis_position: SubstringPosition,
        open_parenthesis_fillers: Fillers<StringType>,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (next_token, fillers) = match self.next() {
            Some(value) => value,
            None => {
                return Err(ParsingError::MismatchedOpenParenthesis(
                    MismatchedOpenParenthesis {
                        position: open_parenthesis_position,
                    },
                ));
            }
        };
        match next_token.content {
            TokenContent::CloseParenthesis => {
                let close_parenthesis_fillers = fillers;
                Ok((
                    Expression::Tuple(Tuple {
                        elements: vec![],
                        open_parenthesis_position,
                        commas_positions: vec![],
                        close_parenthesis_position: next_token.position,
                        open_parenthesis_fillers,
                        commas_fillers: vec![],
                        close_parenthesis_fillers,
                    }),
                    self,
                ))
            }
            _ => {
                let (expression, next_parser) =
                    self.parse_expression(next_token, fillers)?;
                let mut parser = next_parser;
                let (token, fillers) = match parser.next() {
                    Some(value) => value,
                    None => {
                        return Err(ParsingError::MismatchedOpenParenthesis(
                            MismatchedOpenParenthesis {
                                position: open_parenthesis_position,
                            },
                        ));
                    }
                };
                match token.content {
                    TokenContent::CloseParenthesis => {
                        let close_parenthesis_fillers = fillers;
                        Ok((
                            Expression::Grouping(Grouping {
                                expression: Box::new(expression),
                                open_parenthesis_position,
                                close_parenthesis_position: token.position,
                                open_parenthesis_fillers,
                                close_parenthesis_fillers,
                            }),
                            parser,
                        ))
                    }
                    TokenContent::Comma => {
                        let mut commas_fillers = vec![fillers];
                        let mut commas_positions = vec![token.position];
                        let mut elements = vec![expression];
                        let (mut token, mut fillers) = match parser.next() {
                            Some(value) => value,
                            None => {
                                return Err(
                                    ParsingError::MismatchedOpenParenthesis(
                                        MismatchedOpenParenthesis {
                                            position:
                                                open_parenthesis_position,
                                        },
                                    ),
                                );
                            }
                        };
                        let (
                            close_parenthesis_position,
                            close_parenthesis_fillers,
                        ) = loop {
                            match token.content {
                                TokenContent::CloseParenthesis => {
                                    break (token.position, fillers);
                                }
                                _ => {
                                    let (expression, next_parser) = {
                                        parser
                                            .parse_expression(token, fillers)?
                                    };
                                    parser = next_parser;
                                    elements.push(expression);
                                    let (next_token, next_fillers) =
                                        match parser.next() {
                                            Some(value) => value,
                                            None => {
                                                return Err(
                                                    ParsingError::MismatchedOpenParenthesis(
                                                        MismatchedOpenParenthesis {
                                                            position: open_parenthesis_position,
                                                        },
                                                    ),
                                                );
                                            }
                                        };
                                    match next_token.content {
                                        TokenContent::Comma => {
                                            commas_positions
                                                .push(next_token.position);
                                            commas_fillers.push(next_fillers);
                                            (token, fillers) = match parser
                                                .next()
                                            {
                                                Some(value) => value,
                                                None => {
                                                    return Err(
                                                        ParsingError::MismatchedOpenParenthesis(
                                                            MismatchedOpenParenthesis {
                                                                position: open_parenthesis_position,
                                                            },
                                                        ),
                                                    );
                                                }
                                            };
                                        }
                                        TokenContent::CloseParenthesis => {
                                            break (
                                                next_token.position,
                                                next_fillers,
                                            );
                                        }
                                        _ => {
                                            return Err(
                                                ParsingError::UnexpectedToken(
                                                    UnexpectedToken {
                                                        token: next_token,
                                                    },
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        };
                        Ok((
                            Expression::Tuple(Tuple {
                                elements,
                                open_parenthesis_position,
                                commas_positions,
                                close_parenthesis_position,
                                open_parenthesis_fillers,
                                commas_fillers,
                                close_parenthesis_fillers,
                            }),
                            parser,
                        ))
                    }
                    _ => Err(ParsingError::MismatchedOpenParenthesis(
                        MismatchedOpenParenthesis {
                            position: open_parenthesis_position,
                        },
                    )),
                }
            }
        }
    }

    fn parse_operand(
        mut self,
        precedence: Precedence,
    ) -> Result<
        (Expression<StringType>, Self),
        ParsingError<StringType, TokenStringType>,
    > {
        let (mut operand, mut parser) = {
            let (token, fillers) = self
                .next()
                .ok_or_else(|| ParsingError::OutOfTokens(OutOfTokens))?;
            self.parse_term(token, fillers)?
        };
        while let Some(next_token) = parser.peek() {
            if let Ok(next_operator) =
                BinaryOperator::try_from(&next_token.content)
            {
                let next_precedence = Precedence::from(next_operator);
                if !(next_precedence > precedence
                    || (next_precedence == precedence
                        && matches!(
                            Associativity::from(next_operator),
                            Associativity::RightToLeft
                        )))
                {
                    break;
                }
                let (expression, next_parser) = parser.parse_sub_expression(
                    operand,
                    if precedence < next_precedence {
                        precedence.increment()
                    } else {
                        precedence.clone()
                    },
                )?;
                (operand, parser) = (expression, next_parser);
            } else {
                break;
            }
        }
        Ok((operand, parser))
    }
}

impl<
        StringType,
        TokenStringType: Into<StringType>,
        Tokens: Iterator<Item = Token<TokenStringType>>,
    > Iterator for Parser<StringType, TokenStringType, Tokens>
{
    type Item = (Token<TokenStringType>, Fillers<StringType>);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .cursor
            .take()
            .map(|token| (token, std::mem::take(&mut self.fillers)));
        self.update_cursor();
        result
    }
}
