pub trait IteratorExt<T> {
    fn dyn_rev<'t>(self, rev: bool) -> Box<dyn Iterator<Item = T> + 't>
    where
        Self: DoubleEndedIterator<Item = T> + 't;
}

impl<I, T> IteratorExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn dyn_rev<'t>(self, rev: bool) -> Box<dyn Iterator<Item = T> + 't>
    where
        Self: DoubleEndedIterator<Item = T> + 't,
    {
        if rev {
            Box::new(self.rev())
        } else {
            Box::new(self)
        }
    }
}
