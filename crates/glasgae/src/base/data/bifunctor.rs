use crate::prelude::{Either, FunctionT, Functor, Term};

use super::{bipointed::Bipointed, with_bipointed::WithBipointed};

pub trait Bifunctor<A, B>: Bipointed + WithBipointed<A, B> + Functor<B>
where
    A: Term,
    B: Term,
{
    fn first(self, f: impl FunctionT<Self::Left, A>) -> Self::WithLeft;
    fn second(self, f: impl FunctionT<Self::Right, B>) -> Self::WithRight;
    fn bimap(
        self,
        fa: impl FunctionT<Self::Left, A>,
        fb: impl FunctionT<Self::Right, B>,
    ) -> Self::WithBipointed;
}

