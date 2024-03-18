mod append;
mod filter;

pub use append::*;
pub use filter::*;

pub mod array;
pub mod string;
pub mod vec;
pub mod vec_deque;
pub mod linked_list;

/// Implement standard typeclasses for a type with std::collection list semantics
#[macro_export]
macro_rules! impl_list {
    ($ty:ident<$arg:ident>, $append:ident) => {
        impl<$arg> $crate::prelude::Pointed for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
        {
            type Pointed = $arg;
        }

        impl<$arg, U> $crate::prelude::WithPointed<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            type WithPointed = $ty<U>;
        }

        impl<$arg, U> $crate::prelude::Fmap<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
            U: $crate::prelude::Term,
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
            $arg: $crate::prelude::Term,
        {
            fn pure_a(t: Self::Pointed) -> Self {
                FromIterator::from_iter([t])
            }
        }

        impl<$arg, A, B> $crate::prelude::AppA<$ty<A>, $ty<B>> for $ty<$arg>
        where
            $arg: $crate::prelude::Term + $crate::prelude::FunctionT<A, B>,
            A: $crate::prelude::Term,
            B: $crate::prelude::Term,
        {
            fn app_a(self, a: $ty<A>) -> $ty<B> {
                self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
            }
        }

        impl<$arg> $crate::prelude::ReturnM for $ty<$arg> where $arg: $crate::prelude::Term {}

        impl<$arg, U> $crate::prelude::ChainM<$ty<U>> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            fn chain_m(self, f: impl $crate::prelude::FunctionT<$arg, $ty<U>>) -> $ty<U> {
                self.into_iter().flat_map(|t| f.to_function()(t)).collect()
            }
        }

        impl<$arg, U> $crate::prelude::Foldable<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
        {
            fn foldr(
                self,
                f: impl $crate::base::data::function::bifunction::BifunT<$arg, U, U>,
                init: U,
            ) -> U {
                self.into_iter()
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
            $arg: $crate::prelude::Term,
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
                self.into_iter()
                    .rev()
                    .reduce(|y, x| f.to_bifun()(x, y))
                    .unwrap()
            }
        }

        impl<$arg, U> $crate::prelude::FoldMap<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
            U: $crate::prelude::Monoid,
        {
            fn fold_map(self, f: impl $crate::prelude::FunctionT<$arg, U> + Clone) -> U {
                U::mconcat(self.into_iter().map(|t| f.to_function()(t)).collect())
            }
        }

        impl<$arg, A_, A1, A2> $crate::prelude::TraverseT<A1, A_, A2> for $ty<$arg>
        where
            A1: $crate::prelude::Fmap<$crate::prelude::Function<$ty<A_>, $ty<A_>>, Pointed = A_>,
            A1::WithPointed: $crate::prelude::AppA<A2, A2>,
            A_: $crate::prelude::Term,
            A2: $crate::prelude::PureA<Pointed = $ty<A_>>,
            $arg: $crate::prelude::Term,
        {
            fn traverse_t(self, f: impl $crate::prelude::FunctionT<Self::Pointed, A1>) -> A2 {
                $crate::base::data::traversable::traverse_t_default(self, f)
            }
        }

        impl<$arg, A_, A2> $crate::prelude::SequenceA<A_, A2> for $ty<$arg>
        where
            $arg: $crate::prelude::Fmap<$crate::prelude::Function<$ty<A_>, $ty<A_>>, Pointed = A_>,
            $arg::WithPointed: $crate::prelude::AppA<A2, A2>,
            A_: $crate::prelude::Term,
            A2: $crate::prelude::PureA<Pointed = $ty<A_>>,
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
            $arg: $crate::prelude::Term,
        {
            fn assoc_s(self, a: Self) -> Self {
                self.into_iter().chain(a).collect()
            }
        }

        impl<$arg> $crate::prelude::Monoid for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
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
