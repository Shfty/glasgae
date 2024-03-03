//! Generalized boolean operations.

pub trait Or {
    fn or(self, other: Self) -> Self;
}

impl Or for bool {
    fn or(self, other: Self) -> Self {
        self | other
    }
}

pub trait And {
    fn or(self, other: Self) -> Self;
}

impl And for bool {
    fn or(self, other: Self) -> Self {
        self & other
    }
}

