use crate::prelude::Term;

use super::bipointed::Bipointed;

pub trait WithBipointed<A, B>: Bipointed {
    type WithLeft: Term;
    type WithRight: Term;
    type WithBipointed: Term;
}

