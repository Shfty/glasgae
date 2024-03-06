//! The identity functor and monad.
//!
//! This trivial type constructor serves two purposes:
//!
//! It can be used with functions parameterized by functor or monad classes.
//! It can be used as a base monad to which a series of monad transformers may be applied to construct a composite monad. Most monad transformer modules include the special case of applying the transformer to Identity. For example, State s is an abbreviation for StateT s Identity.

use crate::{prelude::*, base::data::function::bifunction::BifunT};

use super::Functor;

/// Identity functor and monad.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(pub T);

impl<T> Identity<T> {
    pub fn run(self) -> T {
        self.0
    }
}

impl<T> Pointed for Identity<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Identity<T> {
    type WithPointed = Identity<U>;
}

impl<T, U> Functor<U> for Identity<T>
where
    U: Clone,
{
    fn fmap(self, f: impl FunctionT<T, U> + Clone) -> Identity<U> {
        Identity(f(self.0))
    }
}

impl<T> PureA for Identity<T> {
    fn pure_a(t: Self::Pointed) -> Self {
        Identity(t)
    }
}

impl<F, A, B> AppA<Identity<A>, Identity<B>> for Identity<F>
where
    F: FnOnce(A) -> B + Clone + 'static,
    B: Clone,
{
    fn app_a(self, a: Identity<A>) -> Identity<B> {
        a.fmap(self.0)
    }
}

impl<T> ReturnM for Identity<T> {}

impl<T, U> ChainM<Identity<U>> for Identity<T>
where
    U: 'static + Clone,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Identity<U>>) -> Identity<U> {
        f(self.0)
    }
}

impl<T> Semigroup for Identity<T>
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        Identity(self.run().assoc_s(a.run()))
    }
}

impl<T> Monoid for Identity<T>
where
    T: Monoid,
{
    fn mempty() -> Self {
        Identity(T::mempty())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        Identity(Monoid::mconcat(
            list.into_iter().map(Identity::run).collect::<Vec<_>>(),
        ))
    }
}

impl<T, U> Foldr<T, U> for Identity<T> {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, init: U) -> U {
        f(self.run(), init)
    }
}

impl<T, A_, A1, A3> TraverseT<A1, A_, A3> for Identity<T>
where
    A1: Clone + PureA + Pointed<Pointed = A_> + Functor<Function<Identity<A_>, Identity<A_>>>,
    A1::Pointed: 'static + Clone + Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A3: PureA<Pointed = Identity<A1::Pointed>>,
{
    fn traverse_t(self, f: impl FunctionT<T, A1> + Clone) -> A3 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A_, A3> SequenceA<A_, A3> for Identity<A1>
where
    A1: PureA<Pointed = A_> + Functor<Function<Identity<A_>, Identity<A_>>>,
    A1::Pointed: 'static + Clone + Monoid,
    A1::WithPointed: AppA<A3, A3>,
    A3: PureA<Pointed = Identity<A1::Pointed>>,
{
    fn sequence_a(self) -> A3 {
        self.foldr(
            |next, acc| next.fmap(|t| (|_| Identity(t)).boxed()).app_a(acc),
            PureA::pure_a(Identity(Monoid::mempty())),
        )
    }
}
