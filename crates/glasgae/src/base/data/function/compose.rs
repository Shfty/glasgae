/// Left to right function composition
pub trait Compose<A, B, C>: Sized {
    fn compose(self, f: impl Fn(B) -> C + Clone) -> impl Fn(A) -> C
    where
        Self: Fn(A) -> B + Clone,
    {
        move |a| f.clone()(self.clone()(a))
    }

    fn compose_mut(self, f: impl FnMut(B) -> C + Clone) -> impl FnMut(A) -> C
    where
        Self: FnMut(A) -> B + Clone,
    {
        move |a| f.clone()(self.clone()(a))
    }

    fn compose_once(self, f: impl FnOnce(B) -> C) -> impl FnOnce(A) -> C
    where
        Self: FnOnce(A) -> B,
    {
        move |a| f(self(a))
    }

    fn compose_clone(self, f: impl FnOnce(B) -> C + Clone) -> impl FnOnce(A) -> C + Clone
    where
        Self: FnOnce(A) -> B + Clone,
    {
        move |a| f(self(a))
    }
}

impl<F, A, B, C> Compose<A, B, C> for F where F: FnOnce(A) -> B {}
