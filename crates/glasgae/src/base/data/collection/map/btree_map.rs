use std::collections::BTreeMap;

use crate::derive_iterable_map;

derive_iterable_map!(BTreeMap<K, V>, insert, Ord);

pub fn insert<K, V>(k: K, v: V, mut m: BTreeMap<K, V>) -> BTreeMap<K, V>
where
    K: Ord,
{
    m.insert(k, v);
    m
}
