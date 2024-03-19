use std::fmt::Debug;

use crate::prelude::*;

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

impl<T, A1, A2, U, A3> TraverseT<A1, A2, A3> for RoseTree<T>
where
    T: Term,
    A1: Term + Fmap<Function<Vec<RoseTree<U>>, RoseTree<U>>, Pointed = U>,
    A1::WithPointed: AppA<A2, A3>,
    A2: PureA<Pointed = Vec<RoseTree<U>>>,
    U: Term,
    A3: PureA<Pointed = RoseTree<U>> + Fmap<Function<Vec<RoseTree<U>>, Vec<RoseTree<U>>>>,
    A3::WithPointed: AppA<A2, A2>,
{
    fn traverse_t(self, f: impl FunctionT<T, A1>) -> A3 {
        rose_tree_traverse(self, f)
    }
}

fn rose_tree_traverse<T, A1, A2, U, A3>(
    RoseTree(x, xs): RoseTree<T>,
    f: impl FunctionT<T, A1>,
) -> A3
where
    T: Term,
    A1: Term + Fmap<Function<Vec<RoseTree<U>>, RoseTree<U>>, Pointed = U>,
    A1::WithPointed: AppA<A2, A3>,
    A2: PureA<Pointed = Vec<RoseTree<U>>>,
    U: Term,
    A3: PureA<Pointed = RoseTree<U>> + Fmap<Function<Vec<RoseTree<U>>, Vec<RoseTree<U>>>>,
    A3::WithPointed: AppA<A2, A2>,
{
    let f = f.to_function();
    RoseTree.lift_a2()(f.clone()(x), xs.traverse_t(|t| rose_tree_traverse(t, f)))
}

#[test]
fn test_rose_tree_sequence() {
    let tree = RoseTree(
        0,
        vec![
            RoseTree(
                1,
                vec![
                    RoseTree(4, vec![RoseTree(10, vec![])]),
                    RoseTree(5, vec![RoseTree(11, vec![])]),
                ],
            ),
            RoseTree(
                2,
                vec![
                    RoseTree(6, vec![RoseTree(12, vec![])]),
                    RoseTree(7, vec![RoseTree(13, vec![])]),
                ],
            ),
            RoseTree(
                3,
                vec![
                    RoseTree(8, vec![RoseTree(14, vec![])]),
                    RoseTree(9, vec![RoseTree(15, vec![])]),
                ],
            ),
        ],
    );
    println!("Tree:\n{tree:#?}");

    let traversed: Vec<RoseTree<_>> = rose_tree_traverse(tree, |t| vec![(t + 1).show()]);
    println!("Traversed:\n{traversed:#?}");
}

impl<A1, A2, A3> SequenceA<A2, A3> for RoseTree<A1>
where
    Self: TraverseT<A1, A2, A3, Pointed = A1>,
    A1: Term,
    A2: Term,
    A3: Term,
{
    fn sequence_a(self) -> A3 {
        sequence_a_default(self)
    }
}
