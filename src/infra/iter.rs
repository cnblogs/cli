pub trait IteratorExt<T>: Iterator<Item = T> {
    #[inline]
    fn dyn_rev<'t>(self, rev: bool) -> Box<dyn Iterator<Item = T> + 't>
    where
        Self: DoubleEndedIterator<Item = T> + Sized + 't,
    {
        if rev {
            Box::new(self.rev())
        } else {
            Box::new(self)
        }
    }
}

impl<T, I> IteratorExt<T> for I where I: Iterator<Item = T> {}
