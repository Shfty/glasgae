use crate::prelude::{FunctionT, Term};

pub trait Filter<T>: Term
where
    T: Term,
{
    fn filter(self, p: impl FunctionT<T, bool>) -> Self;
}

impl<T> Filter<T> for Vec<T>
where
    T: Term,
{
    fn filter(self, p: impl FunctionT<T, bool>) -> Self {
        self.into_iter().fold(vec![], |mut acc, next| {
            if p.to_function()(next.clone()) {
                acc.push(next);
                acc
            } else {
                acc
            }
        })
    }
}
