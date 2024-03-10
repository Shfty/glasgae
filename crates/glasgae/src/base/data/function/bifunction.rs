use crate::{
    base::data::term::{Term, TermBase},
    prelude::Boxed,
};

/// Binary function
pub trait BifunT<A, B, C>: TermBase + FnOnce(A, B) -> C {
    fn to_bifun(&self) -> Bifun<A, B, C>;
}

impl<F, A, B, C> BifunT<A, B, C> for F
where
    F: Term + FnOnce(A, B) -> C,
    A: Term,
    B: Term,
    C: Term,
{
    fn to_bifun(&self) -> Bifun<A, B, C> {
        self.clone().boxed()
    }
}

pub type Bifun<A, B, C> = Box<dyn BifunT<A, B, C>>;

impl<A, B, C> Clone for Bifun<A, B, C>
where
    A: Term,
    B: Term,
    C: Term,
{
    fn clone(&self) -> Self {
        (**self).to_bifun()
    }
}
