use crate::{
    base::{data::term::Term, grl::num::Zero},
    prelude::*,
};

/// Monoid under addition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sum<T>(pub T);

impl<T> Sum<T> {
    pub fn get(self) -> T {
        self.0
    }
}

impl<T> Pointed for Sum<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Sum<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = Sum<U>;
}

impl<T, U> Functor<U> for Sum<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, <Sum<U> as Pointed>::Pointed>) -> Sum<U> {
        Sum(f(self.get()))
    }
}

impl<T> PureA for Sum<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Sum(t)
    }
}

impl<F, A, B> AppA<Sum<A>, Sum<B>> for Sum<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Sum<A>) -> Sum<B> {
        Sum(self.get()(a.get()))
    }
}

impl<T> ReturnM for Sum<T> where T: Term {}

impl<T, U> ChainM<Sum<U>> for Sum<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Sum<U>>) -> Sum<U> {
        f(self.get())
    }
}

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
