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

