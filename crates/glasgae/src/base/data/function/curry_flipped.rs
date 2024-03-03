use crate::prelude::{Curry, Flip};

/// Flip a function's arguments, and then curry it.
pub trait CurryFlipped<A, B, C>: Flip<A, B, C> {
    fn curry_flipped(self, b: B) -> impl FnOnce(A) -> C
    where
        Self: FnOnce(A, B) -> C,
    {
        self.flip_once().curry_once(b)
    }
}

impl<F, A, B, C> CurryFlipped<A, B, C> for F where F: Flip<A, B, C> {}
