use crate::prelude::Term;

/// Abstraction over a single free type parameter
pub trait Kinded: Term {
    type Kinded: Term;
}

