use crate::prelude::Boxed;

/// Nullary function which produces an output from no input.
pub trait NullaryT<A>: FnOnce() -> A + 'static {
    fn clone_io(&self) -> Box<dyn NullaryT<A>>;
}

impl<F, A> NullaryT<A> for F
where
    F: FnOnce() -> A + Clone + 'static,
{
    fn clone_io(&self) -> Box<dyn NullaryT<A>> {
        self.clone().boxed()
    }
}

impl<A> Clone for Box<dyn NullaryT<A>>
where
    A: 'static,
{
    fn clone(&self) -> Self {
        self.clone_io()
    }
}

pub type Nullary<A> = Box<dyn NullaryT<A>>;
