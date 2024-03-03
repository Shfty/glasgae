use crate::prelude::*;

impl Pointed for String {
    type Pointed = char;
}

impl Semigroup for String {
    fn assoc_s(self, a: Self) -> Self {
        self + &a
    }
}

impl Monoid for String {
    fn mempty() -> Self {
        String::default()
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.concat()
    }
}
