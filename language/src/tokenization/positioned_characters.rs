use std::str::CharIndices;

use super::byte_count::ByteCount;
use super::character_position::CharacterPosition;
use super::utf_8_count::Utf8Count;
use super::utf_8_size::Utf8Size;

pub(super) struct PositionedCharacters<'a> {
    indexed_characters: CharIndices<'a>,
    utf_8_index: Utf8Count,
}

impl<'a> From<&'a str> for PositionedCharacters<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            indexed_characters: value.char_indices(),
            utf_8_index: Default::default(),
        }
    }
}

impl<'a> Iterator for PositionedCharacters<'a> {
    type Item = (CharacterPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_characters.next().map(
            |(character_byte_index, character)| {
                let position = CharacterPosition {
                    byte: ByteCount::from(character_byte_index),
                    utf_8: self.utf_8_index,
                };
                self.utf_8_index += character.utf_8_size();
                (position, character)
            },
        )
    }
}
