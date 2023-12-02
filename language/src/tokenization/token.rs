use super::substring_position::SubstringPosition;
use super::token_content::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Token<StringType> {
    pub content: TokenContent<StringType>,
    pub position: SubstringPosition,
}
