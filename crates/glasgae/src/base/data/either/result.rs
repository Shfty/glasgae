use crate::{derive_pointed, derive_with_pointed, prelude::*};

use super::{Either, Either::*};

impl<T, E> From<Either<E, T>> for Result<T, E> {
    fn from(value: Either<E, T>) -> Self {
        match value {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }
}

derive_pointed!(Result<(T), E>);
derive_with_pointed!(Result<(T), E>);

impl<T, E, U> Functor<U> for Result<T, E>
where
    T: Term,
    E: Term,
    U: Term,
{
    type Mapped = Result<U, E>;

    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::Mapped {
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

impl<F, E, A, B> AppA<A, B> for Result<F, E>
where
    F: Term + FunctionT<A, B>,
    E: Term,
    A: Term,
    B: Term,
{
    type WithA = Result<A, E>;
    type WithB = Result<B, E>;

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

impl<T, E, U> ChainM<U> for Result<T, E>
where
    T: Term,
    E: Term,
    U: Term,
{
    type Chained = Result<U, E>;

    fn chain_m(self, f: impl FunctionT<Self::Pointed, Result<U, E>>) -> Result<U, E> {
        self.and_then(f)
    }
}

impl<E, A, B> FoldMap<B> for Result<A, E>
where
    E: Term,
    A: Term,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B>) -> B {
        fold_map_default(self, f.to_function())
    }
}

impl<E, A, B> Foldable<B> for Result<A, E>
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

impl<E, T, A, MA, MB> TraverseT<MA, (), MB> for Result<T, E>
where
    T: Term,
    MA: Functor<Result<A, E>, Pointed = A, Mapped = MB>,
    MB: PureA<Pointed = Result<A, E>>,
    E: Term,
    A: Term,
{
    type Inner = MA;
    type Value = A;
    type Traversed = MB;

    fn traverse_t(self, f: impl FunctionT<Self::Pointed, MA>) -> MB {
        match self {
            Ok(y) => f(y).fmap(Ok.boxed()),
            Err(x) => PureA::pure_a(Err(x)),
        }
    }
}

impl<E, A1, A, A2> SequenceA<(), A2> for Result<A1, E>
where
    A1: Functor<Result<A, E>, Pointed = A, Mapped = A2>
        + WithPointed<Function<Result<A1, E>, Result<A, E>>>,
    A2: PureA<Pointed = Result<A, E>>,
    E: Term,
    A: Term,
{
    type Inner = A1;
    type Value = PointedT<A1>;
    type Sequenced = A2;

    fn sequence_a(self) -> A1::Mapped {
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
