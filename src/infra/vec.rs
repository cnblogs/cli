pub trait VecExt<T> {
    fn chain_push(self, item: T) -> Vec<T>;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline]
    fn chain_push(mut self, item: T) -> Self {
        self.push(item);
        self
    }
}
