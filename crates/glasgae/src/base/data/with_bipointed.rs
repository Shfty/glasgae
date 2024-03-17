use crate::prelude::Pointed;

use super::bipointed::Bipointed;

pub trait WithBipointed<A = <Self as Bipointed>::Bipointed>:
    Bipointed
{
    type WithBipointed: Bipointed<Bipointed = A> + Pointed<Pointed = Self::Pointed>;
}

pub type WithBipointedT<T, A> = <T as WithBipointed<A>>::WithBipointed;
