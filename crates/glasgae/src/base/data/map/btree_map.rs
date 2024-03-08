use std::collections::BTreeMap;

use crate::{
    base::data::FoldMap,
    prelude::{
        AppA, Boxed, Foldr, Function, FunctionT, Functor, Monoid, Pointed, PureA, Semigroup,
        SequenceA, TraverseT, WithPointed, WithPointedT,
    },
};

impl<K, V> Pointed for BTreeMap<K, V> {
    type Pointed = V;
}

impl<K, V, V_> WithPointed<V_> for BTreeMap<K, V> {
    type WithPointed = BTreeMap<K, V_>;
}

impl<K, V, V_> Functor<V_> for BTreeMap<K, V>
where
    K: Ord,
    V_: Clone,
{
    fn fmap(
        self,
        f: impl crate::prelude::FunctionT<Self::Pointed, V_> + Clone,
    ) -> Self::WithPointed {
        self.into_iter()
            .map(move |(k, v)| (k, f.clone()(v)))
            .collect()
    }
}

impl<K, V, V_> FoldMap<V, V_> for BTreeMap<K, V>
where
    V_: Monoid,
{
    fn fold_map(self, f: impl crate::prelude::FunctionT<V, V_> + Clone) -> V_ {
        self.into_values()
            .fold(Monoid::mempty(), |acc, next| acc.assoc_s(f.clone()(next)))
    }
}

impl<K, V, V_> Foldr<V, V_> for BTreeMap<K, V> {
    fn foldr(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<V, V_, V_> + Clone,
        z: V_,
    ) -> V_ {
        self.into_values()
            .rfold(z, |acc, next| f.clone()(next, acc))
    }
}

impl<K, V, A1, T, A2> TraverseT<A1, T, A2> for BTreeMap<K, V> {
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        todo!()
    }
}

impl<K, V1, V_, V2> SequenceA<V_, V2> for BTreeMap<K, V1> {
    fn sequence_a(self) -> V2 {
        todo!()
    }
}

pub fn insert<K, V>(k: K, v: V, mut m: BTreeMap<K, V>) -> BTreeMap<K, V>
where
    K: Ord,
{
    m.insert(k, v);
    m
}

impl<K, V> Semigroup for BTreeMap<K, V>
where
    K: Ord,
{
    fn assoc_s(self, a: Self) -> Self {
        self.into_iter().chain(a).collect()
    }
}

impl<K, V> Monoid for BTreeMap<K, V>
where
    K: 'static + Ord,
    V: 'static,
{
    fn mempty() -> Self {
        BTreeMap::new()
    }
}
