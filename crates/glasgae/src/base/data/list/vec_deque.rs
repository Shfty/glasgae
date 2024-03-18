use std::collections::VecDeque;

use crate::{
    base::data::{foldl1_default, foldr1_default},
    prelude::{
        AppA, Boxed, ChainM, Fmap, FoldMap, Foldable, Foldable1, Function, FunctionT, Monoid,
        Pointed, PureA, ReturnM, SequenceA, Term, TraverseT, WithPointed,
    },
};

impl<T> Pointed for VecDeque<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for VecDeque<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = VecDeque<U>;
}

impl<T, U> Fmap<U> for VecDeque<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::WithPointed {
        self.into_iter().map(|t| f.to_function()(t)).collect()
    }
}

impl<T> PureA for VecDeque<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        FromIterator::from_iter([t])
    }
}

impl<F, A, B> AppA<VecDeque<A>, VecDeque<B>> for VecDeque<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: VecDeque<A>) -> VecDeque<B> {
        self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
    }
}

impl<T> ReturnM for VecDeque<T> where T: Term {}

impl<T, U> ChainM<VecDeque<U>> for VecDeque<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, VecDeque<U>>) -> VecDeque<U> {
        self.into_iter().flat_map(|t| f.to_function()(t)).collect()
    }
}

impl<T, U> Foldable<U> for VecDeque<T>
where
    T: Term,
    U: Term,
{
    fn foldr(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<Self::Pointed, U, U>,
        z: U,
    ) -> U {
        self.into_iter()
            .rfold(z, |acc, next| f.to_bifun()(next, acc))
    }

    fn foldl(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<U, Self::Pointed, U>,
        z: U,
    ) -> U {
        self.into_iter()
            .fold(z, |acc, next| f.to_bifun()(acc, next))
    }
}

impl<T> Foldable1<T> for VecDeque<T>
where
    T: Term,
{
    fn foldr1(self, f: impl crate::base::data::function::bifunction::BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl crate::base::data::function::bifunction::BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}

impl<T, U> FoldMap<U> for VecDeque<T>
where
    T: Term,
    U: Monoid,
{
    fn fold_map(self, f: impl FunctionT<Self::Pointed, U> + Clone) -> U {
        self.into_iter()
            .map(|t| f.to_function()(t))
            .fold(U::mempty(), |acc, next| acc.assoc_s(next))
    }
}

impl<T, A1, U, A2> TraverseT<A1, U, A2> for VecDeque<T>
where
    Self: SequenceA<U, A2>,
    T: Term,
    A1: Fmap<Function<VecDeque<U>, VecDeque<U>>, Pointed = U>,
    A1::WithPointed: AppA<A2, A2>,
    U: Term,
    A2: PureA<Pointed = VecDeque<U>>,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        self.fmap(f).sequence_a()
    }
}

impl<A1, T, A2> SequenceA<T, A2> for VecDeque<A1>
where
    A1: Fmap<Function<VecDeque<T>, VecDeque<T>>, Pointed = T>,
    A1::WithPointed: AppA<A2, A2>,
    T: Term,
    A2: PureA<Pointed = VecDeque<T>>,
{
    fn sequence_a(self) -> A2 {
        self.foldr(
            |next, acc| next.fmap(|t| (|v| push_back(t, v)).boxed()).app_a(acc),
            PureA::pure_a(Default::default()),
        )
    }
}

pub fn push_back<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_back(t);
    deque
}

pub fn push_front<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_front(t);
    deque
}

