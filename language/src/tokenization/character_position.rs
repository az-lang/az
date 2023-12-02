use super::byte_index::ByteIndex;
use super::utf_8_index::Utf8Index;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterPosition {
    pub byte: ByteIndex,
    pub utf_8: Utf8Index,
}
