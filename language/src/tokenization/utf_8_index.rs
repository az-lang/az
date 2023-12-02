use std::ops::{Add, AddAssign};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Utf8Index(usize);

impl Add for Utf8Index {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl AddAssign for Utf8Index {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<usize> for Utf8Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Utf8Index> for usize {
    fn from(value: Utf8Index) -> Self {
        value.0
    }
}
