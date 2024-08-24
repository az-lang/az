use super::character_offset::CharacterOffset;
use super::line_offset::LineOffset;

#[derive(Clone, Debug, Default)]
pub struct Offset {
    pub(crate) line: LineOffset,
    pub(crate) character: CharacterOffset,
}
