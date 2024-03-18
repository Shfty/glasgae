use crate::prelude::Pointed;

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
