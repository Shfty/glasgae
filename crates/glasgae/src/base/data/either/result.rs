use std::panic::UnwindSafe;

use crate::{
    base::data::{fold_map_default, function::bifunction::BifunT, FoldMap},
    prelude::{
        AppA, Boxed, ChainM, Foldr, FunctionT, Functor, Monoid, Pointed, PureA, ReturnM, Semigroup,
        SequenceA, TraverseT, WithPointed,
    },
};

use super::{Either, Either::*};

impl<T, E> From<Either<E, T>> for Result<T, E> {
    fn from(value: Either<E, T>) -> Self {
        match value {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }
}

impl<T, E> Pointed for Result<T, E> {
    type Pointed = T;
}

impl<T, E, U> WithPointed<U> for Result<T, E> {
    type WithPointed = Result<U, E>;
}

impl<T, E, U> Functor<U> for Result<T, E>
where
    U: Clone + UnwindSafe,
{
    fn fmap(
        self,
        f: impl crate::prelude::FunctionT<Self::Pointed, U> + Clone,
    ) -> Self::WithPointed {
        match self {
            Ok(t) => Ok(f(t)),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> PureA for Result<T, E> {
    fn pure_a(t: Self::Pointed) -> Self {
        Ok(t)
    }
}

impl<F, E, A, B> AppA<Result<A, E>, Result<B, E>> for Result<F, E>
where
    F: FunctionT<A, B>,
{
    fn app_a(self, a: Result<A, E>) -> Result<B, E> {
        self.and_then(|f| a.map(f))
    }
}

impl<T, E> ReturnM for Result<T, E> {}

impl<T, E, U> ChainM<Result<U, E>> for Result<T, E> {
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Result<U, E>> + Clone) -> Result<U, E> {
        self.and_then(f)
    }
}

impl<E, A, B> FoldMap<A, B> for Result<A, E>
where
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B> + Clone) -> B {
        fold_map_default(self, f)
    }
}

impl<E, A, B> Foldr<A, B> for Result<A, E> {
    fn foldr(self, f: impl BifunT<A, B, B> + Clone, z: B) -> B {
        match self {
            Ok(y) => f(y, z),
            Err(_) => z,
        }
    }
}

impl<E, A, A_, A1> TraverseT<A1, A_, A1::WithPointed> for Result<A, E>
where
    A1: Functor<Result<A_, E>, Pointed = A_>,
    A1::WithPointed: PureA<Pointed = Result<A_, E>>,
    E: 'static + Clone + UnwindSafe,
    A_: 'static + Clone + UnwindSafe,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A1::WithPointed {
        match self {
            Ok(y) => f(y).fmap(Ok.boxed()),
            Err(x) => PureA::pure_a(Err(x)),
        }
    }
}

impl<E, A1, A_> SequenceA<A_, A1::WithPointed> for Result<A1, E>
where
    A1: Functor<Result<A_, E>, Pointed = A_>,
    A1::WithPointed: PureA<Pointed = Result<A_, E>>,
    E: 'static + Clone + UnwindSafe,
    A_: 'static + Clone + UnwindSafe,
{
    fn sequence_a(self) -> A1::WithPointed {
        match self {
            Ok(y) => y.fmap(Ok.boxed()),
            Err(x) => PureA::pure_a(Err(x)),
        }
    }
}

impl<E, A> Semigroup for Result<E, A> {
    fn assoc_s(self, b: Self) -> Self {
        match self {
            Ok(_) => self,
            Err(_) => b,
        }
    }
}
