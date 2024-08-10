use super::character_position::CharacterPosition;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubstringPosition {
    pub start: CharacterPosition,
    pub end: CharacterPosition,
}
