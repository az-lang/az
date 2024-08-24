pub(crate) trait RefTo<'a, T> {
    fn ref_to(&'a self) -> T;
}
