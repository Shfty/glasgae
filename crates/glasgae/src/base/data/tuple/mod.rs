pub mod one;
pub mod pair;
pub mod unit;

use crate::prelude::{Pointed, WithPointed};

pub trait Cons<T>: Sized {
    fn cons(self, t: T) -> (Self, T) {
        (self, t)
    }
}

impl<T, U> Cons<U> for T {}

pub trait Snoc<T>: Sized {
    fn snoc(self, t: T) -> (T, Self) {
        (t, self)
    }
}

impl<T, U> Snoc<U> for T {}

pub trait Apply<const N: usize, A, B, C> {
    fn apply(self, f: impl FnOnce(A, B) -> C) -> C;
}

impl<A, B, C> Apply<2, A, B, C> for (A, B) {
    fn apply(self, f: impl FnOnce(A, B) -> C) -> C {
        f(self.0, self.1)
    }
}

macro_rules ! impl_tuple {
    ($a:tt) => {
        impl<$a> $crate::prelude::Kinded for ($a,)
        where
            $a: $crate::prelude::Term
        {
            type Kinded = $a;
        }

        impl<$a, U> $crate::prelude::WithKinded<U> for ($a,)
        where
            $a: $crate::prelude::Term,
            U: $crate::prelude::Term
        {
            type WithKinded = (U,);
        }

        impl<$a> $crate::prelude::Pointed for ($a,) where $a: $crate::prelude::Term {
            type Pointed = $a;
        }

        impl<$a, R_> $crate::prelude::WithPointed<R_> for ($a,)
        where
            $a: $crate::prelude::Term,
            R_: $crate::prelude::Term
        {
            type WithPointed = (R_,);
        }

        impl<$a, R_> $crate::prelude::Fmap<R_> for ($a,)
        where
            $a: $crate::prelude::Term,
            R_: $crate::prelude::Term,
        {
            fn fmap(
                self,
                f: impl $crate::prelude::FunctionT<Self::Pointed, R_>,
            ) -> Self::WithPointed {
                let (t,) = self;
                (f(t),)
            }
        }
    };
    ($a:tt | $($tuple:tt),*) => {
        impl<$($tuple,)* $a> $crate::prelude::Kinded for ($($tuple,)* $a)
        where
            $(
                $tuple: $crate::prelude::Term,
            )*
            $a: $crate::prelude::Term,
        {
            type Kinded = $a;
        }

        impl<$($tuple,)* $a, U> $crate::prelude::WithKinded<U> for ($($tuple,)* $a)
        where
            $(
                $tuple: $crate::prelude::Term,
            )*
            $a: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            type WithKinded = ($($tuple,)* U,);
        }


        impl<$($tuple,)* $a> Pointed for ($($tuple,)* $a)
        where
            $(
                $tuple: $crate::prelude::Term,
            )*
            $a: $crate::prelude::Term,
        {
            type Pointed = $a;
        }

        impl<$($tuple,)* $a, R_> WithPointed<R_> for ($($tuple,)* $a)
        where
            $(
                $tuple: $crate::prelude::Term,
            )*
            $a: $crate::prelude::Term,
            R_: $crate::prelude::Term,
        {
            type WithPointed = ($($tuple,)* R_,);
        }

        impl<$($tuple,)* $a, R_> $crate::prelude::Fmap<R_> for ($($tuple,)* $a)
        where
            $(
                $tuple: $crate::prelude::Term,
            )*
            $a: $crate::prelude::Term,
            R_: $crate::prelude::Term,
        {
            fn fmap(
                self,
                f: impl $crate::prelude::FunctionT<Self::Pointed, R_>,
            ) -> Self::WithPointed {
                #[allow(non_snake_case)]
                let ($($tuple,)* t,) = self;
                ($($tuple,)* f(t),)
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(B | A);
impl_tuple!(C | A, B);
impl_tuple!(D | A, B, C);
impl_tuple!(E | A, B, C, D);
impl_tuple!(F | A, B, C, D, E);
impl_tuple!(G | A, B, C, D, E, F);
impl_tuple!(H | A, B, C, D, E, F, G);
impl_tuple!(I | A, B, C, D, E, F, G, H);
impl_tuple!(J | A, B, C, D, E, F, G, H, I);
impl_tuple!(K | A, B, C, D, E, F, G, H, I, J);
impl_tuple!(L | A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(M | A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(N | A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(O | A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(P | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
