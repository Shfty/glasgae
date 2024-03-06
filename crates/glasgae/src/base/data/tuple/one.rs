use crate::{base::data::function::bifunction::BifunT, prelude::*};

impl<A> PureA for (A,) {
    fn pure_a(t: Self::Pointed) -> Self {
        (t,)
    }
}

impl<F, A, B> AppA<(A,), (B,)> for (F,)
where
    F: FnOnce(A) -> B,
{
    fn app_a(self, a: (A,)) -> (B,) {
        (self.0(a.0),)
    }
}

impl<T> ReturnM for (T,) {}

impl<T, U> ChainM<(U,)> for (T,) {
    fn chain_m(self, f: impl FunctionT<Self::Pointed, (U,)> + Clone) -> (U,) {
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

impl<T, U> Foldr<T, U> for (T,) {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, init: U) -> U {
        f(self.0, init)
    }
}

impl<T, A1, A_, A3> TraverseT<A1, A_, A3> for (T,)
where
    A1: Clone + PureA<Pointed = A_> + Functor<Function<(A_,), (A_,)>>,
    A1::Pointed: 'static + Clone + Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A3: PureA<Pointed = (A1::Pointed,)>,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A3 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A3, A_> SequenceA<A_, A3> for (A1,)
where
    A1: PureA<Pointed = A_> + Functor<Function<(A_,), (A_,)>>,
    A1::Pointed: 'static + Clone + Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A3: PureA<Pointed = (A1::Pointed,)>,
{
    fn sequence_a(self) -> A3 {
        self.foldr(
            |next, acc| next.fmap(|t| (|_| (t,)).boxed()).app_a(acc),
            PureA::pure_a((Monoid::mempty(),)),
        )
    }
}
