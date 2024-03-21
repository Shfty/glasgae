use crate::{prelude::*, derive_kinded_unary, derive_with_kinded_unary};

derive_kinded_unary!(Option<T>);
derive_with_kinded_unary!(Option<T>);

impl<T> Pointed for Option<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Option<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = Option<U>;
}

impl<T, U> Fmap<U> for Option<T>
where
    T: Term,
    U: Term,
{
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

impl<T, U> ChainM<Option<U>> for Option<T>
where
    T: Term,
    U: Term,
{
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
    A: Fmap<Option<U>, Pointed = U, WithPointed = B>,
    A::WithPointed: PureA<Pointed = Option<U>>,
    T: Term,
    U: Term,
{
    fn traverse_t(self, f: impl FunctionT<T, A>) -> A::WithPointed {
        match self {
            Some(x) => f(x).fmap(Some.boxed()),
            None => PureA::pure_a(None),
        }
    }
}

impl<A1, A_, A2> SequenceA<(), A2> for Option<A1>
where
    A1: Clone + Fmap<Option<A_>, Pointed = A_, WithPointed = A2>,
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
