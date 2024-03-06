use crate::{
    base::data::functor::r#const::Const,
    prelude::{Either, Pointed, PureA},
};

use super::Lift;

/// An applicative functor that collects a monoid (e.g. lists) of errors.
///
/// A sequence of computations fails if any of its components do,
/// but unlike monads made with [`ExceptT`](crate::transformers::except::ExceptT) from [`except`](crate::transformers::except),
/// these computations continue after an error, collecting all the errors.
pub type Errors<E> = Lift<Const<E>>;

impl<E> Errors<E>
where
    E: Pointed,
{
    pub fn run(self) -> Either<E, E::Pointed> {
        match self {
            Lift::Pure(x) => Either::Right(x),
            Lift::Other(Const(e)) => Either::Left(e),
        }
    }

    pub fn failure(e: E) -> Self {
        Lift::Other(Const(e))
    }
}

impl<E, A> From<Either<E, A>> for Errors<E>
where
    E: Pointed<Pointed = A>,
{
    fn from(value: Either<E, A>) -> Self {
        match value {
            Either::Left(e) => Self::failure(e),
            Either::Right(x) => PureA::pure_a(x),
        }
    }
}

