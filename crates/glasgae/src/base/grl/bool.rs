//! Generalized boolean operations.

use crate::base::data::term::Term;

pub trait Or: Term {
    fn or(self, other: Self) -> Self;
}

impl Or for bool {
    fn or(self, other: Self) -> Self {
        self | other
    }
}

pub trait And: Term {
    fn or(self, other: Self) -> Self;
}

impl And for bool {
    fn or(self, other: Self) -> Self {
        self & other
    }
}

