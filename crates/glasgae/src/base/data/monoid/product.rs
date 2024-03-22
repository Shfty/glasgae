use crate::{base::grl::num::One, derive_pointed, derive_with_pointed, prelude::*, derive_functor};

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

impl<T> PureA for Product<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Product(t)
    }
}

impl<F, A, B> AppA<Product<A>, Product<B>> for Product<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Product<A>) -> Product<B> {
        Product(self.get()(a.get()))
    }
}

impl<T> ReturnM for Product<T> where T: Term {}

impl<T, U> ChainM<U> for Product<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Product<U>>) -> Product<U> {
        f(self.get())
    }
}

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
