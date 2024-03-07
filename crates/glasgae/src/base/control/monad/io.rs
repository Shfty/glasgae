use crate::prelude::{IO, Pointed};

pub trait MonadIO<T>: Pointed<Pointed = T> {
    fn lift_io(m: IO<T>) -> Self;
}

impl<T> MonadIO<T> for IO<T> {
    fn lift_io(m: IO<T>) -> Self {
        m
    }
}

