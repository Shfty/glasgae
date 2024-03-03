use crate::prelude::FunctionT;

pub trait Filter<T> {
    fn filter(self, p: impl FunctionT<T, bool> + Clone) -> Self;
}

impl<T> Filter<T> for Vec<T>
where
    T: Clone,
{
    fn filter(self, p: impl FunctionT<T, bool> + Clone) -> Self {
        self.into_iter().fold(vec![], |mut acc, next| {
            if p.clone()(next.clone()) {
                acc.push(next);
                acc
            } else {
                acc
            }
        })
    }
}

