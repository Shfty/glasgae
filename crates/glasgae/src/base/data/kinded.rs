use crate::prelude::Term;

/// Abstraction over a single free type parameter
pub trait Kinded: Term {
    type Kinded: Term;
}

#[macro_export]
macro_rules! derive_kinded_unary {
    ($ty:ident<$free:ident>) => {
        impl<$free> $crate::prelude::Kinded for $ty<$free>
        where
            $free: $crate::prelude::Term,
        {
            type Kinded = $free;
        }
    };
}

