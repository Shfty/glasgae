use std::collections::HashMap;
use std::hash::Hash;

use crate::{derive_iterable_map, derive_pointed, derive_with_pointed};

derive_pointed!(HashMap<K : Eq : Hash, (V)>);
derive_with_pointed!(HashMap<K : Eq : Hash, (V)>);
derive_iterable_map!(HashMap<K, V>, insert, Eq, Hash);

pub fn insert<K, V>(k: K, v: V, mut m: HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    m.insert(k, v);
    m
}
