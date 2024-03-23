use crate::{derive_pointed, derive_with_pointed, prelude::*};

derive_pointed!(Option<(T)>);
derive_with_pointed!(Option<(T)>);

impl<T, U> Functor<U> for Option<T>
where
    T: Term,
    U: Term,
{
    type Mapped = Option<U>;

    fn fmap(self, f: impl FunctionT<Self::Pointed, U>) -> Option<U> {
        self.map(f)
    }
}

impl<T> PureA for Option<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Some(t)
    }
}

impl<F, A, B> AppA<Option<A>, Option<B>> for Option<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Option<A>) -> Option<B> {
        self.and_then(|f| a.map(f))
    }
}

impl<T> ReturnM for Option<T> where T: Term {}

impl<T, U> ChainM<U> for Option<T>
where
    T: Term,
    U: Term,
{
    type Chained = Option<U>;

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

impl<T, U> Foldable<U> for Option<T>
where
    T: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, z: U) -> U {
        match self {
            Some(x) => f(x, z),
            None => z,
        }
    }

    fn foldl(self, f: impl BifunT<U, T, U>, z: U) -> U {
        match self {
            Some(y) => f(z, y),
            None => z,
        }
    }
}

impl<T> Foldable1<T> for Option<T>
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

impl<T, A, U, B> TraverseT<A, (), B> for Option<T>
where
    A: Functor<Option<U>, Pointed = U, Mapped = B>,
    A::Mapped: PureA<Pointed = Option<U>>,
    T: Term,
    U: Term,
{
    fn traverse_t(self, f: impl FunctionT<T, A>) -> A::Mapped {
        match self {
            Some(x) => f(x).fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}

impl<A1, A_, A2> SequenceA<(), A2> for Option<A1>
where
    A1: Clone + Functor<Option<A_>, Pointed = A_, Mapped = A2>,
    A_: Term,
    A2: PureA<Pointed = Option<A_>>,
{
    fn sequence_a(self) -> A2 {
        match self {
            Some(x) => x.fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}
