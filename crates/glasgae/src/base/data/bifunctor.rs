use crate::prelude::{FunctionT, Functor, Term, WithPointed, WithPointedT};

use super::with_bipointed::{WithBipointed, WithBipointedT};

pub trait Bifunctor<A, B>: WithBipointed<A> + Functor<B>
where
    A: Term,
    B: Term,
    WithBipointedT<Self, A>: WithPointed<B>,
{
    fn first(self, f: impl FunctionT<Self::Bipointed, A>) -> Self::WithBipointed;
    fn second(self, f: impl FunctionT<Self::Pointed, B>) -> Self::WithPointed;
    fn bimap(
        self,
        fa: impl FunctionT<Self::Bipointed, A>,
        fb: impl FunctionT<Self::Pointed, B>,
    ) -> WithPointedT<WithBipointedT<Self, A>, B>;
}
