use std::ops::{Add, AddAssign, Sub};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Utf8Count(usize);

impl Add for Utf8Count {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0).into()
    }
}

impl AddAssign for Utf8Count {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl From<usize> for Utf8Count {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Utf8Count> for usize {
    fn from(value: Utf8Count) -> Self {
        value.0
    }
}

impl Sub for Utf8Count {
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        (self.0 - subtrahend.0).into()
    }
}
