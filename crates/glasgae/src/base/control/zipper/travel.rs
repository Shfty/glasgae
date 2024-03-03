use crate::prelude::FunctionT;

pub trait Travel<D, M, N>: Sized {
    fn travel(self, tf: impl FunctionT<Self, M> + Clone) -> N;
}
