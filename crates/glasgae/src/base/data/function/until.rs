use crate::prelude::{FunctionT, Term};

pub trait Until<T>: Term + FunctionT<T, T>
where
    T: Term,
{
    fn until(self, p: impl FunctionT<T, bool>, t: T) -> T {
        let p = p.to_function();
        if p.clone()(t.clone()) {
            t
        } else {
            self.clone().until(p, self(t))
        }
    }
}

impl<T, U> Until<U> for T
where
    T: Term + FunctionT<U, U>,
    U: Term,
{
}
