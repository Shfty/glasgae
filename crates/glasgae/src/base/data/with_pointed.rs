use crate::prelude::Pointed;

/// A [`WithPointed`] type is a [`Pointed`] with the means to modify its free type parameter.
pub trait WithPointed<T>: Pointed {
    /// [`Self`], with [`Self::Pointed`](Pointed::Pointed) replaced by `T`.
    type WithPointed: Pointed<Pointed = T>;
}

/// Convenience alias to [`WithPointed::WithPointed`]
pub type WithPointedT<T, U> = <T as WithPointed<U>>::WithPointed;

// Derive WithPointed over the inner type
#[macro_export]
macro_rules! derive_with_pointed {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*, U> $crate::prelude::WithPointed<U> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            U: $crate::prelude::Term $(+ $trait)*
        {
            type WithPointed = $ty<$($_arg,)* U $(, $arg_)*>;
        }
    };
}

// Derive WithPointed by recursing into the inner type
#[macro_export]
macro_rules! derive_with_pointed_via {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*, U> $crate::prelude::WithPointed<U> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::WithPointed<U> $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            U: $crate::prelude::Term $($(+ $trait)*)?
        {
            type WithPointed = $ty<$($_arg,)* $arg::WithPointed $(, $arg_)*>;
        }
    };
}
