use std::collections::BTreeSet;

use crate::{
    base::data::{
        foldl1_default, foldr1_default, list::vec::push, traversable::traverse_t_default,
    },
    prelude::{
        AppA, ChainM, Fmap, Foldable, Foldable1, Function, FunctionT, Monoid, Pointed, PureA,
        ReturnM, Semigroup, SequenceA, Term, TraverseT, WithPointed, WithPointedT,
    },
};

impl<T> Pointed for BTreeSet<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for BTreeSet<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = BTreeSet<U>;
}

impl<T, U> Fmap<U> for BTreeSet<T>
where
    T: Term,
    U: Term + Ord,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::WithPointed {
        self.into_iter().map(|t| f.to_function()(t)).collect()
    }
}

impl<T> PureA for BTreeSet<T>
where
    T: Term + Ord,
{
    fn pure_a(t: Self::Pointed) -> Self {
        FromIterator::from_iter([t])
    }
}

impl<F, A, B> AppA<BTreeSet<A>, BTreeSet<B>> for BTreeSet<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term + Ord,
{
    fn app_a(self, a: BTreeSet<A>) -> BTreeSet<B> {
        self.into_iter().zip(a).map(|(f, a)| f(a)).collect()
    }
}

impl<T> ReturnM for BTreeSet<T> where T: Term + Ord {}

impl<T, U> ChainM<BTreeSet<U>> for BTreeSet<T>
where
    T: Term,
    U: Term + Ord,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, BTreeSet<U>>) -> BTreeSet<U> {
        self.into_iter().flat_map(|t| f.to_function()(t)).collect()
    }
}

impl<T, U> Foldable<U> for BTreeSet<T>
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

impl<T> Foldable1<T> for BTreeSet<T>
where
    T: Term + Ord,
{
    fn foldr1(self, f: impl crate::base::data::function::bifunction::BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl crate::base::data::function::bifunction::BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}

impl<T, A1, U, A2> TraverseT<A1, U, A2> for BTreeSet<T>
where
    WithPointedT<Self, A1>: SequenceA<U, A2>,
    T: Term,
    A1: Term + Ord,
    U: Term,
    A2: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<A1, U, A2> SequenceA<U, A2> for BTreeSet<A1>
where
    A1: Fmap<Function<Vec<U>, Vec<U>>, Pointed = U>,
    A1::WithPointed: AppA<A2, A2>,
    U: Term,
    A2: PureA<Pointed = Vec<U>>,
{
    fn sequence_a(self) -> A2 {
        crate::prelude::Foldable::foldr(
            self,
            |next, acc| AppA::app_a(next.fmap(|t| Box::new(|v| push(t, v))), acc),
            PureA::pure_a(Default::default()),
        )
    }
}

impl<T> Semigroup for BTreeSet<T>
where
    T: Term + Ord,
{
    fn assoc_s(self, a: Self) -> Self {
        self.into_iter().chain(a).collect()
    }
}

impl<T> Monoid for BTreeSet<T>
where
    T: Term + Ord,
{
    fn mempty() -> Self {
        BTreeSet::default()
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.into_iter().flatten().collect()
    }
}
