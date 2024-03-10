use crate::{
    base::data::function::Term,
    prelude::{Pointed, IO},
};

pub trait MonadIO<T>: Pointed<Pointed = T>
where
    T: Term,
{
    fn lift_io(m: IO<T>) -> Self;
}

impl<T> MonadIO<T> for IO<T>
where
    T: Term,
{
    fn lift_io(m: IO<T>) -> Self {
        m
    }
}
