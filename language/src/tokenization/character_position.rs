use super::byte_count::ByteCount;
use super::utf_8_count::Utf8Count;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterPosition {
    pub byte: ByteCount,
    pub utf_8: Utf8Count,
}

impl CharacterPosition {
    pub(crate) fn move_by(&self, offset: CharacterPosition) -> Self {
        Self {
            byte: self.byte + offset.byte,
            utf_8: self.utf_8 + offset.utf_8,
        }
    }
}
