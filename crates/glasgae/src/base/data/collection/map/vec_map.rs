use vector_map::*;

use crate::{
    derive_iterable_map, derive_pointed, derive_with_pointed,
};

derive_pointed!(VecMap<K: PartialEq, (V)>);
derive_with_pointed!(VecMap<K: PartialEq, (V)>);
derive_iterable_map!(VecMap<K, V>, insert, PartialEq);

pub fn insert<K, V>(k: K, v: V, mut m: VecMap<K, V>) -> VecMap<K, V>
where
    K: PartialEq,
{
    m.insert(k, v);
    m
}
