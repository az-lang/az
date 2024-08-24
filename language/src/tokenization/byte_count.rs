use std::ops::{Add, AddAssign, Sub};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteCount(usize);

impl Add for ByteCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl AddAssign for ByteCount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<usize> for ByteCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<ByteCount> for usize {
    fn from(value: ByteCount) -> Self {
        value.0
    }
}

impl Sub for ByteCount {
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        (self.0 - subtrahend.0).into()
    }
}
