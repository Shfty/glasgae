use crate::prelude::{Fmap, FunctionT, Term, WithPointed, WithPointedT};

use super::with_bipointed::{WithBipointed, WithBipointedT};

pub trait Bifmap<T>: WithBipointed<T>
where
    T: Term,
{
    fn bifmap(self, f: impl FunctionT<Self::Bipointed, T>) -> Self::WithBipointed;

    fn bireplace(self, t: T) -> Self::WithBipointed
    where
        T: 'static,
    {
        self.bifmap(|_| t)
    }
}

pub trait Bifunctor<A, B>: Bifmap<A> + Fmap<B>
where
    A: Term,
    B: Term,
    WithBipointedT<Self, A>: WithPointed<B>,
{
    fn bimap(
        self,
        fa: impl FunctionT<Self::Bipointed, A>,
        fb: impl FunctionT<Self::Pointed, B>,
    ) -> WithPointedT<WithBipointedT<Self, A>, B>;
}
