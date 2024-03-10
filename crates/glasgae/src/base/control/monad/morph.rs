//! Monad morphisms.

use crate::prelude::{Either, Pointed, PointedT, WithPointed};

/// Lift extra data into a monadic type.
pub trait MonadLift<T>: Pointed {
    type Lifted: Pointed;
}

impl<MA, A, B> MonadLift<(A, B)> for MA
where
    MA: Pointed<Pointed = A> + WithPointed<(A, B)>,
{
    type Lifted = MA::WithPointed;
}

impl<MA, A, B> MonadLift<Either<A, B>> for MA
where
    MA: Pointed<Pointed = B> + WithPointed<Either<A, B>>,
{
    type Lifted = MA::WithPointed;
}

/// Convenience alias to [`MonadLift::Lifted`].
pub type MonadLiftedT<T, U> = <T as MonadLift<U>>::Lifted;

/// Lower extra data out of a monadic type.
pub trait MonadLower<T, A>: Pointed {
    type Lowered: Pointed;
}

impl<MA, A, B> MonadLower<A, B> for MA
where
    MA: Pointed + WithPointed<LoweredT<PointedT<MA>, A, B>>,
    MA::Pointed: Lower<A, B>,
{
    type Lowered = MA::WithPointed;
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
    type Lowered = A;
}

pub type LoweredT<T, A, B> = <T as Lower<A, B>>::Lowered;

// Hoisting
// -----------------------------------------------------------------------------

/// Utility alias for hoisting transformers that use tuple data
pub type HoistTupleT<T, U> = MonadLiftedT<T, (PointedT<T>, U)>;

/// Utility alias for hoisting transformers that use Either data
pub type HoistEitherT<T, U> = MonadLiftedT<T, Either<U, PointedT<T>>>;
