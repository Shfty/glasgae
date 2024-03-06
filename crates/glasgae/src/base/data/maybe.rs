//! The [`Maybe`] type encapsulates an optional value.
//!
//! A value of type [`Maybe<A>`] either contains a value of type a (represented as [`Some(A)`]),
//! or it is empty (represented as [`None`]).
//!
//! Using `Maybe` is a good way to deal with errors or exceptional cases
//! without resorting to drastic measures such as error.
//!
//! The `Maybe` type is also a monad.
//! It is a simple kind of error monad, where all errors are represented by `Nothing`.
//! A richer error monad can be built using the `Either` type.

use crate::prelude::*;

use super::function::bifunction::BifunT;

pub type Maybe<T> = Option<T>;

impl<T> Pointed for Maybe<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Maybe<T> {
    type WithPointed = Maybe<U>;
}

impl<T, U> Functor<U> for Maybe<T>
where
    U: Clone,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, U> + Clone) -> Maybe<U> {
        self.map(f)
    }
}

impl<T> PureA for Maybe<T> {
    fn pure_a(t: Self::Pointed) -> Self {
        Some(t)
    }
}

impl<F, A, B> AppA<Maybe<A>, Maybe<B>> for Maybe<F>
where
    F: FnOnce(A) -> B,
{
    fn app_a(self, a: Maybe<A>) -> Maybe<B> {
        self.and_then(|f| a.map(f))
    }
}

impl<T> ReturnM for Maybe<T> {}

impl<T, U> ChainM<Maybe<U>> for Maybe<T> {
    fn chain_m(self, f: impl FunctionT<T, Maybe<U>> + 'static) -> Maybe<U> {
        self.and_then(f)
    }
}

impl<T> Semigroup for Maybe<T>
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

impl<T> Monoid for Maybe<T>
where
    T: 'static + Semigroup,
{
    fn mempty() -> Self {
        None
    }
}

impl<T, U> Foldr<T, U> for Maybe<T> {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, z: U) -> U {
        match self {
            Some(x) => f(x, z),
            None => z,
        }
    }
}

impl<T, A, U, B> TraverseT<A, U, B> for Maybe<T>
where
    A: Functor<Option<U>, Pointed = U, WithPointed = B>,
    A::Pointed: 'static,
    A::WithPointed: PureA<Pointed = Option<U>>,
    U: Clone,
{
    fn traverse_t(self, f: impl FunctionT<T, A> + Clone) -> A::WithPointed {
        match self {
            Some(x) => f(x).fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for Maybe<A1>
where
    A1: Clone + Functor<Option<A_>, Pointed = A_, WithPointed = A2>,
    A_: 'static + Clone,
    A2: PureA<Pointed = Option<A_>>,
{
    fn sequence_a(self) -> A2 {
        match self {
            Some(x) => x.fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}

pub fn maybe<A, B>(default: B, f: impl FunctionT<A, B>, t: Option<A>) -> B {
    match t {
        Some(t) => f(t),
        None => default,
    }
}
