use std::cmp::Ordering;

use crate::{
    base::data::{function::bifunction::BifunT, FoldMap, Foldable1},
    prelude::*,
};

impl<T> Pointed for Vec<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Vec<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = Vec<U>;
}

impl<T, U> Functor<U> for Vec<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, <Vec<U> as Pointed>::Pointed>) -> Vec<U> {
        self.into_iter().map(|t| f.to_function()(t)).collect()
    }
}

impl<T> PureA for Vec<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        vec![t]
    }
}

impl<F, A, B> AppA<Vec<A>, Vec<B>> for Vec<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Vec<A>) -> Vec<B> {
        self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
    }
}

impl<T> ReturnM for Vec<T> where T: Term {}

impl<T, U> ChainM<Vec<U>> for Vec<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<T, Vec<U>>) -> Vec<U> {
        self.into_iter().flat_map(|t| f.to_function()(t)).collect()
    }
}

impl<T, U> Foldable<U> for Vec<T>
where
    T: Term,
{
    fn foldr(mut self, f: impl BifunT<T, U, U>, init: U) -> U {
        let mut acc = init;
        while let Some(next) = self.pop() {
            acc = f.to_bifun()(next, acc);
        }
        acc
    }

    fn foldl(mut self, f: impl BifunT<U, T, U>, init: U) -> U {
        let mut acc = init;
        while !self.is_empty() {
            let next = self.remove(0);
            acc = f.to_bifun()(acc, next);
        }
        acc
    }
}

impl<T> Foldable1<T> for Vec<T>
where
    T: Term,
{
    fn foldr1(self, f: impl BifunT<T, T, T>) -> T {
        self.into_iter().reduce(|x, y| f.to_bifun()(x, y)).unwrap()
    }

    fn foldl1(self, f: impl BifunT<T, T, T>) -> T {
        self.into_iter()
            .rev()
            .reduce(|y, x| f.to_bifun()(x, y))
            .unwrap()
    }
}

impl<T, U> FoldMap<U> for Vec<T>
where
    T: Term,
    U: Monoid,
{
    fn fold_map(self, f: impl FunctionT<T, U> + Clone) -> U {
        U::mconcat(self.fmap(f))
    }
}

impl<T, A_, A1, A2> TraverseT<A1, A_, A2> for Vec<T>
where
    A1: Functor<Function<Vec<A_>, Vec<A_>>, Pointed = A_>,
    A1::WithPointed: AppA<A2, A2>,
    A_: Term,
    A2: PureA<Pointed = Vec<A_>>,
    T: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for Vec<A1>
where
    A1: Functor<Function<Vec<A_>, Vec<A_>>, Pointed = A_>,
    A1::WithPointed: AppA<A2, A2>,
    A_: Term,
    A2: PureA<Pointed = Vec<A_>>,
{
    fn sequence_a(self) -> A2 {
        self.foldr(
            |next, acc| next.fmap(|t| (|v| push(t, v)).boxed()).app_a(acc),
            PureA::pure_a(vec![]),
        )
    }
}

impl<T> Semigroup for Vec<T>
where
    T: Term,
{
    fn assoc_s(self, a: Self) -> Self {
        self.into_iter().chain(a).collect()
    }
}

impl<T> Monoid for Vec<T>
where
    T: Term,
{
    fn mempty() -> Self {
        vec![]
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.into_iter().flatten().collect()
    }
}

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
