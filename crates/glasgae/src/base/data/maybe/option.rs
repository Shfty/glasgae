use std::panic::UnwindSafe;

use crate::{
    base::data::function::bifunction::BifunT,
    prelude::{
        AppA, Boxed, ChainM, Foldr, FunctionT, Functor, Monoid, Pointed, PureA, ReturnM, Semigroup,
        SequenceA, TraverseT, WithPointed,
    },
};

impl<T> Pointed for Option<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Option<T> {
    type WithPointed = Option<U>;
}

impl<T, U> Functor<U> for Option<T>
where
    U: Clone + UnwindSafe,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, U> + Clone) -> Option<U> {
        self.map(f)
    }
}

impl<T> PureA for Option<T> {
    fn pure_a(t: Self::Pointed) -> Self {
        Some(t)
    }
}

impl<F, A, B> AppA<Option<A>, Option<B>> for Option<F>
where
    F: FnOnce(A) -> B,
{
    fn app_a(self, a: Option<A>) -> Option<B> {
        self.and_then(|f| a.map(f))
    }
}

impl<T> ReturnM for Option<T> {}

impl<T, U> ChainM<Option<U>> for Option<T> {
    fn chain_m(self, f: impl FunctionT<T, Option<U>> + 'static) -> Option<U> {
        self.and_then(f)
    }
}

impl<T> Semigroup for Option<T>
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        match (self, a) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some(l.assoc_s(r)),
        }
    }
}

impl<T> Monoid for Option<T>
where
    T: 'static + Semigroup,
{
    fn mempty() -> Self {
        None
    }
}

impl<T, U> Foldr<T, U> for Option<T> {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, z: U) -> U {
        match self {
            Some(x) => f(x, z),
            None => z,
        }
    }
}

impl<T, A, U, B> TraverseT<A, U, B> for Option<T>
where
    A: Functor<Option<U>, Pointed = U, WithPointed = B>,
    A::Pointed: 'static,
    A::WithPointed: PureA<Pointed = Option<U>>,
    U: Clone + UnwindSafe,
{
    fn traverse_t(self, f: impl FunctionT<T, A> + Clone) -> A::WithPointed {
        match self {
            Some(x) => f(x).fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for Option<A1>
where
    A1: Clone + Functor<Option<A_>, Pointed = A_, WithPointed = A2>,
    A_: 'static + Clone + UnwindSafe,
    A2: PureA<Pointed = Option<A_>>,
{
    fn sequence_a(self) -> A2 {
        match self {
            Some(x) => x.fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}
