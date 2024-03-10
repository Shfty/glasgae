//! Traits generalizing over parametrized types.

use crate::prelude::Term;

/// A [`Pointed`] type addresses a single free type parameter.
pub trait Pointed: Term {
    /// Free type parameter.
    type Pointed: Term;
}

/// Convenience alias to [`Pointed::Pointed`]
pub type PointedT<T> = <T as Pointed>::Pointed;

#[macro_export]
macro_rules! derive_pointed_unary {
    ($ty:ident<$free:ident>) => {
        impl<$free> $crate::prelude::Pointed for $ty<$free>
        where
            $free: $crate::prelude::Term,
        {
            type Pointed = $free;
        }
    };
}

/// A [`WithPointed`] type is a [`Pointed`] with the means to modify its free type parameter.
pub trait WithPointed<T>: Pointed {
    /// [`Self`], with [`Self::Pointed`](Pointed::Pointed) replaced by `T`.
    type WithPointed: Pointed<Pointed = T>;
}

/// Convenience alias to [`WithPointed::WithPointed`]
pub type WithPointedT<T, U> = <T as WithPointed<U>>::WithPointed;

#[macro_export]
macro_rules! derive_with_pointed_unary {
    ($ty:ident<$free:ident>) => {
        impl<$free, U> $crate::prelude::WithPointed<U> for $ty<$free>
        where
            $free: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            type WithPointed = $ty<U>;
        }
    };
}
