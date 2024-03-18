use std::collections::HashMap;

use crate::derive_iterable_map;

derive_iterable_map!(HashMap<K, V>, insert, Eq, std::hash::Hash);

pub fn insert<K, V>(k: K, v: V, mut m: HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    m.insert(k, v);
    m
}

