//! The identity functor and monad.
//!
//! This trivial type constructor serves two purposes:
//!
//! It can be used with functions parameterized by functor or monad classes.
//! It can be used as a base monad to which a series of monad transformers may be applied to construct a composite monad. Most monad transformer modules include the special case of applying the transformer to Identity. For example, State s is an abbreviation for StateT s Identity.

use crate::{
    derive_applicative, derive_functor, derive_monad, derive_pointed, derive_with_pointed,
    prelude::*,
};

use super::Functor;

/// Identity functor and monad.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(pub T);

impl<T> Identity<T> {
    pub fn run(self) -> T {
        self.0
    }
}

derive_pointed!(Identity<(T)>);
derive_with_pointed!(Identity<(T)>);
derive_functor!(Identity<(T)>);
derive_applicative!(Identity<(T)>);
derive_monad!(Identity<(T)>);

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

impl<T, U> Foldable<U> for Identity<T>
where
    T: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, init: U) -> U {
        f(self.run(), init)
    }

    fn foldl(self, f: impl BifunT<U, T, U>, init: U) -> U {
        f(init, self.run())
    }
}

impl<T> Foldable1<T> for Identity<T>
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

impl<T, A1, A_, A2> TraverseT<A1, (), A2> for Identity<T>
where
    A1: PureA<Pointed = A_> + Functor<Function<Identity<A_>, Identity<A_>>>,
    A1::Pointed: Monoid,
    A1::Mapped: Applicative<A2, A2>,
    T: Term,
    A_: Term,
    A2: PureA<Pointed = Identity<A1::Pointed>>,
{
    fn traverse_t(self, f: impl FunctionT<T, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<A1, A_, A2> SequenceA<(), A2> for Identity<A1>
where
    A1: PureA<Pointed = A_> + Functor<Function<Identity<A_>, Identity<A_>>>,
    A1::Mapped: Applicative<A2, A2>,
    A_: Monoid,
    A2: PureA<Pointed = Identity<A1::Pointed>>,
{
    fn sequence_a(self) -> A2 {
        self.foldr(
            |next, acc| next.fmap(|t| (|_| Identity(t)).boxed()).app_a(acc),
            PureA::pure_a(Identity(Monoid::mempty())),
        )
    }
}
