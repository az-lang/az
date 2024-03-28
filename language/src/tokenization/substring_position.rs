use super::character_position::CharacterPosition;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubstringPosition {
    pub start_line: usize,
    pub start_character: CharacterPosition,
    pub end_line: usize,
    pub end_character: CharacterPosition,
}
