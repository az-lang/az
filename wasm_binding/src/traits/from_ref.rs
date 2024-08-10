use super::ref_to::RefTo;

pub(crate) trait FromRef<'a, T>: Sized {
    fn from_ref(value: &'a T) -> Self;
}

impl<'a, T, U: FromRef<'a, T>> RefTo<'a, U> for T {
    fn ref_to(&'a self) -> U {
        U::from_ref(self)
    }
}
