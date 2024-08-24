use crate::tokenization::Utf8Count;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct CharacterOffset(Utf8Count);

impl From<usize> for CharacterOffset {
    fn from(value: usize) -> Self {
        Self(Utf8Count::from(value))
    }
}

impl From<CharacterOffset> for usize {
    fn from(value: CharacterOffset) -> Self {
        value.0.into()
    }
}
