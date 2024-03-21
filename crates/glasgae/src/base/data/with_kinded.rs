use super::kinded::Kinded;

/// Modification of a single free type parameter
pub trait WithKinded<T>: Kinded {
    type WithKinded: Kinded<Kinded = T>;
}

#[macro_export]
macro_rules! derive_with_kinded_unary {
    ($ty:ident<$free:ident>) => {
        impl<$free, U> $crate::prelude::WithKinded<U> for $ty<$free>
        where
            $free: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            type WithKinded = $ty<U>;
        }
    };
}
