use super::FunctionT;

pub trait Until<T>: Sized + Clone + FunctionT<T, T>
where
    T: Clone,
{
    fn until(self, p: impl FunctionT<T, bool> + Clone, t: T) -> T {
        if p.clone()(t.clone()) {
            t
        } else {
            self.clone().until(p, self(t))
        }
    }
}

impl<T, U> Until<U> for T
where
    T: Clone + FunctionT<U, U>,
    U: Clone,
{
}

