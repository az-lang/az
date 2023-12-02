#[derive(Debug)]
pub enum BinaryArithmeticOperator {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LowerThan,
    LowerThanOrEqualTo,
    NotEqualTo,
}

#[derive(Debug)]
pub enum UnaryArithmeticOperator {
    Negation,
}
