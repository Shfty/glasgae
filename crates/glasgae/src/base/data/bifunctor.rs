use crate::prelude::{Functor, FunctionT, Term, WithPointedT, MappedT};

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

pub trait Bifunctor<A, B>: Bifmap<A> + Functor<B>
where
    Self::WithBipointed: Functor<B>,
    A: Term,
    B: Term,
{
    fn bimap(
        self,
        fa: impl FunctionT<Self::Bipointed, A>,
        fb: impl FunctionT<Self::Pointed, B>,
    ) -> MappedT<WithBipointedT<Self, A>, B> {
        self.bifmap(fa).fmap(fb)
    }
}
