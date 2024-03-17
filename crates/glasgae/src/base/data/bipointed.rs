use crate::prelude::{Pointed, Term};

pub trait Bipointed: Pointed {
    type Bipointed: Term;
}
