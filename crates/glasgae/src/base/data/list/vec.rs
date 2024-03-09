use std::panic::UnwindSafe;

use crate::{base::data::function::bifunction::BifunT, prelude::*};

impl<T> Pointed for Vec<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Vec<T> {
    type WithPointed = Vec<U>;
}

impl<T, U> Functor<U> for Vec<T>
where
    U: Clone + UnwindSafe,
{
    fn fmap(
        self,
        f: impl FunctionT<Self::Pointed, <Vec<U> as Pointed>::Pointed> + Clone,
    ) -> Vec<U> {
        self.into_iter().map(|t| f.clone()(t)).collect()
    }
}

impl<T> PureA for Vec<T> {
    fn pure_a(t: Self::Pointed) -> Self {
        vec![t]
    }
}

impl<F, A, B> AppA<Vec<A>, Vec<B>> for Vec<F>
where
    F: FnOnce(A) -> B,
{
    fn app_a(self, a: Vec<A>) -> Vec<B> {
        self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
    }
}

impl<T> ReturnM for Vec<T> {}

impl<T, U> ChainM<Vec<U>> for Vec<T> {
    fn chain_m(self, f: impl FunctionT<T, Vec<U>> + Clone) -> Vec<U> {
        self.into_iter().flat_map(|t| f.clone_fun()(t)).collect()
    }
}

impl<T, U> Foldr<T, U> for Vec<T> {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, init: U) -> U {
        self.into_iter()
            .rfold(init, |acc, next| f.clone()(next, acc))
    }
}

impl<T, A_, A1, A2> TraverseT<A1, A_, A2> for Vec<T>
where
    Self: Functor<A1>,
    <Self as WithPointed<A1>>::WithPointed: SequenceA<A_, A2>,
    A1: Clone + UnwindSafe,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for Vec<A1>
where
    A1: Functor<Function<Vec<A_>, Vec<A_>>, Pointed = A_>,
    A_: 'static + Clone + UnwindSafe,
    A2: PureA<Pointed = Vec<A_>>,
    A1::WithPointed: AppA<A2, A2>,
{
    fn sequence_a(self) -> A2 {
        self.foldr(
            |next, acc| next.fmap(|t| (|v| push(t, v)).boxed()).app_a(acc),
            PureA::pure_a(vec![]),
        )
    }
}

impl<T> Semigroup for Vec<T> {
    fn assoc_s(self, a: Self) -> Self {
        self.into_iter().chain(a).collect()
    }
}

impl<T> Monoid for Vec<T>
where
    T: 'static,
{
    fn mempty() -> Self {
        vec![]
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.into_iter().flatten().collect()
    }
}

pub fn push<T>(t: T, mut v: Vec<T>) -> Vec<T> {
    v.insert(0, t);
    v
}
