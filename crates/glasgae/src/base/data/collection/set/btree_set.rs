use std::collections::BTreeSet;

use crate::derive_iterable_collection;

derive_iterable_collection!(BTreeSet<T>, insert, Ord);

pub fn insert<T>(t: T, mut m: BTreeSet<T>) -> BTreeSet<T>
where
    T: Ord,
{
    m.insert(t);
    m
}
