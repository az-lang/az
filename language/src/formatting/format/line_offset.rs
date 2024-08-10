#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct LineOffset(usize);

impl From<usize> for LineOffset {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<LineOffset> for usize {
    fn from(value: LineOffset) -> Self {
        value.0
    }
}
