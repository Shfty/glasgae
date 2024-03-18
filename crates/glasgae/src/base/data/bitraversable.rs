use crate::prelude::*;

use super::bipointed::Bipointed;

pub trait BitraverseT<AA, AB, AO>: Bipointed
where
    AA: Term,
    AB: Term,
{
    fn bitraverse_t(
        self,
        fa: impl FunctionT<Self::Bipointed, AA>,
        fb: impl FunctionT<Self::Pointed, AB>,
    ) -> AO;
}

pub trait BisequenceA<AO>: Pointed {
    fn bisequence_a(self) -> AO;
}

pub trait Bisequence<AO>: BisequenceA<AO> {
    fn bisequence(self) -> AO;
}

impl<T, AO> Bisequence<AO> for T
where
    T: BisequenceA<AO>,
{
    fn bisequence(self) -> AO {
        self.bisequence_a()
    }
}

pub trait BimapM<AA, AB, AO>: BitraverseT<AA, AB, AO>
where
    AA: Term,
    AB: Term,
{
    fn bimap_m(
        self,
        fa: impl FunctionT<Self::Bipointed, AA>,
        fb: impl FunctionT<Self::Pointed, AB>,
    ) -> AO {
        self.bitraverse_t(fa, fb)
    }
}

impl<T, AA, AB, AO> BimapM<AA, AB, AO> for T
where
    T: BitraverseT<AA, AB, AO>,
    AA: Term,
    AB: Term,
{
}
