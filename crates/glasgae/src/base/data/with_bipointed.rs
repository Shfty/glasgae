use crate::prelude::Pointed;

use super::bipointed::Bipointed;

pub trait WithBipointed<A = <Self as Bipointed>::Bipointed, B = <Self as Pointed>::Pointed>:
    Bipointed
{
    type WithLeft: Bipointed<Bipointed = A> + Pointed<Pointed = Self::Pointed>;
    type WithRight: Bipointed<Bipointed = Self::Bipointed> + Pointed<Pointed = B>;
    type WithBipointed: Bipointed<Bipointed = A> + Pointed<Pointed = B>;
}
