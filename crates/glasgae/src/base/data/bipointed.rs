use crate::prelude::{Pointed, Term};

pub trait Bipointed: Pointed {
    type Bipointed: Term;
}

pub type BipointedT<T> = <T as Bipointed>::Bipointed;
