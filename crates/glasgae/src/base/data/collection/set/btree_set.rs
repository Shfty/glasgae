use std::collections::BTreeSet;

use crate::{
    derive_applicative_iterable, derive_foldable_iterable, derive_functor_iterable,
    derive_monad_iterable, derive_monoid_iterable, derive_semigroup_iterable,
    derive_traversable_iterable, derive_pointed, derive_with_pointed,
};

derive_pointed!(BTreeSet<(X: Ord)>);
derive_with_pointed!(BTreeSet<(X: Ord)>);
derive_functor_iterable!(BTreeSet<(X: Ord)>);
derive_applicative_iterable!(BTreeSet<(X: Ord)>);
derive_monad_iterable!(BTreeSet<(X: Ord)>);
derive_semigroup_iterable!(BTreeSet<(X : Ord)>);
derive_monoid_iterable!(BTreeSet<(X: Ord)>);
derive_foldable_iterable!(BTreeSet<(X : Ord)>);
derive_traversable_iterable!(BTreeSet<(X: Ord)>, insert);

pub fn insert<T>(t: T, mut m: BTreeSet<T>) -> BTreeSet<T>
where
    T: Ord,
{
    m.insert(t);
    m
}
