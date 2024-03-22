//! Traits generalizing over parametrized types.

use crate::prelude::Term;

/// A [`Pointed`] type addresses a single free type parameter.
pub trait Pointed: Term {
    /// Free type parameter.
    type Pointed: Term;
}

/// Convenience alias to [`Pointed::Pointed`]
pub type PointedT<T> = <T as Pointed>::Pointed;

// Derive Pointed over the inner type
#[macro_export]
macro_rules! derive_pointed {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::Pointed for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            type Pointed = $arg;
        }
    };
}

// Derive Pointed by recursing into the inner type
#[macro_export]
macro_rules! derive_pointed_via {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::Pointed for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Pointed $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            type Pointed = $arg::Pointed;
        }
    };
}
