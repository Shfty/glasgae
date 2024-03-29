use std::fmt::Debug;

use crate::{derive_pointed, derive_with_pointed, prelude::*};

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

derive_pointed!(RoseTree<(T)>);
derive_with_pointed!(RoseTree<(T)>);

impl<T, U> Functor<U> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    type Mapped = RoseTree<U>;

    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, U>) -> Self::Mapped {
        let RoseTree(t, children) = self;
        let f = f.to_function();
        RoseTree(
            f.clone()(t),
            children.fmap(Functor::fmap.flip_clone().curry_clone(f)),
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

impl<F, A, B> AppA<A, B> for RoseTree<F>
where
    F: Term + FunctionT<A, B>,
    Vec<RoseTree<F>>: Applicative<
        Vec<RoseTree<A>>,
        Vec<RoseTree<B>>,
        WithA = Vec<RoseTree<A>>,
        WithB = Vec<RoseTree<B>>,
    >,
    A: Term,
    B: Term,
{
    type WithA = RoseTree<A>;
    type WithB = RoseTree<B>;

    fn app_a(self, a: RoseTree<A>) -> RoseTree<B> {
        let RoseTree(f, fc) = self;
        let RoseTree(a, ac) = a;
        RoseTree(f(a), fc.app_a(ac))
    }
}

impl<T> ReturnM for RoseTree<T> where T: Term {}

impl<T, U> ChainM<U> for RoseTree<T>
where
    T: Term,
    U: Term,
{
    type Chained = RoseTree<U>;

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

impl<T, A1, AF1, A, A2, A3, AF3> TraverseT<A1, A2, A3> for RoseTree<T>
where
    T: Term,
    A1: Pointed<Pointed = A>
        + Functor<Function<Vec<RoseTree<A>>, RoseTree<A>>, Mapped = AF1>
        + WithPointed<Vec<RoseTree<A>>, WithPointed = A2>
        + WithPointed<RoseTree<A>, WithPointed = A3>,
    AF1: Applicative<Vec<RoseTree<A>>, RoseTree<A>, WithA = A2, WithB = A3>,
    A: Term,
    A2: PureA<Pointed = Vec<RoseTree<A>>>,
    A3: Pointed<Pointed = RoseTree<A>>
        + Functor<Function<Vec<RoseTree<A>>, Vec<RoseTree<A>>>, Mapped = AF3>
        + WithPointed<Vec<RoseTree<A>>, WithPointed = A2>,
    AF3: Applicative<Vec<RoseTree<A>>, Vec<RoseTree<A>>, WithA = A2, WithB = A2>,
{
    type Mapped = A1;
    type Value = A;
    type Traversed = A3;

    fn traverse_t(self, f: impl FunctionT<T, A1>) -> A3 {
        rose_tree_traverse(self, f)
    }
}

fn rose_tree_traverse<T, A1, AF1, A, A2, A3, AF3>(
    RoseTree(x, xs): RoseTree<T>,
    f: impl FunctionT<T, A1>,
) -> A3
where
    T: Term,
    A1: Pointed<Pointed = A>
        + Functor<Function<Vec<RoseTree<A>>, RoseTree<A>>, Mapped = AF1>
        + WithPointed<Vec<RoseTree<A>>, WithPointed = A2>
        + WithPointed<RoseTree<A>, WithPointed = A3>,
    AF1: Applicative<Vec<RoseTree<A>>, RoseTree<A>, WithA = A2, WithB = A3>,
    A: Term,
    A2: PureA<Pointed = Vec<RoseTree<A>>>,
    A3: Pointed<Pointed = RoseTree<A>>
        + Functor<Function<Vec<RoseTree<A>>, Vec<RoseTree<A>>>, Mapped = AF3>
        + WithPointed<Vec<RoseTree<A>>, WithPointed = A2>,
    AF3: Applicative<Vec<RoseTree<A>>, Vec<RoseTree<A>>, WithA = A2, WithB = A2>,
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
    Self: TraverseT<A1, A2, A3, Pointed = A1, Mapped = A1, Traversed = A3>,
    A1: Pointed + WithPointed<Function<RoseTree<A1>, RoseTree<PointedT<A1>>>>,
    A2: Term,
    A3: Term,
{
    type Inner = A1;
    type Value = PointedT<A1>;
    type Sequenced = A3;

    fn sequence_a(self) -> A3 {
        sequence_a_default(self)
    }
}
