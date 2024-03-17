use std::collections::BTreeMap;

use crate::{
    base::data::{
        foldl1_default, foldr1_default, function::bifunction::BifunT,
        traversable::traverse_t_default, FoldMap, Foldable1,
    },
    prelude::*,
};

impl<K, V> Pointed for BTreeMap<K, V>
where
    K: Term,
    V: Term,
{
    type Pointed = V;
}

impl<K, V, V_> WithPointed<V_> for BTreeMap<K, V>
where
    K: Term,
    V: Term,
    V_: Term,
{
    type WithPointed = BTreeMap<K, V_>;
}

impl<K, V, V_> Fmap<V_> for BTreeMap<K, V>
where
    K: Term + Ord,
    V: Term,
    V_: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, V_>) -> Self::WithPointed {
        self.into_iter()
            .map(move |(k, v)| (k, f.to_function()(v)))
            .collect()
    }
}

impl<K, V, V_> FoldMap<V_> for BTreeMap<K, V>
where
    K: Term + Ord,
    V: Term,
    V_: Monoid,
{
    fn fold_map(mut self, f: impl FunctionT<V, V_>) -> V_ {
        let mut acc = V_::mempty();
        while let Some((_, next)) = self.pop_first() {
            acc = f.to_function()(next).assoc_s(acc);
        }
        acc
    }
}

impl<K, V, V_> Foldable<V_> for BTreeMap<K, V>
where
    K: Term + Ord,
    V: Term,
{
    fn foldr(mut self, f: impl BifunT<V, V_, V_>, z: V_) -> V_ {
        let mut acc = z;
        while let Some((_, next)) = self.pop_last() {
            acc = f.to_bifun()(next, acc);
        }
        acc
    }

    fn foldl(mut self, f: impl BifunT<V_, V, V_>, z: V_) -> V_ {
        let mut acc = z;
        while let Some((_, next)) = self.pop_first() {
            acc = f.to_bifun()(acc, next);
        }
        acc
    }
}

impl<K, V> Foldable1<V> for BTreeMap<K, V>
where
    K: Term + Ord,
    V: Term,
{
    fn foldr1(self, f: impl BifunT<V, V, V>) -> V {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<V, V, V>) -> V {
        foldl1_default(self, f)
    }
}

impl<K, V, A1, T, A2> TraverseT<A1, T, A2> for BTreeMap<K, V>
where
    Self: Fmap<A1>,
    WithPointedT<Self, A1>: SequenceA<T, A2>,
    K: Ord + Term,
    V: Term,
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<K, V1, V_, V2> SequenceA<V_, V2> for BTreeMap<K, V1>
where
    K: Term,
    V1: Term,
{
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
    K: Term + Ord,
    V: Term,
{
    fn assoc_s(self, a: Self) -> Self {
        self.into_iter().chain(a).collect()
    }
}

impl<K, V> Monoid for BTreeMap<K, V>
where
    K: Term + Ord,
    V: Term,
{
    fn mempty() -> Self {
        BTreeMap::new()
    }
}
