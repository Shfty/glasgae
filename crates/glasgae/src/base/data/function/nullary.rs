use crate::prelude::Boxed;

use super::{Term, TermBase};

/// Nullary function which produces an output from no input.
pub trait NullaryT<A: Term>: TermBase + FnOnce() -> A {
    fn clone_io(&self) -> Box<dyn NullaryT<A>>;
}

impl<F, A> NullaryT<A> for F
where
    F: Term + FnOnce() -> A,
    A: Term,
{
    fn clone_io(&self) -> Box<dyn NullaryT<A>> {
        self.clone().boxed()
    }
}

impl<A> Clone for Box<dyn NullaryT<A>>
where
    A: Term,
{
    fn clone(&self) -> Self {
        (**self).clone_io()
    }
}

pub type Nullary<A> = Box<dyn NullaryT<A>>;
