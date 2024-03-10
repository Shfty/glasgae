use crate::{
    base::data::term::Term,
    prelude::{Boxed, Function},
};

use super::{bifunction::BifunT, FunctionT};

/// Unfold a binary function into a series of unary functions
pub trait Curried<A, B, C>: Term + BifunT<A, B, C>
where
    A: Term,
    B: Term,
    C: Term,
{
    fn curried(self) -> impl Term + FunctionT<A, Function<B, C>> {
        |a| (|b| self(a, b)).boxed()
    }
}

impl<T, A, B, C> Curried<A, B, C> for T
where
    T: Term + BifunT<A, B, C>,
    A: Term,
    B: Term,
    C: Term,
{
}
