use crate::prelude::Term;

/// The class of types with an associative binary operation.
///
/// Instances should satisfy the following:
/// ### Associativity
/// ```text
/// x.assoc_s(y.assoc_s(z)) == x.assoc_s(y).assoc_s(z)
/// ```
///
/// You can alternatively define `sconcat` instead of [`assoc_s`](Semigroup::assoc_s), in which case the laws are:
///
/// ### Unit
/// ```text
/// PureA::pure_a(x).concat_s() == x
/// ```
///
/// ### Multiplication
/// ```text
/// xss.join().concat_s() == xss.fmap(ConcatS::concat_s).concat_s()
/// ```
pub trait Semigroup: Term {
    /// An associative operation.
    ///
    /// # Examples
    /// ```
    /// # use glasgae::prelude::Semigroup;
    /// assert_eq!(
    ///     vec![1,2,3]
    ///         .assoc_s(vec![4,5,6]),
    ///     vec![1, 2, 3, 4, 5, 6]
    /// );
    /// ```
    /// ```
    /// # use glasgae::prelude::Semigroup;
    /// assert_eq!(
    ///     Some(vec![1, 2, 3])
    ///         .assoc_s(Some(vec![4, 5, 6])),
    ///     Some(vec![1, 2, 3, 4, 5, 6])
    /// );
    /// ```
    /// ```
    /// # use glasgae::prelude::{Semigroup, Show, put_str, put_str_ln};
    /// let io = put_str("Hello, ".show()).assoc_s(
    ///     put_str_ln("World!".show())
    /// );
    ///
    /// // Prints "Hello, World!"
    /// unsafe { io.run() };
    /// ```
    fn assoc_s(self, a: Self) -> Self;
}

#[macro_export]
macro_rules! derive_semigroup_iterable {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::Semigroup for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn assoc_s(self, a: Self) -> Self {
                self.into_iter().chain(a).collect()
            }
        }
    };
}

