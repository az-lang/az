use super::token_collection::TokenCollection;

pub trait Tokenize<StringType> {
    fn tokenize(self) -> TokenCollection<StringType>;
}
