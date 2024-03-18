use std::cmp::Ordering;

use crate::{base::data::function::bifunction::BifunT, impl_list, prelude::*};

impl_list!(Vec<X>, push);

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
