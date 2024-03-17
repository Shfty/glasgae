use crate::prelude::Term;

pub trait Bipointed: Term {
    type Left: Term;
    type Right: Term;
}

