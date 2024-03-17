use crate::{
    base::data::{
        bifunctor::{Bifmap, Bifunctor},
        bipointed::Bipointed,
        with_bipointed::WithBipointed,
    },
    prelude::{AppA, Fmap, FunctionT, Monoid, PureA, Term},
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

impl<M, FR, AR, BR> AppA<(M, AR), (M, BR)> for (M, FR)
where
    M: Monoid,
    FR: Term + FunctionT<AR, BR>,
    AR: Term,
    BR: Term,
{
    fn app_a(self, a: (M, AR)) -> (M, BR) {
        let (u, f) = self;
        let (v, x) = a;
        (u.assoc_s(v), f(x))
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
    (L_, R): Fmap<R_, Pointed = R>,
{
    fn bimap(
        self,
        fa: impl crate::prelude::FunctionT<Self::Bipointed, L_>,
        fb: impl crate::prelude::FunctionT<Self::Pointed, R_>,
    ) -> crate::prelude::WithPointedT<crate::base::data::with_bipointed::WithBipointedT<Self, L_>, R_>
    {
        self.bifmap(fa).fmap(fb)
    }
}
