use crate::{prelude::*, base::grl::num::One};

/// Monoid under multiplication.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

impl<T> Product<T> {
    pub fn get(self) -> T {
        self.0
    }
}

impl<T> Pointed for Product<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Product<T> {
    type WithPointed = Product<U>;
}

impl<T, U> Functor<U> for Product<T>
where
    U: Clone,
{
    fn fmap(
        self,
        f: impl FunctionT<Self::Pointed, <Product<U> as Pointed>::Pointed> + Clone,
    ) -> Product<U> {
        Product(f(self.get()))
    }
}

impl<T> PureA for Product<T> {
    fn pure_a(t: Self::Pointed) -> Self {
        Product(t)
    }
}

impl<F, A, B> AppA<Product<A>, Product<B>> for Product<F>
where
    F: FunctionT<A, B>,
{
    fn app_a(self, a: Product<A>) -> Product<B> {
        Product(self.get()(a.get()))
    }
}

impl<T> ReturnM for Product<T> {}

impl<T, U> ChainM<Product<U>> for Product<T> {
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Product<U>> + Clone) -> Product<U> {
        f(self.get())
    }
}

impl<T> Semigroup for Product<T>
where
    T: std::ops::Mul<Output = T>,
{
    fn assoc_s(self, a: Self) -> Self {
        Product(self.get() * a.get())
    }
}

impl<T> Monoid for Product<T>
where
    T: One + 'static + std::ops::Mul<Output = T>,
{
    fn mempty() -> Self {
        Product(One::one())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.into_iter().fold(Monoid::mempty(), Semigroup::assoc_s)
    }
}
