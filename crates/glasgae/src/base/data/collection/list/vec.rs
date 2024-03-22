use std::cmp::Ordering;

use crate::{
    base::data::function::bifunction::BifunT, derive_applicative_iterable,
    derive_foldable_iterable, derive_functor_iterable, derive_monad_iterable,
    derive_monoid_iterable, derive_pointed, derive_semigroup_iterable, derive_traversable_iterable,
    derive_with_pointed, prelude::*,
};

derive_pointed!(Vec<(X)>);
derive_with_pointed!(Vec<(X)>);
derive_functor_iterable!(Vec<(X)>);
derive_applicative_iterable!(Vec<(X)>);
derive_monad_iterable!(Vec<(X)>);
derive_semigroup_iterable!(Vec<(X)>);
derive_monoid_iterable!(Vec<(X)>);
derive_foldable_iterable!(Vec<(X)>);
derive_traversable_iterable!(Vec<(X)>, push);

pub fn push<T>(t: T, mut v: Vec<T>) -> Vec<T> {
    v.insert(0, t);
    v
}

pub trait Sort<T> {
    fn sort(self) -> Self;
}

impl<T> Sort<T> for Vec<T>
where
    T: Term + Ord,
{
    fn sort(mut self) -> Self {
        <[T]>::sort(&mut self);
        self
    }
}

pub trait SortBy<T> {
    fn sort_by(self, f: impl BifunT<T, T, Ordering>) -> Self;
}

impl<T> SortBy<T> for Vec<T>
where
    T: Term,
{
    fn sort_by(mut self, f: impl BifunT<T, T, Ordering>) -> Vec<T> {
        <[T]>::sort_by(&mut self, |t, u| f.to_bifun()(t.clone(), u.clone()));
        self
    }
}
