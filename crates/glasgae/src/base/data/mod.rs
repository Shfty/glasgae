//! # Data

pub mod tree;

pub mod bifoldable;
pub mod bifoldable1;
pub mod bifunctor;
pub mod bipointed;
pub mod bitraversable;
pub mod with_bipointed;

pub mod collection;
pub mod either;
pub mod function;
pub mod functor;
pub mod maybe;
pub mod monoid;
pub mod pointed;
pub mod term;
pub mod traversable;
pub mod tuple;

mod boxed;
mod foldable;
mod foldable1;
mod semigroup;

pub use bifoldable::*;
pub use bifoldable1::*;
pub use bitraversable::*;
pub use boxed::*;
pub use foldable::*;
pub use foldable1::*;
pub use semigroup::*;

/// Implement standard typeclasses for a type with std::collection iterator semantics
#[macro_export]
macro_rules! derive_iterable_collection {
    ($ty:ident<$arg:ident>, $append:ident $(, $trait:path)*) => {
        impl<$arg> $crate::prelude::Pointed for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            type Pointed = $arg;
        }

        impl<$arg, U> $crate::prelude::WithPointed<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
            U: $crate::prelude::Term $(+ $trait)*,
        {
            type WithPointed = $ty<U>;
        }

        impl<$arg, U> $crate::prelude::Fmap<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
            U: $crate::prelude::Term $(+ $trait)*,
        {
            fn fmap(
                self,
                f: impl $crate::prelude::FunctionT<
                    Self::Pointed,
                    <$ty<U> as $crate::prelude::Pointed>::Pointed,
                >,
            ) -> $ty<U> {
                self.into_iter().map(|t| f.to_function()(t)).collect()
            }
        }

        impl<$arg> $crate::prelude::PureA for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn pure_a(t: Self::Pointed) -> Self {
                FromIterator::from_iter([t])
            }
        }

        impl<$arg, A, B> $crate::prelude::AppA<$ty<A>, $ty<B>> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)* + $crate::prelude::FunctionT<A, B>,
            A: $crate::prelude::Term $(+ $trait)*,
            B: $crate::prelude::Term $(+ $trait)*,
        {
            fn app_a(self, a: $ty<A>) -> $ty<B> {
                self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
            }
        }

        impl<$arg> $crate::prelude::ReturnM for $ty<$arg> where $arg: $crate::prelude::Term $(+ $trait)* {}

        impl<$arg, U> $crate::prelude::ChainM<$ty<U>> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
            U: $crate::prelude::Term $(+ $trait)*,
        {
            fn chain_m(self, f: impl $crate::prelude::FunctionT<$arg, $ty<U>>) -> $ty<U> {
                self.into_iter().flat_map(|t| f.to_function()(t)).collect()
            }
        }

        impl<$arg, U> $crate::prelude::Foldable<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn foldr(
                self,
                f: impl $crate::base::data::function::bifunction::BifunT<$arg, U, U>,
                init: U,
            ) -> U {
                self
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rfold(init, |acc, next| f.to_bifun()(next, acc))
            }

            fn foldl(
                self,
                f: impl $crate::base::data::function::bifunction::BifunT<U, $arg, U>,
                init: U,
            ) -> U {
                self.into_iter()
                    .fold(init, |acc, next| f.to_bifun()(acc, next))
            }
        }

        impl<$arg> $crate::prelude::Foldable1<$arg> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn foldr1(
                self,
                f: impl $crate::base::data::function::bifunction::BifunT<$arg, $arg, $arg>,
            ) -> $arg {
                self.into_iter().reduce(|x, y| f.to_bifun()(x, y)).unwrap()
            }

            fn foldl1(
                self,
                f: impl $crate::base::data::function::bifunction::BifunT<$arg, $arg, $arg>,
            ) -> $arg {
                self
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .reduce(|y, x| f.to_bifun()(x, y))
                    .unwrap()
            }
        }

        impl<$arg, U> $crate::prelude::FoldMap<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
            U: $crate::prelude::Monoid,
        {
            fn fold_map(self, f: impl $crate::prelude::FunctionT<$arg, U> + Clone) -> U {
                U::mconcat(self.into_iter().map(|t| f.to_function()(t)).collect())
            }
        }

        impl<$arg, A_, A1, A2> $crate::prelude::TraverseT<A1, A_, A2> for $ty<$arg>
        where
            A1: $crate::prelude::Fmap<$crate::prelude::Function<$ty<A_>, $ty<A_>>, Pointed = A_> $(+ $trait)*,
            A1::WithPointed: $crate::prelude::AppA<A2, A2>,
            A_: $crate::prelude::Term $(+ $trait)*,
            A2: $crate::prelude::PureA<Pointed = $ty<A_>> $(+ $trait)*,
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn traverse_t(self, f: impl $crate::prelude::FunctionT<Self::Pointed, A1>) -> A2 {
                $crate::base::data::traversable::traverse_t_default(self, f)
            }
        }

        impl<$arg, A_, A2> $crate::prelude::SequenceA<A_, A2> for $ty<$arg>
        where
            $arg: $crate::prelude::Fmap<$crate::prelude::Function<$ty<A_>, $ty<A_>>, Pointed = A_> $(+ $trait)*,
            $arg::WithPointed: $crate::prelude::AppA<A2, A2>,
            A_: $crate::prelude::Term $(+ $trait)*,
            A2: $crate::prelude::PureA<Pointed = $ty<A_>> $(+ $trait)*,
        {
            fn sequence_a(self) -> A2 {
                $crate::prelude::Foldable::foldr(
                    self,
                    |next, acc| {
                        $crate::prelude::AppA::app_a(
                            next.fmap(|t| Box::new(|v| $append(t, v))),
                            acc,
                        )
                    },
                    $crate::prelude::PureA::pure_a(Default::default()),
                )
            }
        }

        impl<$arg> $crate::prelude::Semigroup for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn assoc_s(self, a: Self) -> Self {
                self.into_iter().chain(a).collect()
            }
        }

        impl<$arg> $crate::prelude::Monoid for $ty<$arg>
        where
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn mempty() -> Self {
                Default::default()
            }

            fn mconcat(list: Vec<Self>) -> Self {
                list.into_iter().flatten().collect()
            }
        }
    };
}
