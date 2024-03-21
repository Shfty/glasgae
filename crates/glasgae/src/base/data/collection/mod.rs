pub mod list;
pub mod map;
pub mod set;

/// Implement standard typeclasses for a type with std::collection iterator semantics
#[macro_export]
macro_rules! derive_iterable_collection {
    ($ty:ident<$arg:ident>, $append:ident $(, $trait:path)*) => {
        impl<$arg> $crate::prelude::Kinded for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
        {
            type Kinded = $arg;
        }

        impl<$arg, U> $crate::prelude::WithKinded<U> for $ty<$arg>
        where
            $arg: $crate::prelude::Term,
            U: $crate::prelude::Term,
        {
            type WithKinded = $ty<U>;
        }

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

        impl<$arg, U> $crate::prelude::Functor<U> for $ty<$arg>
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
            fn app_a(self, xs: $ty<A>) -> $ty<B> {
                let fs = self;
                $crate::prelude::ChainM::chain_m(fs, |f| $crate::prelude::ChainM::chain_m(xs, |x| $crate::prelude::ReturnM::return_m(f(x))))
            }
        }

        impl<$arg> $crate::prelude::ReturnM for $ty<$arg> where $arg: $crate::prelude::Term $(+ $trait)* {}

        impl<$arg, U> $crate::prelude::ChainM<U> for $ty<$arg>
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

        impl<$arg, A_, A1, A2> $crate::prelude::TraverseT<A1, (), A2> for $ty<$arg>
        where
            A1: $crate::prelude::Functor<$crate::prelude::Function<$ty<A_>, $ty<A_>>, Pointed = A_> $(+ $trait)*,
            A1::WithPointed: $crate::prelude::Applicative<A2, A2>,
            A_: $crate::prelude::Term $(+ $trait)*,
            A2: $crate::prelude::PureA<Pointed = $ty<A_>> $(+ $trait)*,
            $arg: $crate::prelude::Term $(+ $trait)*,
        {
            fn traverse_t(self, f: impl $crate::prelude::FunctionT<Self::Pointed, A1>) -> A2 {
                let f = f.to_function();
                $crate::prelude::Foldable::foldr(
                    self,
                    |x, ys|
                        $crate::prelude::LiftA2::lift_a2($append)(
                            f(x),
                            ys
                        ),
                    $crate::prelude::PureA::pure_a($ty::new())
                )
            }
        }

        impl<$arg, A2> $crate::prelude::SequenceA<(), A2> for $ty<$arg>
        where
            Self: $crate::prelude::TraverseT<$arg, (), A2, Pointed = $arg>,
            $arg: $crate::prelude::Term,
            A2: $crate::prelude::Term
        {
            fn sequence_a(self) -> A2 {
                $crate::prelude::sequence_a_default(self)
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
