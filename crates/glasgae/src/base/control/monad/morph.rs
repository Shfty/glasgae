//! Monad morphisms.

use crate::prelude::{Either, Pointed, PointedT, Term, WithPointed};

use super::Monad;

/// Lift extra data into a monadic type.
pub trait MonadLift<B>: WithPointed<B> {
    type Lifted: Pointed<Pointed = B>;
}

impl<MA, A, B> MonadLift<(A, B)> for MA
where
    MA: Monad<(A, B), Pointed = A>,
    A: Term,
    B: Term,
{
    type Lifted = MA::Chained;
}

impl<MA, A, B> MonadLift<Either<A, B>> for MA
where
    MA: Monad<Either<A, B>, Pointed = B>,
    A: Term,
    B: Term,
{
    type Lifted = MA::Chained;
}

/// Convenience alias to [`MonadLift::Lifted`].
pub type MonadLiftedT<T, B> = <T as MonadLift<B>>::Lifted;

/// Lower extra data out of a monadic type.
pub trait MonadLower<T, A>: Pointed {
    type Lowered: Pointed;
}

impl<MA, A, B> MonadLower<A, B> for MA
where
    MA: Monad<LoweredT<PointedT<MA>, A, B>>,
    LoweredT<PointedT<MA>, A, B>: Term,
    MA::Pointed: Lower<A, B>,
{
    type Lowered = MA::Chained;
}

/// Convenience alias to [`Lower::Lowered`].
pub type MonadLoweredT<T, A, B> = <T as MonadLower<A, B>>::Lowered;

pub trait Lower<A, B> {
    type Lowered;
}

impl<A, B> Lower<A, B> for (A, B) {
    type Lowered = A;
}

impl<A, B> Lower<A, B> for Either<A, B> {
    type Lowered = B;
}

pub type LoweredT<T, A, B> = <T as Lower<A, B>>::Lowered;

// Hoisting
// -----------------------------------------------------------------------------

/// Utility alias for hoisting transformers that use tuple data
pub type HoistTupleT<T, U> = MonadLiftedT<T, (PointedT<T>, U)>;

/// Utility alias for hoisting transformers that use Either data
pub type HoistEitherT<T, E> = MonadLiftedT<T, Either<E, PointedT<T>>>;
