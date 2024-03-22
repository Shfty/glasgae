use std::collections::LinkedList;

use crate::{
    derive_applicative_iterable, derive_foldable_iterable, derive_functor_iterable,
    derive_monad_iterable, derive_monoid_iterable, derive_semigroup_iterable,
    derive_traversable_iterable, derive_pointed, derive_with_pointed,
};

derive_pointed!(LinkedList<(X)>);
derive_with_pointed!(LinkedList<(X)>);
derive_functor_iterable!(LinkedList<(X)>);
derive_applicative_iterable!(LinkedList<(X)>);
derive_monad_iterable!(LinkedList<(X)>);
derive_semigroup_iterable!(LinkedList<(X)>);
derive_monoid_iterable!(LinkedList<(X)>);
derive_foldable_iterable!(LinkedList<(X)>);
derive_traversable_iterable!(LinkedList<(X)>, push_back);

pub fn push_back<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_back(t);
    list
}

pub fn push_front<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_front(t);
    list
}
