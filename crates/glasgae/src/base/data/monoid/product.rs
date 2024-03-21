use crate::{base::grl::num::One, prelude::*, derive_kinded_unary, derive_with_kinded_unary};

/// Monoid under multiplication.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

impl<T> Product<T> {
    pub fn get(self) -> T {
        self.0
    }
}

derive_kinded_unary!(Product<T>);
derive_with_kinded_unary!(Product<T>);

impl<T> Pointed for Product<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Product<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = Product<U>;
}

impl<T, U> Functor<U> for Product<T>
where
    T: Term,
    U: Term,
{
    fn fmap(
        self,
        f: impl FunctionT<Self::Pointed, <Product<U> as Pointed>::Pointed>,
    ) -> Product<U> {
        Product(f(self.get()))
    }
}

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
