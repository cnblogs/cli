use futures::future::{join_all, JoinAll};
use std::future::Future;

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

pub trait ExactSizeIteratorExt<T>: ExactSizeIterator<Item = T> {
    #[inline]
    fn dyn_rev<'t>(self, rev: bool) -> Box<dyn ExactSizeIterator<Item = T> + 't>
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

impl<T, I> ExactSizeIteratorExt<T> for I where I: ExactSizeIterator<Item = T> {}

pub trait IntoIteratorExt: IntoIterator {
    #[inline]
    fn join_all(self) -> JoinAll<Self::Item>
    where
        Self::Item: Future,
        Self: Sized,
    {
        join_all(self)
    }
}

impl<I> IntoIteratorExt for I where I: IntoIterator {}
