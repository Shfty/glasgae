use crate::{
    base::data::{bifunctor::Bifunctor, bipointed::Bipointed, with_bipointed::WithBipointed},
    prelude::Term,
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

impl<L, L_, R, R_> Bifunctor<L_, R_> for (L, R)
where
    L: Term,
    L_: Term,
    R: Term,
    R_: Term,
{
    fn first(self, f: impl crate::prelude::FunctionT<Self::Bipointed, L_>) -> Self::WithBipointed {
        (f(self.0), self.1)
    }

    fn second(self, f: impl crate::prelude::FunctionT<Self::Pointed, R_>) -> Self::WithPointed {
        (self.0, f(self.1))
    }

    fn bimap(
        self,
        fa: impl crate::prelude::FunctionT<Self::Bipointed, L_>,
        fb: impl crate::prelude::FunctionT<Self::Pointed, R_>,
    ) -> crate::prelude::WithPointedT<crate::base::data::with_bipointed::WithBipointedT<Self, L_>, R_>
    {
        (fa(self.0), fb(self.1))
    }
}

