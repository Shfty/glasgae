use std::panic::UnwindSafe;

use crate::prelude::{Boxed, Function};

/// Unfold a binary function into a series of unary functions
pub trait CurriedClone<A, B, C>: FnOnce(A, B) -> C + Clone + UnwindSafe + 'static {
    fn curried_clone(self) -> impl FnOnce(A) -> Function<B, C> + Clone + 'static
    where
        A: 'static + Clone + UnwindSafe,
    {
        |a| (|b| self(a, b)).boxed()
    }
}

impl<T, A, B, C> CurriedClone<A, B, C> for T where
    T: FnOnce(A, B) -> C + UnwindSafe + Clone + 'static
{
}
