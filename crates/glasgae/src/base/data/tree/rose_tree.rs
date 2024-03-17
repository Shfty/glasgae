use std::fmt::Debug;

use crate::{
    base::data::{foldl1_default, foldr1_default, function::bifunction::BifunT, list::vec::push, traversable::traverse_t_default},
    prelude::{
        AppA, Boxed, ChainM, Curry, Flip, Fmap, Foldable, Foldable1, Function, FunctionT, Pointed,
        PureA, ReturnM, Semigroup, SequenceA, Show, Term, TraverseT, WithPointed,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoseTree<T>(pub T, pub Vec<Self>);

impl<T> Show for RoseTree<T>
where
    T: Debug,
{
    fn show(self) -> String {
        format!("{self:#?}")
    }
}

impl<T> Pointed for RoseTree<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = RoseTree<U>;
}

impl<T, U> Fmap<U> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::WithPointed {
        let RoseTree(t, children) = self;
        let f = f.to_function();
        RoseTree(
            f.clone()(t),
            children.fmap(Fmap::fmap.flip_clone().curry_clone(f)),
        )
    }
}

impl<T> PureA for RoseTree<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        RoseTree(t, vec![])
    }
}

impl<F, A, B> AppA<RoseTree<A>, RoseTree<B>> for RoseTree<F>
where
    F: Term + FunctionT<A, B>,
    Vec<RoseTree<F>>: AppA<Vec<RoseTree<A>>, Vec<RoseTree<B>>>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: RoseTree<A>) -> RoseTree<B> {
        let RoseTree(f, fc) = self;
        let RoseTree(a, ac) = a;
        RoseTree(f(a), fc.app_a(ac))
    }
}

impl<T> ReturnM for RoseTree<T> where T: Term {}

impl<T, U> ChainM<RoseTree<U>> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, RoseTree<U>>) -> RoseTree<U> {
        let f = f.to_function();
        let RoseTree(x, branches) = self;
        let RoseTree(y, branches_) = f.clone()(x);
        RoseTree(y, branches_.assoc_s(branches.fmap(|t| t.chain_m(f))))
    }
}

impl<T, U> Foldable<U> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    fn foldr(self, f: impl BifunT<Self::Pointed, U, U>, z: U) -> U {
        let f = f.to_bifun();
        let RoseTree(x, xs) = self;
        f.clone()(x, xs.foldr(|acc, next| acc.foldr(f, next), z))
    }

    fn foldl(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<U, Self::Pointed, U>,
        z: U,
    ) -> U {
        let f = f.to_bifun();
        let RoseTree(x, xs) = self;
        f.clone()(xs.foldl(|next, acc| acc.foldl(f, next), z), x)
    }
}

impl<T> Foldable1<T> for RoseTree<T>
where
    T: Term,
{
    fn foldr1(self, f: impl BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}

impl<T, A1, U, A2> TraverseT<A1, U, A2> for RoseTree<T>
where
    RoseTree<A1>: SequenceA<U, A2>,
    T: Term,
    A1: Term,
    U: Term,
    A2: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<A1, U, A2> SequenceA<U, A2> for RoseTree<A1>
where
    A1: Fmap<Function<Vec<U>, Vec<U>>, Pointed = U>,
    A1::WithPointed: AppA<A2, A2>,
    U: Term,
    A2: PureA<Pointed = Vec<U>>,
{
    fn sequence_a(self) -> A2 {
        self.foldr(
            |next, acc| next.fmap(|t| (|v| push(t, v)).boxed()).app_a(acc),
            PureA::pure_a(vec![]),
        )
    }
}
