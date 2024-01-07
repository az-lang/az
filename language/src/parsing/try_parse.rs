use crate::parsing::operators::{
    BinaryArithmeticOperator, ComparisonOperator, UnaryArithmeticOperator,
};
use std::iter::Peekable;

use crate::tokenization::{PositionedToken, Token};

use super::associativity::Associativity;
use super::binary_operation::BinaryOperator;
use super::precedence::Precedence;
use super::types::{Block, Expression, Identifier, ParsingError, Statement};

pub trait TryParse<'a> {
    fn try_parse(self) -> Result<Vec<Statement<'a>>, ParsingError<'a>>;
}

impl<'a> TryParse<'a> for Vec<PositionedToken<'a>> {
    fn try_parse(self) -> Result<Vec<Statement<'a>>, ParsingError<'a>> {
        let mut parser = Parser {
            tokens: self
                .into_iter()
                .filter(|positioned_token| {
                    !matches!(
                        positioned_token.token,
                        Token::CommentBlock(..)
                            | Token::CommentLine(..)
                            | Token::Newline
                            | Token::Whitespace(..)
                    )
                })
                .peekable(),
        };
        let mut result = vec![];
        while parser.tokens.peek().is_some() {
            let (next_statement, next_parser) = parse_statement(parser)?;
            parser = next_parser;
            result.push(next_statement);
        }
        Ok(result)
    }
}

const CONDITIONAL_ALTERNATIVE_STARTER: &str = "else";
const CONDITIONAL_ANTECEDENT_STARTER: &str = "if";
const FUNCTION_STARTER: &str = "Function";

struct Parser<'a, Tokens: Iterator<Item = PositionedToken<'a>>> {
    tokens: Peekable<Tokens>,
}

fn parse_call<'a, Tokens: Iterator<Item = PositionedToken<'a>>>(
    callable: Expression<'a>,
    mut parser: Parser<'a, Tokens>,
) -> Result<(Expression<'a>, Parser<'a, Tokens>), ParsingError<'a>> {
    let opened_parenthesis =
        unsafe { parser.tokens.next().unwrap_unchecked() };
    let arguments = match parser.tokens.peek() {
        Some(next_positioned_token) => match next_positioned_token.token {
            Token::CloseParenthesis => {
                drop(parser.tokens.next());
                vec![]
            }
            _ => {
                let (argument, next_parser) = parse_expression(parser)?;
                parser = next_parser;
                let mut arguments = vec![argument];
                loop {
                    match parser.tokens.next() {
                        Some(next_positioned_token) => {
                            match next_positioned_token.token {
                                Token::CloseParenthesis => {
                                    break;
                                }
                                Token::Comma => {
                                    let (next_argument, next_parser) =
                                        parse_expression(parser)?;
                                    parser = next_parser;
                                    arguments.push(next_argument);
                                }
                                _ => {
                                    return Err(
                                        ParsingError::MismatchedOpenParentheses(
                                            opened_parenthesis,
                                        ),
                                    );
                                }
                            }
                        }
                        None => {
                            return Err(
                                ParsingError::MismatchedOpenParentheses(
                                    opened_parenthesis,
                                ),
                            );
                        }
                    }
                }
                arguments
            }
        },
        None => {
            return Err(ParsingError::MismatchedOpenParentheses(
                opened_parenthesis,
            ));
        }
    };
    Ok((
        Expression::Call {
            callable: Box::new(callable),
            arguments,
        },
        parser,
    ))
}

fn parse_expression<'a, Tokens: Iterator<Item = PositionedToken<'a>>>(
    parser: Parser<'a, Tokens>,
) -> Result<(Expression<'a>, Parser<'a, Tokens>), ParsingError<'a>> {
    let (result, parser) = parse_term(parser)?;
    parse_sub_expression(result, parser, Precedence::default())
}

fn parse_statement<'a, Tokens: Iterator<Item = PositionedToken<'a>>>(
    parser: Parser<'a, Tokens>,
) -> Result<(Statement<'a>, Parser<'a, Tokens>), ParsingError<'a>> {
    let (expression, mut parser) = parse_expression(parser)?;
    let positioned_token = parser
        .tokens
        .next()
        .ok_or_else(|| ParsingError::OutOfTokens)?;
    match positioned_token.token {
        Token::Semicolon => Ok((Statement::Expression(expression), parser)),
        _ => Err(ParsingError::MissingSemicolon(positioned_token)),
    }
}

fn parse_sub_expression<'a, Tokens: Iterator<Item = PositionedToken<'a>>>(
    mut result: Expression<'a>,
    mut parser: Parser<'a, Tokens>,
    min_precedence: Precedence,
) -> Result<(Expression<'a>, Parser<'a, Tokens>), ParsingError<'a>> {
    while let Some(next_positioned_token) = parser.tokens.peek() {
        let (operator, precedence) = {
            if matches!(next_positioned_token.token, Token::OpenParenthesis) {
                (result, parser) = parse_call(result, parser)?;
                continue;
            }
            if let Ok(operator) =
                BinaryOperator::try_from(&next_positioned_token.token)
            {
                let precedence = Precedence::from(operator);
                if precedence < min_precedence {
                    break;
                }
                drop(parser.tokens.next());
                (operator, precedence)
            } else {
                break;
            }
        };
        let (mut term, next_parser) = parse_term(parser)?;
        parser = next_parser;
        while let Some(next_positioned_token) = parser.tokens.peek() {
            if let Ok(next_operator) =
                BinaryOperator::try_from(&next_positioned_token.token)
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
                let (next_term, next_parser) = parse_sub_expression(
                    term,
                    parser,
                    if precedence < next_precedence {
                        precedence.increment()
                    } else {
                        precedence
                    },
                )?;
                (term, parser) = (next_term, next_parser);
            } else {
                break;
            }
        }
        result = match operator {
            BinaryOperator::Addition => {
                Expression::BinaryArithmeticOperation {
                    left: Box::new(result),
                    right: Box::new(term),
                    operator: BinaryArithmeticOperator::Addition,
                }
            }
            BinaryOperator::Annotation => match result {
                Expression::Identifier(identifier) => {
                    Expression::AnnotatedIdentifier {
                        identifier,
                        annotation: Box::new(term),
                    }
                }
                _ => {
                    return Err(ParsingError::UnexpectedExpression(term));
                }
            },
            BinaryOperator::Assignment => Expression::Assignment {
                target: Box::new(result),
                value: Box::new(term),
            },
            BinaryOperator::Division => {
                Expression::BinaryArithmeticOperation {
                    left: Box::new(result),
                    right: Box::new(term),
                    operator: BinaryArithmeticOperator::Division,
                }
            }
            BinaryOperator::EqualTo => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::EqualTo,
            },
            BinaryOperator::GreaterThan => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::GreaterThan,
            },
            BinaryOperator::GreaterThanOrEqualTo => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::GreaterThanOrEqualTo,
            },
            BinaryOperator::LowerThan => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::LowerThan,
            },
            BinaryOperator::LowerThanOrEqualTo => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::LowerThanOrEqualTo,
            },
            BinaryOperator::MemberAccess => match term {
                Expression::Identifier(identifier) => {
                    Expression::MemberAccess {
                        object: Box::new(result),
                        member: identifier,
                    }
                }
                _ => {
                    return Err(ParsingError::UnexpectedExpression(term));
                }
            },
            BinaryOperator::Multiplication => {
                Expression::BinaryArithmeticOperation {
                    left: Box::new(result),
                    right: Box::new(term),
                    operator: BinaryArithmeticOperator::Multiplication,
                }
            }
            BinaryOperator::NotEqualTo => Expression::Comparison {
                left: Box::new(result),
                right: Box::new(term),
                operator: ComparisonOperator::NotEqualTo,
            },
            BinaryOperator::Subtraction => {
                Expression::BinaryArithmeticOperation {
                    left: Box::new(result),
                    right: Box::new(term),
                    operator: BinaryArithmeticOperator::Subtraction,
                }
            }
        };
    }
    Ok((result, parser))
}

fn parse_term<'a, Tokens: Iterator<Item = PositionedToken<'a>>>(
    mut parser: Parser<'a, Tokens>,
) -> Result<(Expression<'a>, Parser<'a, Tokens>), ParsingError<'a>> {
    let positioned_token = parser
        .tokens
        .next()
        .ok_or_else(|| ParsingError::OutOfTokens)?;
    match positioned_token.token {
        Token::Identifier(string) => match string {
            FUNCTION_STARTER => {
                let next_positioned_token = parser
                    .tokens
                    .next()
                    .ok_or_else(|| ParsingError::OutOfTokens)?;
                let (parameters, next_parser) = match next_positioned_token
                    .token
                {
                    Token::OpenParenthesis => {
                        let open_parenthesis = next_positioned_token;
                        let mut parameters = vec![];
                        loop {
                            match parser.tokens.peek() {
                                Some(next_positioned_token) => {
                                    match next_positioned_token.token {
                                        Token::CloseParenthesis => {
                                            drop(parser.tokens.next());
                                            break;
                                        }
                                        Token::Identifier(_) => {
                                            let (next_parameter, next_parser) =
                                                parse_expression(parser)?;
                                            parser = next_parser;
                                            parameters.push(next_parameter);
                                            if matches!(
                                                parser.tokens.peek(),
                                                Some(PositionedToken {
                                                    token: Token::Comma,
                                                    ..
                                                })
                                            ) {
                                                drop(parser.tokens.next());
                                            }
                                        }
                                        _ => {
                                            return Err(ParsingError::MismatchedOpenParentheses(
                                                open_parenthesis
                                        ));
                                        }
                                    }
                                }
                                None => {
                                    return Err(ParsingError::MismatchedOpenParentheses(open_parenthesis));
                                }
                            }
                        }
                        (parameters, parser)
                    }
                    _ => {
                        return Err(ParsingError::UnexpectedToken(
                            next_positioned_token,
                        ));
                    }
                };
                parser = next_parser;
                let (return_type, next_parser) = match parser
                    .tokens
                    .peek()
                    .ok_or_else(|| ParsingError::OutOfTokens)?
                    .token
                {
                    Token::Arrow => {
                        drop(parser.tokens.next());
                        parse_term(parser)?
                    }
                    _ => {
                        return Err(ParsingError::UnexpectedToken(unsafe {
                            parser.tokens.next().unwrap_unchecked()
                        }));
                    }
                };
                parser = next_parser;
                let (body, next_parser) = parse_term(parser)?;
                match body {
                    Expression::Block(body) => {
                        parser = next_parser;
                        Ok((
                            Expression::FunctionDefinition {
                                parameters,
                                return_type: Box::new(return_type),
                                body,
                            },
                            parser,
                        ))
                    }
                    _ => {
                        return Err(ParsingError::UnexpectedExpression(body));
                    }
                }
            }
            CONDITIONAL_ANTECEDENT_STARTER => {
                let (antecedent, next_parser) = parse_expression(parser)?;
                parser = next_parser;
                match parser
                    .tokens
                    .peek()
                    .ok_or_else(|| ParsingError::OutOfTokens)?
                    .token
                {
                    Token::OpenBrace => {
                        let (consequent, next_parser) = parse_term(parser)?;
                        parser = next_parser;
                        let alternative = if matches!(
                            parser
                                .tokens
                                .peek()
                                .ok_or_else(|| ParsingError::OutOfTokens)?
                                .token,
                            Token::Identifier(CONDITIONAL_ALTERNATIVE_STARTER)
                        ) {
                            drop(parser.tokens.next());
                            match parser
                                .tokens
                                .peek()
                                .ok_or_else(|| ParsingError::OutOfTokens)?
                                .token
                            {
                                Token::OpenBrace
                                | Token::Identifier(
                                    CONDITIONAL_ANTECEDENT_STARTER,
                                ) => {
                                    let (alternative, next_parser) =
                                        parse_term(parser)?;
                                    parser = next_parser;
                                    Some(Box::new(alternative))
                                }
                                _ => {
                                    return Err(ParsingError::UnexpectedToken(
                                        unsafe {
                                            parser
                                                .tokens
                                                .next()
                                                .unwrap_unchecked()
                                        },
                                    ))
                                }
                            }
                        } else {
                            None
                        };
                        Ok((
                            Expression::Conditional {
                                antecedent: Box::new(antecedent),
                                consequent: Box::new(consequent),
                                alternative,
                            },
                            parser,
                        ))
                    }
                    _ => {
                        return Err(ParsingError::UnexpectedToken(unsafe {
                            parser.tokens.next().unwrap_unchecked()
                        }));
                    }
                }
            }
            _ => Ok((
                Expression::Identifier(Identifier {
                    string,
                    position: positioned_token.position,
                }),
                parser,
            )),
        },
        Token::Minus => {
            let (result, next_parser) = parse_term(parser)?;
            parser = next_parser;
            Ok((
                Expression::UnaryArithmeticOperation {
                    operand: Box::new(result),
                    operator: UnaryArithmeticOperator::Negation,
                },
                parser,
            ))
        }
        Token::NumericLiteral { value, type_ } => Ok((
            Expression::NumericLiteral {
                value,
                type_,
                position: positioned_token.position,
            },
            parser,
        )),
        Token::OpenBrace => {
            let mut statements = vec![];
            let mut expression = None;
            loop {
                match parser.tokens.peek() {
                    Some(next_positioned_token) => {
                        match next_positioned_token.token {
                            Token::CloseBrace => {
                                drop(parser.tokens.next());
                                break;
                            }
                            _ => {
                                let (next_expression, next_parser) =
                                    parse_expression(parser)?;
                                parser = next_parser;
                                match parser.tokens.next() {
                                    Some(next_positioned_token) => {
                                        match next_positioned_token.token {
                                            Token::CloseBrace => {
                                                expression = Some(Box::new(
                                                    next_expression,
                                                ));
                                                break;
                                            }
                                            Token::Semicolon => {
                                                statements.push(
                                                    Statement::Expression(
                                                        next_expression,
                                                    ),
                                                );
                                            }
                                            _ => {
                                                return Err(ParsingError::MismatchedOpenBrace(
                                            positioned_token,
                                        ));
                                            }
                                        }
                                    }
                                    None => {
                                        return Err(
                                            ParsingError::MismatchedOpenBrace(
                                                positioned_token,
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        return Err(ParsingError::MismatchedOpenBrace(
                            positioned_token,
                        ))
                    }
                }
            }
            Ok((
                Expression::Block(Block {
                    statements,
                    expression,
                }),
                parser,
            ))
        }
        Token::OpenParenthesis => {
            let next_positioned_token = match parser.tokens.peek() {
                Some(value) => value,
                None => {
                    return Err(ParsingError::MismatchedOpenParentheses(
                        positioned_token,
                    ))
                }
            };
            match next_positioned_token.token {
                Token::CloseParenthesis => {
                    drop(parser.tokens.next());
                    Ok((Expression::Tuple { elements: vec![] }, parser))
                }
                _ => {
                    let (result, next_parser) = parse_expression(parser)?;
                    parser = next_parser;
                    let next_positioned_token = match parser.tokens.next() {
                        Some(value) => value,
                        None => {
                            return Err(
                                ParsingError::MismatchedOpenParentheses(
                                    positioned_token,
                                ),
                            );
                        }
                    };
                    match next_positioned_token.token {
                        Token::CloseParenthesis => Ok((result, parser)),
                        Token::Comma => {
                            let mut elements = vec![result];
                            loop {
                                match parser.tokens.peek() {
                                    Some(next_positioned_token) => {
                                        match next_positioned_token.token {
                                            Token::CloseParenthesis => {
                                                drop(parser.tokens.next());
                                                break;
                                            }
                                            _ => {
                                                let (
                                                    next_element,
                                                    next_parser,
                                                ) = parse_expression(parser)?;
                                                parser = next_parser;
                                                elements.push(next_element);
                                                if matches!(
                                                    parser.tokens.peek(),
                                                    Some(PositionedToken {
                                                        token: Token::Comma,
                                                        ..
                                                    })
                                                ) {
                                                    drop(parser.tokens.next());
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        return Err(ParsingError::MismatchedOpenParentheses(
                                            positioned_token,
                                        ));
                                    }
                                }
                            }
                            Ok((Expression::Tuple { elements }, parser))
                        }
                        _ => Err(ParsingError::MismatchedOpenParentheses(
                            positioned_token,
                        )),
                    }
                }
            }
        }
        _ => Err(ParsingError::UnexpectedToken(positioned_token)),
    }
}
