pub trait IntoOption
    where
        Self: Sized,
{
    #[inline]
    fn into_some<E>(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> IntoOption for T {}

pub trait OptionExt<T> {
    fn bind_result<E, F>(self, f: F) -> Result<T, E>
        where
            F: FnOnce() -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn bind_result<E, F>(self, f: F) -> Result<T, E>
        where
            F: FnOnce() -> Result<T, E>,
    {
        match self {
            Some(val) => Ok(val),
            _ => f()
        }
    }
}
