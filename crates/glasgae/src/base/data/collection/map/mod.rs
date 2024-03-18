pub mod btree_map;
pub mod hash_map;
pub mod vec_map;

/// Implement standard typeclasses for a type with std::collection iterator semantics
#[macro_export]
macro_rules! derive_iterable_map {
    ($ty:ident<$key:ident, $value:ident>, $append:ident $(, $trait:path)*) => {
        impl<$key, $value> $crate::prelude::Pointed for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            $value: $crate::prelude::Term,
        {
            type Pointed = $value;
        }

        impl<$key, $value, V_> $crate::prelude::WithPointed<V_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            $value: $crate::prelude::Term,
            V_: $crate::prelude::Term,
        {
            type WithPointed = $ty<$key, V_>;
        }

        impl<$key, $value, V_> $crate::prelude::Fmap<V_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
            V_: $crate::prelude::Term,
        {
            fn fmap(self, f: impl $crate::prelude::FunctionT<Self::Pointed, V_>) -> Self::WithPointed {
                self.into_iter()
                    .map(move |(k, v)| (k, f.to_function()(v)))
                    .collect()
            }
        }

        impl<$key, $value, V_> $crate::prelude::FoldMap<V_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
            V_: $crate::prelude::Monoid,
        {
            fn fold_map(self, f: impl $crate::prelude::FunctionT<$value, V_>) -> V_ {
                V_::mconcat(self.into_iter().map(|(_, v)| f.to_function()(v)).collect())
            }
        }

        impl<$key, $value, V_> $crate::prelude::Foldable<V_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
        {
            fn foldr(self, f: impl $crate::prelude::BifunT<$value, V_, V_>, z: V_) -> V_ {
                self
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rfold(z, |acc, (_, v)| f.to_bifun()(v, acc))
            }

            fn foldl(self, f: impl $crate::prelude::BifunT<V_, $value, V_>, z: V_) -> V_ {
                self
                    .into_iter()
                    .fold(z, |acc, (_, v)| f.to_bifun()(acc, v))
            }
        }

        impl<$key, $value> $crate::prelude::Foldable1<$value> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
        {
            fn foldr1(self, f: impl $crate::prelude::BifunT<$value, $value, $value>) -> V {
                $crate::prelude::foldr1_default(self, f)
            }

            fn foldl1(self, f: impl $crate::prelude::BifunT<$value, $value, $value>) -> V {
                $crate::prelude::foldl1_default(self, f)
            }
        }

        impl<$key, $value, A1, A2> $crate::prelude::TraverseT<A1, A2> for $ty<$key, $value>
        where
            Self: $crate::prelude::Fmap<A1>,
            $crate::prelude::WithPointedT<Self, A1>: $crate::prelude::SequenceA<A2>,
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
            A1: $crate::prelude::Term,
        {
            fn traverse_t(self, f: impl $crate::prelude::FunctionT<Self::Pointed, A1>) -> A2 {
                $crate::prelude::traverse_t_default(self, f)
            }
        }

        impl<$key, $value, V2> $crate::prelude::SequenceA<V2> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Fmap<$crate::prelude::Function<$ty<$key, $value>, $ty<$key, $value>>, Pointed = ($key, $value)>,
            $crate::prelude::WithPointedT<$value, $crate::prelude::Function<$ty<$key, $value>, $ty<$key, $value>>>: $crate::prelude::AppA<V2, V2>,
            V2: $crate::prelude::PureA<Pointed = $ty<$key, $value>>,
        {
            fn sequence_a(self) -> V2 {
                $crate::prelude::Foldable::foldr(
                    self,
                    |next, acc| {
                        $crate::prelude::AppA::app_a(
                            next.fmap(|(k, v)|
                                Box::new(|t| $append(k, v, t))
                            ),
                            acc,
                        )
                    },
                    $crate::prelude::PureA::pure_a($ty::new()),
                )
            }
        }

        impl<$key, $value> $crate::prelude::Bipointed for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            $value: $crate::prelude::Term,
        {
            type Bipointed = K;
        }

        impl<$key, K_, $value> $crate::prelude::WithBipointed<K_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            K_: $crate::prelude::Term,
            $value: $crate::prelude::Term,
        {
            type WithBipointed = $ty<K_, $value>;
        }

        impl<$key, K_, $value> $crate::prelude::Bifmap<K_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            K_: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
        {
            fn bifmap(self, f: impl $crate::prelude::FunctionT<Self::Bipointed, K_>) -> Self::WithBipointed {
                self.into_iter()
                    .map(|(k, v)| (f.to_function()(k), v))
                    .collect()
            }
        }

        impl<$key, K_, $value, V_> $crate::prelude::Bifunctor<K_, V_> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            K_: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
            V_: $crate::prelude::Term,
        {
        }

        impl<$key, $value, T> $crate::prelude::Bifoldable<T> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term,
            $value: $crate::prelude::Term,
            T: $crate::prelude::Term,
        {
            fn bifoldr(
                self,
                fa: impl $crate::prelude::BifunT<Self::Bipointed, T, T>,
                fb: impl $crate::prelude::BifunT<Self::Pointed, T, T>,
                z: T,
            ) -> T {
                self.into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rfold(z, |acc, (k, v)| fb.to_bifun()(v, fa.to_bifun()(k, acc)))
            }

            fn bifoldl(
                self,
                fa: impl $crate::prelude::BifunT<T, Self::Bipointed, T>,
                fb: impl $crate::prelude::BifunT<T, Self::Pointed, T>,
                z: T,
            ) -> T {
                self.into_iter()
                    .fold(z, |acc, (k, v)| fb.to_bifun()(fa.to_bifun()(acc, k), v))
            }
        }

        impl<$key, $value, AC, AD, AO> $crate::prelude::BitraverseT<AC, AD, AO> for $ty<$key, $value>
        where
            Self: $crate::prelude::Bifunctor<AC, AD>,
            $crate::prelude::WithBipointedT<Self, AC>: $crate::prelude::Fmap<AD>,
            $crate::prelude::WithPointedT<$crate::prelude::WithBipointedT<Self, AC>, AD>: $crate::prelude::BisequenceA<AO>,
            AC: $crate::prelude::Term,
            AD: $crate::prelude::Term,
        {
            fn bitraverse_t(
                self,
                fa: impl $crate::prelude::FunctionT<Self::Bipointed, AC>,
                fb: impl $crate::prelude::FunctionT<Self::Pointed, AD>,
            ) -> AO {
                $crate::prelude::BisequenceA::bisequence_a($crate::prelude::Bifunctor::bimap(self, fa, fb))
            }
        }

        impl<$key, $value, T, AO> $crate::prelude::BisequenceA<AO> for $ty<$key, $value>
        where
            $key: $crate::prelude::Term + $crate::prelude::Fmap<$crate::prelude::Function<Vec<$key>, Vec<$key>>, Pointed = $key>,
            $key::WithPointed: $crate::prelude::AppA<AO, AO>,
            $value: $crate::prelude::Term + $crate::prelude::Fmap<$crate::prelude::Function<Vec<$value>, Vec<$value>>, Pointed = $value>,
            $value::WithPointed: $crate::prelude::AppA<AO, AO>,
            AO: $crate::prelude::PureA<Pointed = Vec<T>>,
        {
            fn bisequence_a(self) -> AO {
                $crate::prelude::Bifoldable::bifoldr(
                    self,
                    |next, acc| $crate::prelude::AppA::app_a(next.fmap(|k| Box::new(|t| $crate::prelude::list::vec::push(k, t))), acc),
                    |next, acc| $crate::prelude::AppA::app_a(next.fmap(|v| Box::new(|t| $crate::prelude::list::vec::push(v, t))), acc),
                    $crate::prelude::PureA::pure_a(vec![]),
                )
            }
        }

        impl<$key, $value> $crate::prelude::Semigroup for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
        {
            fn assoc_s(self, a: Self) -> Self {
                self.into_iter().chain(a).collect()
            }
        }

        impl<$key, $value> $crate::prelude::Monoid for $ty<$key, $value>
        where
            $key: $crate::prelude::Term $(+ $trait)*,
            $value: $crate::prelude::Term,
        {
            fn mempty() -> Self {
                $ty::new()
            }
        }
    };
}
