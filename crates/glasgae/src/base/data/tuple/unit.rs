use crate::prelude::*;

impl Pointed for () {
    type Pointed = ();
}

impl Semigroup for () {
    fn assoc_s(self, a: Self) -> Self {
        a
    }
}

impl Monoid for () {
    fn mempty() -> Self {}
    fn mconcat(_: Vec<Self>) -> Self {}
}
