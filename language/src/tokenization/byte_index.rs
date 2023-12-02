use std::ops::{Add, AddAssign};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteIndex(usize);

impl Add for ByteIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl AddAssign for ByteIndex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<usize> for ByteIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<ByteIndex> for usize {
    fn from(value: ByteIndex) -> Self {
        value.0
    }
}
