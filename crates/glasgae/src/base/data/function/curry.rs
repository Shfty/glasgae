/// Apply a single argument to a function, creating a closure
pub trait Curry<A, B, C>: Sized {
    fn curry(self, a: A) -> impl Fn(B) -> C
    where
        Self: Fn(A, B) -> C,
        A: Clone,
    {
        move |b| self(a.clone(), b)
    }

    fn curry_mut(mut self, a: A) -> impl FnMut(B) -> C
    where
        Self: FnMut(A, B) -> C,
        A: Clone,
    {
        move |b| self(a.clone(), b)
    }

    fn curry_once(self, a: A) -> impl FnOnce(B) -> C
    where
        Self: FnOnce(A, B) -> C,
    {
        move |b| self(a, b)
    }

    fn curry_clone(self, a: A) -> impl FnOnce(B) -> C + Clone
    where
        Self: FnOnce(A, B) -> C + Clone,
        A: Clone,
    {
        move |b| self(a, b)
    }
}

impl<A, B, C, F> Curry<A, B, C> for F {}

