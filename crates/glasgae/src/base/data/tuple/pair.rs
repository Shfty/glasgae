use crate::{
    base::data::{
        bifunctor::{Bifmap, Bifunctor},
        bipointed::Bipointed,
        with_bipointed::WithBipointed,
    },
    prelude::{
        foldl1_default, foldr1_default, sequence_a_default, AppA, Bifoldable, FoldMap, Foldable,
        Foldable1, Function, FunctionT, Functor, Monoid, Pointed, PointedT, PureA, SequenceA, Term,
        TraverseT, WithPointed,
    },
};

pub trait Pair<L, R>: Term {
    fn pair(l: L, r: R) -> Self;
    fn fst(self) -> L;
    fn snd(self) -> R;
}

impl<L, R> Pair<L, R> for (L, R)
where
    L: Term,
    R: Term,
{
    fn pair(l: L, r: R) -> Self {
        (l, r)
    }

    fn fst(self) -> L {
        self.0
    }

    fn snd(self) -> R {
        self.1
    }
}

impl<L, R> PureA for (L, R)
where
    L: Monoid,
    R: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        (L::mempty(), t)
    }
}

impl<M, FR, AR, BR> AppA<AR, BR> for (M, FR)
where
    M: Monoid,
    FR: Term + FunctionT<AR, BR>,
    AR: Term,
    BR: Term,
{
    type WithA = (M, AR);
    type WithB = (M, BR);

    fn app_a(self, a: (M, AR)) -> (M, BR) {
        let (u, f) = self;
        let (v, x) = a;
        (u.assoc_s(v), f(x))
    }
}

impl<L, R, R_> Foldable<R_> for (L, R)
where
    L: Term,
    R: Term,
    R_: Term,
{
    fn foldr(self, f: impl crate::prelude::BifunT<Self::Pointed, R_, R_>, z: R_) -> R_ {
        f(self.1, z)
    }

    fn foldl(self, f: impl crate::prelude::BifunT<R_, Self::Pointed, R_>, z: R_) -> R_ {
        f(z, self.1)
    }
}

impl<L, R> Foldable1<R> for (L, R)
where
    L: Term,
    R: Term,
{
    fn foldr1(self, f: impl crate::prelude::BifunT<R, R, R>) -> R {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl crate::prelude::BifunT<R, R, R>) -> R {
        foldl1_default(self, f)
    }
}

impl<L, R, R_> FoldMap<R_> for (L, R)
where
    L: Term,
    R: Term,
    R_: Monoid,
{
    fn fold_map(self, f: impl FunctionT<Self::Pointed, R_> + Clone) -> R_ {
        f(self.1)
    }
}

impl<L, R, A1, A, A2> TraverseT<A1, (), A2> for (L, R)
where
    L: Term,
    R: Term,
    A1: Functor<(L, A), Pointed = A, Mapped = A2>,
    A: Term,
    A2: Term,
{
    type Mapped = A1;
    type Value = A;
    type Traversed = A2;

    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        f(self.1).fmap(|t| (self.0, t))
    }
}

impl<L, A1, A2> SequenceA<(), A2> for (L, A1)
where
    Self: TraverseT<A1, (), A2, Pointed = A1, Mapped = A1, Traversed = A2>,
    A1: Pointed + WithPointed<Function<(L, A1), (L, PointedT<A1>)>>,
    A2: Term,
    L: Term,
{
    type Inner = A1;
    type Value = PointedT<A1>;
    type Sequenced = A2;

    fn sequence_a(self) -> A2 {
        sequence_a_default(self)
    }
}

impl<L, R> Bipointed for (L, R)
where
    L: Term,
    R: Term,
{
    type Bipointed = L;
}

impl<L, L_, R> WithBipointed<L_> for (L, R)
where
    L: Term,
    L_: Term,
    R: Term,
{
    type WithBipointed = (L_, R);
}

impl<L, L_, R> Bifmap<L_> for (L, R)
where
    L: Term,
    L_: Term,
    R: Term,
{
    fn bifmap(self, f: impl crate::prelude::FunctionT<Self::Bipointed, L_>) -> Self::WithBipointed {
        (f(self.0), self.1)
    }
}

impl<L, L_, R, R_> Bifunctor<L_, R_> for (L, R)
where
    L: Term,
    L_: Term,
    R: Term,
    R_: Term,
    (L_, R): Functor<R_, Pointed = R>,
{
    fn bimap(
        self,
        fa: impl crate::prelude::FunctionT<Self::Bipointed, L_>,
        fb: impl crate::prelude::FunctionT<Self::Pointed, R_>,
    ) -> crate::prelude::MappedT<crate::base::data::with_bipointed::WithBipointedT<Self, L_>, R_>
    {
        self.bifmap(fa).fmap(fb)
    }
}

impl<L, R, T> Bifoldable<T> for (L, R)
where
    L: Term,
    R: Term,
    T: Term,
{
    fn bifoldr(
        self,
        fa: impl crate::prelude::BifunT<Self::Bipointed, T, T>,
        fb: impl crate::prelude::BifunT<Self::Pointed, T, T>,
        z: T,
    ) -> T {
        fb(self.1, fa(self.0, z))
    }

    fn bifoldl(
        self,
        fa: impl crate::prelude::BifunT<T, Self::Bipointed, T>,
        fb: impl crate::prelude::BifunT<T, Self::Pointed, T>,
        z: T,
    ) -> T {
        fb(fa(z, self.0), self.1)
    }
}
