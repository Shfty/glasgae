use std::convert::identity;

use crate::prelude::{Boxed, Compose, Function, FunctionT, Semigroup};

use super::Monoid;

/// The monoid of endomorphisms under composition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Endo<F>(F);

impl<A> Endo<Function<A, A>> {
    pub fn new(f: impl FunctionT<A, A> + Clone) -> Self {
        Endo(f.boxed())
    }

    pub fn app(self) -> Function<A, A> {
        self.0
    }
}

impl<A> Semigroup for Endo<Function<A, A>>
where
    A: 'static,
{
    fn assoc_s(self, a: Self) -> Self {
        Endo(self.app().compose_clone(a.app()).boxed())
    }
}

impl<A> Monoid for Endo<Function<A, A>>
where
    A: 'static,
{
    fn mempty() -> Self {
        Endo::new(identity)
    }
}
