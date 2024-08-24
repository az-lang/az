use super::try_from_ref::TryFromRef;

pub(crate) trait TryRefTo<'a, T> {
    type Error;

    fn try_ref_to(&'a self) -> Result<T, Self::Error>;
}

impl<'a, T, U: TryFromRef<'a, T>> TryRefTo<'a, U> for T {
    type Error = U::Error;

    fn try_ref_to(&'a self) -> Result<U, Self::Error> {
        U::try_from_ref(self)
    }
}
