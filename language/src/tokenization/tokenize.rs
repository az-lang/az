use super::token::Token;

pub trait Tokenize<StringType> {
    fn tokenize(self) -> Vec<Token<StringType>>;
}
