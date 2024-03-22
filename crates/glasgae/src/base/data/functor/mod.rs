//! A [`Functor`] is a type with a structure-preserving mapping operation.
//!
//! A type f is a Functor if it provides a function `fmap` which, given any types `A` and `B`
//! lets you apply any function from `A -> B` to turn an `F<A>` into an `F<B>`,
//! preserving the structure of `F`. Furthermore `F` needs to adhere to the following:
//!
//! **Identity**
//! ```text
//! Functor::fmap.curry(identity) == identity
//! ```
//!
//! **Composition**
//! ```text
//! Functor::fmap.curry(g.compose(f)) == Functor::fmap.curry(f).compose(Functor::fmap.curry(g))
//! ```
//!
//! Note, that the second law follows from the free theorem of the type fmap and the first law,
//! so you need only check that the former condition holds.
//!
//! See <https://www.schoolofhaskell.com/user/edwardk/snippets/fmap>
//! or <https://github.com/quchen/articles/blob/master/second_functor_law.md>
//! for an explanation.

pub mod r#const;
pub mod identity;

use crate::prelude::*;

pub trait Functor<T>: WithPointed<T>
where
    T: Term,
{
    /// fmap is used to apply a function of type (a -> b) to a value of type f a, where f is a functor, to produce a value of type f b. Note that for any type constructor with more than one parameter (e.g., Either), only the last type parameter can be modified with fmap (e.g., b in `Either a b`).
    ///
    /// Some type constructors with two parameters or more have a Bifunctor instance that allows both the last and the penultimate parameters to be mapped over.
    ///
    /// ## Examples
    ///
    /// Convert from a `Maybe<usize>` to a `Maybe<String>` using show:
    ///
    ///```
    /// # use glasgae::prelude::{Functor, Show};
    /// assert_eq!(None::<usize>.fmap(Show::show), None);
    /// assert_eq!((Some(3)).fmap(Show::show), Some("3".to_string()));
    /// ```
    ///
    /// Convert from an `Either<Int, Int>, to an `Either<Int, String>` using show:
    ///
    /// ```
    /// # use glasgae::prelude::{Either::*, Functor, Show};
    /// assert_eq!(Left::<usize, String>(17).fmap(Show::show), Left(17));
    /// assert_eq!(Right::<usize, usize>(17).fmap(Show::show), Right("17".to_string()));
    /// ```
    ///
    /// Double each element of a list:
    /// ```
    /// # use glasgae::prelude::{Functor};
    /// assert_eq!(vec![1,2,3].fmap(|t| t * 2), vec![2,4,6]);
    /// ```
    ///
    /// Apply even to the second element of a pair:
    /// ```
    /// # use glasgae::{prelude::{Functor}, base::grl::num::Even};
    /// assert_eq!((2, 2).fmap(Even::even), (2,true));
    /// ```
    ///
    /// It may seem surprising that the function is only applied to the last element
    /// of the tuple compared to the list example above which applies it to every element in the list.
    ///
    /// To understand, remember that tuples are type constructors with multiple type parameters:
    /// a tuple of 3 elements (a,b,c) can also be written (,,) a b c
    /// and its Functor instance is defined for Functor ((,,) a b)
    ///
    /// (i.e., only the third parameter is free to be mapped over with fmap).
    ///
    /// It explains why fmap can be used with tuples containing values
    /// of different types as in the following example:
    /// ```
    /// # use glasgae::{prelude::Functor, base::grl::num::Even};
    /// assert_eq!(("hello", 1.0, 4).fmap(Even::even), ("hello",1.0,true));
    /// ```
    fn fmap(self, f: impl FunctionT<Self::Pointed, T>) -> Self::WithPointed;

    /// Replace all locations in the input with the same value. The default definition is fmap . const, but this may be overridden with a more efficient version.
    ///
    /// # Examples
    ///
    /// Perform a computation with Maybe and replace the result with a constant value if it is Some:
    /// ```
    /// # use glasgae::prelude::Functor;
    /// assert_eq!(Some(2).replace('a'), Some('a'));
    /// assert_eq!(None::<usize>.replace('a'), None);
    /// ```
    fn replace(self, t: T) -> Self::WithPointed
    where
        T: 'static,
    {
        self.fmap(|_| t)
    }
}

// Derive Functor over the inner type
#[macro_export]
macro_rules! derive_functor {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*, U> $crate::prelude::Functor<U> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            U: $crate::prelude::Term $(+ $trait$)*,
        {
            fn fmap(self, f: impl $crate::prelude::FunctionT<T, U>) -> $ty<$($_arg,)* U $(,$arg_)*> {
                $ty(f(self.0))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_functor_iterable {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*, U> $crate::prelude::Functor<U> for $ty<$($_arg,)* $arg $(,$arg_)*>
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
            fn fmap(
                self,
                f: impl $crate::prelude::FunctionT<
                    Self::Pointed,
                    <$ty<$($_arg,)* U $(,$arg_)*> as $crate::prelude::Pointed>::Pointed,
                >,
            ) -> $ty<$($_arg,)* U $(,$arg_)*> {
                self.into_iter().map(|t| f.to_function()(t)).collect()
            }
        }
    };
}
