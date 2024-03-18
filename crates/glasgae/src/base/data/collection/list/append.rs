use crate::prelude::Term;

pub trait Append: Term {
    fn append(self, t: Self) -> Self;
}

impl<T> Append for Vec<T>
where
    T: Term,
{
    fn append(self, t: Self) -> Self {
        self.into_iter().chain(t).collect()
    }
}

impl Append for String {
    fn append(self, t: Self) -> Self {
        self + &t
    }
}
