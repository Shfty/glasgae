use vector_map::set::VecSet;

use crate::derive_iterable_collection;

derive_iterable_collection!(VecSet<T>, insert, PartialEq);

pub fn insert<T>(t: T, mut m: VecSet<T>) -> VecSet<T>
where
    T: PartialEq,
{
    m.insert(t);
    m
}
