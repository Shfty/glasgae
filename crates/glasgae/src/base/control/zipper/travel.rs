use crate::{base::data::function::Term, prelude::FunctionT};

pub trait Travel<D, M, N>: Term
where
    M: Term,
{
    fn travel(self, tf: impl FunctionT<Self, M>) -> N;
}
