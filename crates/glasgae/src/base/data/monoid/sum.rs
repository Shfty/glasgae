use crate::{
    base::grl::num::Zero, derive_applicative, derive_functor, derive_monad, derive_pointed,
    derive_with_pointed, prelude::*,
};

/// Monoid under addition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sum<T>(pub T);

impl<T> Sum<T> {
    pub fn get(self) -> T {
        self.0
    }
}

derive_pointed!(Sum<(T)>);
derive_with_pointed!(Sum<(T)>);
derive_functor!(Sum<(T)>);
derive_applicative!(Sum<(T)>);
derive_monad!(Sum<(T)>);

impl<T> Semigroup for Sum<T>
where
    T: Term + std::ops::Add<Output = T>,
{
    fn assoc_s(self, a: Self) -> Self {
        Sum(self.get() + a.get())
    }
}

impl<T> Monoid for Sum<T>
where
    T: Term + Zero + std::ops::Add<Output = T>,
{
    fn mempty() -> Self {
        Sum(Zero::zero())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        // FIXME: This should use foldl'
        list.foldr(Semigroup::assoc_s, Monoid::mempty())
    }
}
