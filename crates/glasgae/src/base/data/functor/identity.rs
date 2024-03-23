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

impl<T, MA, A, MB> TraverseT<MA, (), MB> for Identity<T>
where
    MA: PureA<Pointed = A>
        + Functor<Function<Identity<A>, Identity<A>>>
        + WithPointed<Function<Identity<MA>, Identity<A>>>,
    MA::Mapped: Applicative<Identity<A>, Identity<A>, WithA = MB, WithB = MB>,
    T: Term,
    A: Monoid,
    MB: PureA<Pointed = Identity<A>>,
{
    type Inner = T;
    type Value = A;
    type Traversed = MB;

    fn traverse_t(self, f: impl FunctionT<T, MA>) -> MB {
        traverse_t_default(self, f)
    }
}

impl<MA, MF, A, MB> SequenceA<(), MB> for Identity<MA>
where
    MA: PureA<Pointed = A>
        + Functor<Function<Identity<A>, Identity<A>>, Mapped = MF>
        + WithPointed<Function<Identity<MA>, Identity<A>>>,
    MF: Applicative<Identity<A>, Identity<A>, WithA = MB, WithB = MB>,
    A: Monoid,
    MB: PureA<Pointed = Identity<A>>,
{
    type Inner = MA;
    type Value = A;
    type Sequenced = MB;

    fn sequence_a(self) -> MB {
        self.run()
            .fmap(|t| r#const(Identity(t)).boxed())
            .app_a(PureA::pure_a(Identity(Monoid::mempty())))
    }
}

fn test() {
    // Starting monad-over-monoid
    let monad_monoid: Identity<Vec<usize>> = Identity(vec![1234]);

    // Unwrap inner monoid
    let monoid_v: Vec<usize> = monad_monoid.run();

    // Map inner monoid to function from monoid-over-monoid to monad-over-value
    let monoid_f: Vec<Function<Identity<Vec<usize>>, Identity<usize>>> =
        monoid_v.fmap(|t| r#const(Identity(t)).to_function());

    // Apply function monoid to identity monad-over-monoid, resulting in monoid-over-monad
    let _monoid_monad: Vec<Identity<usize>> = monoid_f.app_a(PureA::pure_a(Monoid::mempty()));
}
