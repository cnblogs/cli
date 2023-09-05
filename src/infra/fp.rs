pub mod currying {
    #[inline]
    pub fn id<X>(x: X) -> impl Fn(X) -> X
    where
        X: Clone,
    {
        move |_| x.clone()
    }

    #[inline]
    pub fn eq<T>(a: T) -> impl Fn(T) -> bool
    where
        T: PartialEq,
    {
        move |b| a == b
    }

    #[inline]
    pub fn lt<T>(a: T) -> impl Fn(T) -> bool
    where
        T: PartialOrd,
    {
        move |b| a < b
    }

    #[inline]
    pub fn gt<T>(a: T) -> impl Fn(T) -> bool
    where
        T: PartialOrd,
    {
        move |b| a > b
    }
}
