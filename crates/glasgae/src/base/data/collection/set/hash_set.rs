use std::collections::HashSet;

use crate::derive_iterable_collection;

derive_iterable_collection!(HashSet<T>, insert, Eq, std::hash::Hash);

pub fn insert<T>(t: T, mut m: HashSet<T>) -> HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    m.insert(t);
    m
}

