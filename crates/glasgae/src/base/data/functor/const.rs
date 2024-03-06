use crate::{base::data::function::bifunction::BifunT, prelude::*};

use super::Functor;

/// The Const functor.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const<T>(T);

impl<T> Const<T> {
    pub fn get(self) -> T {
        self.0
    }
}

impl<T> Pointed for Const<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Const<T> {
    type WithPointed = Const<U>;
}

impl<T> Functor<T> for Const<T>
where
    T: Clone,
{
    fn fmap(self, _: impl FunctionT<Self::Pointed, T> + Clone) -> Self::WithPointed {
        self
    }
}

impl<T, U> Foldr<T, U> for Const<T> {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, z: U) -> U {
        f(self.0, z)
    }
}

impl<T> PureA for Const<T> {
    fn pure_a(t: T) -> Const<T> {
        Const(t)
    }
}

impl<F, A, B> AppA<Const<A>, Const<B>> for Const<F>
where
    F: FunctionT<A, B>,
{
    fn app_a(self, a: Const<A>) -> Const<B> {
        Const(self.0(a.0))
    }
}

impl<T, A_, A1, A2> TraverseT<A1, A_, A2> for Const<T>
where
    Self: Functor<A1>,
    WithPointedT<Self, A1>: SequenceA<A_, A2>,
    A1: Clone,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A_, A3> SequenceA<A_, A3> for Const<A1>
where
    A1: PureA<Pointed = A_> + Functor<Function<Const<A_>, Const<A_>>>,
    A1::Pointed: 'static + Clone + Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A3: PureA<Pointed = Const<A1::Pointed>>,
{
    fn sequence_a(self) -> A3 {
        self.foldr(
            |next, acc| next.fmap(|t| (|_| Const(t)).boxed()).app_a(acc),
            PureA::pure_a(Const(Monoid::mempty())),
        )
    }
}

impl<T> Semigroup for Const<T>
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        Const(self.0.assoc_s(a.0))
    }
}

impl<T> Monoid for Const<T>
where
    T: 'static + Monoid,
{
    fn mempty() -> Self {
        Const(T::mempty())
    }
}
