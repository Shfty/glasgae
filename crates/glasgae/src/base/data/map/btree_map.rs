use std::collections::BTreeMap;

use crate::{
    base::data::{function::bifunction::BifunT, term::Term, FoldMap},
    prelude::{
        Foldr, FunctionT, Functor, Monoid, Pointed, Semigroup, SequenceA, TraverseT, WithPointed,
    },
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

impl<K, V, V_> Functor<V_> for BTreeMap<K, V>
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

impl<K, V, V_> FoldMap<V, V_> for BTreeMap<K, V>
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

impl<K, V, V_> Foldr<V, V_> for BTreeMap<K, V>
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
}

impl<K, V, A1, T, A2> TraverseT<A1, T, A2> for BTreeMap<K, V>
where
    K: Term,
    V: Term,
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        todo!()
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
