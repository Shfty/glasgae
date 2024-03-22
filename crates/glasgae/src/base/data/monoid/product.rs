use crate::{
    base::grl::num::One, derive_applicative, derive_functor, derive_monad, derive_pointed,
    derive_with_pointed, prelude::*,
};

/// Monoid under multiplication.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

impl<T> Product<T> {
    pub fn get(self) -> T {
        self.0
    }
}

derive_pointed!(Product<(T)>);
derive_with_pointed!(Product<(T)>);
derive_functor!(Product<(T)>);
derive_applicative!(Product<(T)>);
derive_monad!(Product<(T)>);

impl<T> Semigroup for Product<T>
where
    T: Term + std::ops::Mul<Output = T>,
{
    fn assoc_s(self, a: Self) -> Self {
        Product(self.get() * a.get())
    }
}

impl<T> Monoid for Product<T>
where
    T: Term + One + std::ops::Mul<Output = T>,
{
    fn mempty() -> Self {
        Product(One::one())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.into_iter().fold(Monoid::mempty(), Semigroup::assoc_s)
    }
}
