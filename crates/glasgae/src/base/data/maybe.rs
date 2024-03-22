//! The [`Maybe`] type encapsulates an optional value.
//!
//! A value of type [`Maybe<A>`] either contains a value of type a (represented as [`Some(A)`]),
//! or it is empty (represented as [`None`]).
//!
//! In practical terms, this is equivalent to Rust's native [`Option`] type
//! (implementations for which are provided in the [`option`] module.)
//!
//! Using `Maybe` is a good way to deal with errors or exceptional cases
//! without resorting to drastic measures such as error.
//!
//! The `Maybe` type is also a monad.
//! It is a simple kind of error monad, where all errors are represented by `Nothing`.
//! A richer error monad can be built using the `Either` type.

pub mod option;

use crate::{prelude::*, derive_pointed, derive_with_pointed};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

use Maybe::*;

impl<T> From<Option<T>> for Maybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Just(t),
            None => Nothing,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        match value {
            Just(t) => Some(t),
            Nothing => None,
        }
    }
}

derive_pointed!(Maybe<(T)>);
derive_with_pointed!(Maybe<(T)>);

impl<T, U> Functor<U> for Maybe<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, U>) -> Maybe<U> {
        match self {
            Just(t) => Just(f(t)),
            Nothing => Nothing,
        }
    }
}

impl<T> PureA for Maybe<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Just(t)
    }
}

impl<F, A, B> AppA<Maybe<A>, Maybe<B>> for Maybe<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Maybe<A>) -> Maybe<B> {
        match self {
            Just(f) => a.fmap(f),
            Nothing => Nothing,
        }
    }
}

impl<T> ReturnM for Maybe<T> where T: Term {}

impl<T, U> ChainM<U> for Maybe<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<T, Maybe<U>> + 'static) -> Maybe<U> {
        match self {
            Just(t) => f(t),
            Nothing => Nothing,
        }
    }
}

impl<T> Semigroup for Maybe<T>
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        match (self, a) {
            (Nothing, Nothing) => Nothing,
            (Nothing, Just(r)) => Just(r),
            (Just(l), Nothing) => Just(l),
            (Just(l), Just(r)) => Just(l.assoc_s(r)),
        }
    }
}

impl<T> Monoid for Maybe<T>
where
    T: Semigroup,
{
    fn mempty() -> Self {
        Nothing
    }
}

impl<T, U> Foldable<U> for Maybe<T>
where
    T: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, z: U) -> U {
        match self {
            Just(x) => f(x, z),
            Nothing => z,
        }
    }

    fn foldl(self, f: impl BifunT<U, T, U>, z: U) -> U {
        match self {
            Just(y) => f(z, y),
            Nothing => z,
        }
    }
}

impl<T> Foldable1<T> for Maybe<T>
where
    T: Term,
{
    fn foldr1(self, f: impl BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}

impl<T, A, U, B> TraverseT<A, (), B> for Maybe<T>
where
    A: Functor<Maybe<U>, Pointed = U, WithPointed = B>,
    A::WithPointed: PureA<Pointed = Maybe<U>>,
    T: Term,
    U: Term,
{
    fn traverse_t(self, f: impl FunctionT<T, A>) -> A::WithPointed {
        match self {
            Just(x) => f(x).fmap(Just.boxed()),
            Nothing => PureA::pure_a(Nothing),
        }
    }
}

impl<A1, A_, A2> SequenceA<(), A2> for Maybe<A1>
where
    A1: Functor<Maybe<A_>, Pointed = A_, WithPointed = A2>,
    A_: Term,
    A2: PureA<Pointed = Maybe<A_>>,
{
    fn sequence_a(self) -> A2 {
        match self {
            Just(x) => x.fmap(Just.boxed()),
            Nothing => PureA::pure_a(Nothing),
        }
    }
}

pub fn maybe<A, B>(default: B, f: impl FunctionT<A, B>, t: Maybe<A>) -> B
where
    A: Term,
    B: Term,
{
    match t {
        Just(t) => f(t),
        Nothing => default,
    }
}
