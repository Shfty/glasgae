//! Traits generalizing over parametrized types.

use super::function::Term;

/// A [`Pointed`] type addresses a single free type parameter.
pub trait Pointed: Term {
    /// Free type parameter.
    type Pointed: Term;
}

/// Convenience alias to [`Pointed::Pointed`]
pub type PointedT<T> = <T as Pointed>::Pointed;

/// A [`WithPointed`] type is a [`Pointed`] with the means to modify its free type parameter.
pub trait WithPointed<T>: Pointed {
    /// [`Self`], with [`Self::Pointed`](Pointed::Pointed) replaced by `T`.
    type WithPointed: Pointed<Pointed = T>;
}

/// Convenience alias to [`WithPointed::WithPointed`]
pub type WithPointedT<T, U> = <T as WithPointed<U>>::WithPointed;

/// Given a pointed type, extend it with some extra data.
pub trait Lift<T, A>: Pointed<Pointed = A> + WithPointed<(A, T)> {
    type Lifted: Pointed<Pointed = (A, T)>;
}

/// Convenience alias to [`Lift::Lifted`].
pub type LiftedT<T, U, A> = <T as Lift<U, A>>::Lifted;

impl<MA, T, A> Lift<T, A> for MA
where
    MA: Pointed<Pointed = A> + WithPointed<(A, T)>,
{
    type Lifted = MA::WithPointed;
}

/// Given a pointed type that stores some extra data,
/// return the type minus that data.
pub trait Lower<T, A>: Pointed<Pointed = (A, T)> + WithPointed<A> {
    type Lowered: Pointed<Pointed = A>;
}

/// Convenience alias to [`Lower::Lowered`].
pub type LoweredT<T, U, A> = <T as Lower<U, A>>::Lowered;

impl<MA, T, A> Lower<T, A> for MA
where
    MA: Pointed<Pointed = (A, T)> + WithPointed<A>,
{
    type Lowered = MA::WithPointed;
}
