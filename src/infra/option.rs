pub trait IntoOption
where
    Self: Sized,
{
    #[inline]
    fn into_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> IntoOption for T {}

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
        match self {
            Some(val) => Ok(val),
            _ => f(),
        }
    }
}
