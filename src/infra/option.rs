pub trait WrapOption
where
    Self: Sized,
{
    #[inline]
    fn wrap_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> WrapOption for T {}

pub trait OptionExt<T> {
    fn or_eval_result<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn or_eval_result<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.map_or_else(f, |val| Ok(val))
    }
}
