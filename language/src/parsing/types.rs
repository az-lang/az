use crate::tokenization::{
    NumericLiteralType, PositionedToken, SubstringPosition,
};

use super::operators::{
    BinaryArithmeticOperator, ComparisonOperator, UnaryArithmeticOperator,
};

#[derive(Debug)]
pub enum Expression<'a> {
    AnnotatedIdentifier {
        identifier: Identifier<'a>,
        annotation: Box<Expression<'a>>,
    },
    Assignment {
        target: Box<Expression<'a>>,
        value: Box<Expression<'a>>,
    },
    BinaryArithmeticOperation {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: BinaryArithmeticOperator,
    },
    Block(Block<'a>),
    Call {
        callable: Box<Expression<'a>>,
        arguments: Vec<Expression<'a>>,
    },
    Comparison {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: ComparisonOperator,
    },
    Conditional {
        antecedent: Box<Expression<'a>>,
        consequent: Box<Expression<'a>>,
        alternative: Option<Box<Expression<'a>>>,
    },
    FunctionDefinition {
        parameters: Vec<Expression<'a>>,
        return_type: Box<Expression<'a>>,
        body: Block<'a>,
    },
    Identifier(Identifier<'a>),
    MemberAccess {
        object: Box<Expression<'a>>,
        member: Identifier<'a>,
    },
    NumericLiteral {
        value: &'a str,
        type_: NumericLiteralType,
        position: SubstringPosition,
    },
    Tuple {
        elements: Vec<Expression<'a>>,
    },
    UnaryArithmeticOperation {
        operand: Box<Expression<'a>>,
        operator: UnaryArithmeticOperator,
    },
}

#[derive(Debug)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
    pub expression: Option<Box<Expression<'a>>>,
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub string: &'a str,
    pub position: SubstringPosition,
}

pub enum ParsingError<'a> {
    MismatchedOpenBrace(PositionedToken<'a>),
    MismatchedOpenParentheses(PositionedToken<'a>),
    MissingSemicolon(PositionedToken<'a>),
    OutOfTokens,
    UnexpectedExpression(Expression<'a>),
    UnexpectedToken(PositionedToken<'a>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
}
