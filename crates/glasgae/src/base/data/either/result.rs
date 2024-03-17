use crate::{
    base::data::{
        fold_map_default, foldl1_default, foldr1_default, function::bifunction::BifunT, FoldMap,
        Foldable1,
    },
    prelude::*,
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

impl<T, E> Pointed for Result<T, E>
where
    T: Term,
    E: Term,
{
    type Pointed = T;
}

impl<T, U, E> WithPointed<U> for Result<T, E>
where
    T: Term,
    U: Term,
    E: Term,
{
    type WithPointed = Result<U, E>;
}

impl<T, E, U> Functor<U> for Result<T, E>
where
    T: Term,
    E: Term,
    U: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::WithPointed {
        match self {
            Ok(t) => Ok(f(t)),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> PureA for Result<T, E>
where
    T: Term,
    E: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Ok(t)
    }
}

impl<F, E, A, B> AppA<Result<A, E>, Result<B, E>> for Result<F, E>
where
    F: Term + FunctionT<A, B>,
    E: Term,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Result<A, E>) -> Result<B, E> {
        self.and_then(|f| a.map(f))
    }
}

impl<T, E> ReturnM for Result<T, E>
where
    T: Term,
    E: Term,
{
}

impl<T, E, U> ChainM<Result<U, E>> for Result<T, E>
where
    T: Term,
    E: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Result<U, E>>) -> Result<U, E> {
        self.and_then(f)
    }
}

impl<E, A, B> FoldMap<A, B> for Result<A, E>
where
    E: Term,
    A: Term,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B>) -> B {
        fold_map_default(self, f.to_function())
    }
}

impl<E, A, B> Foldable<A, B> for Result<A, E>
where
    A: Term,
    E: Term,
{
    fn foldr(self, f: impl BifunT<A, B, B>, z: B) -> B {
        match self {
            Ok(y) => f(y, z),
            Err(_) => z,
        }
    }

    fn foldl(self, f: impl BifunT<B, A, B>, z: B) -> B {
        match self {
            Ok(y) => f(z, y),
            Err(_) => z,
        }
    }
}

impl<E, A> Foldable1<A> for Result<A, E>
where
    A: Term,
    E: Term,
{
    fn foldr1(self, f: impl BifunT<A, A, A>) -> A {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<A, A, A>) -> A {
        foldl1_default(self, f)
    }
}

impl<E, A, A_, A1> TraverseT<A1, A_, A1::WithPointed> for Result<A, E>
where
    A1: Functor<Result<A_, E>, Pointed = A_>,
    A1::WithPointed: PureA<Pointed = Result<A_, E>>,
    E: Term,
    A: Term,
    A_: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A1::WithPointed {
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
    E: Term,
    A_: Term,
{
    fn sequence_a(self) -> A1::WithPointed {
        match self {
            Ok(y) => y.fmap(Ok.boxed()),
            Err(x) => PureA::pure_a(Err(x)),
        }
    }
}

impl<E, A> Semigroup for Result<E, A>
where
    E: Term,
    A: Term,
{
    fn assoc_s(self, b: Self) -> Self {
        match self {
            Ok(_) => self,
            Err(_) => b,
        }
    }
}
