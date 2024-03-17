use crate::{
    base::data::{foldl1_default, foldr1_default, function::bifunction::BifunT, Foldable1},
    prelude::*,
};

impl<A> PureA for (A,)
where
    A: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        (t,)
    }
}

impl<F, A, B> AppA<(A,), (B,)> for (F,)
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: (A,)) -> (B,) {
        (self.0(a.0),)
    }
}

impl<T> ReturnM for (T,) where T: Term {}

impl<T, U> ChainM<(U,)> for (T,)
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, (U,)>) -> (U,) {
        f(self.0)
    }
}

impl<T> Semigroup for (T,)
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        (self.0.assoc_s(a.0),)
    }
}

impl<T> Monoid for (T,)
where
    T: Monoid,
{
    fn mempty() -> Self {
        (T::mempty(),)
    }
}

impl<T, U> Foldable<U> for (T,)
where
    T: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, init: U) -> U {
        f(self.0, init)
    }

    fn foldl(self, f: impl BifunT<U, T, U>, init: U) -> U {
        f(init, self.0)
    }
}

impl<T> Foldable1<T> for (T,)
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

impl<T, A1, A_, A3> TraverseT<A1, A_, A3> for (T,)
where
    A1: PureA<Pointed = A_> + Fmap<Function<(A_,), (A_,)>>,
    A1::Pointed: Monoid,
    A1::WithPointed: AppA<A3, A3>,
    T: Term,
    A_: Term,
    A3: PureA<Pointed = (A1::Pointed,)>,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A3 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A3, A_> SequenceA<A_, A3> for (A1,)
where
    A1: PureA<Pointed = A_> + Fmap<Function<(A_,), (A_,)>>,
    A1::Pointed: Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A_: Term,
    A3: PureA<Pointed = (A1::Pointed,)>,
{
    fn sequence_a(self) -> A3 {
        self.foldr(
            |next, acc| next.fmap(|t| (|_| (t,)).boxed()).app_a(acc),
            PureA::pure_a((Monoid::mempty(),)),
        )
    }
}
