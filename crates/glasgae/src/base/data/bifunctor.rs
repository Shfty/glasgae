use crate::prelude::{FunctionT, Functor, Term};

use super::{bipointed::Bipointed, with_bipointed::WithBipointed};

// TODO: Split into First / Second / Bimap traits to avoid type inference issues
pub trait Bifunctor<A, B>: Bipointed + WithBipointed<A, B> + Functor<B>
where
    A: Term,
    B: Term,
{
    fn first(self, f: impl FunctionT<Self::Bipointed, A>) -> Self::WithLeft;
    fn second(self, f: impl FunctionT<Self::Pointed, B>) -> Self::WithRight;
    fn bimap(
        self,
        fa: impl FunctionT<Self::Bipointed, A>,
        fb: impl FunctionT<Self::Pointed, B>,
    ) -> Self::WithBipointed;
}
