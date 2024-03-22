use std::collections::HashSet;
use std::hash::Hash;

use crate::{
    derive_applicative_iterable, derive_foldable_iterable, derive_functor_iterable,
    derive_monad_iterable, derive_monoid_iterable, derive_semigroup_iterable,
    derive_traversable_iterable, derive_pointed, derive_with_pointed,
};

derive_pointed!(HashSet<(X : Eq : Hash)>);
derive_with_pointed!(HashSet<(X : Eq : Hash)>);
derive_functor_iterable!(HashSet<(X : Eq : Hash)>);
derive_applicative_iterable!(HashSet<(X : Eq : Hash)>);
derive_monad_iterable!(HashSet<(X : Eq : Hash)>);
derive_semigroup_iterable!(HashSet<(X : Eq : Hash)>);
derive_monoid_iterable!(HashSet<(X : Eq : Hash)>);
derive_foldable_iterable!(HashSet<(X : Eq : Hash)>);
derive_traversable_iterable!(HashSet<(X : Eq : Hash)>, insert);

pub fn insert<T>(t: T, mut m: HashSet<T>) -> HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    m.insert(t);
    m
}
