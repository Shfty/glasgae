use crate::prelude::{Maybe, Maybe::*, Term};

use super::{function::bifunction::BifunT, Foldable};

pub trait Foldable1<T>: Foldable<T> {
    fn foldr1(self, _f: impl BifunT<T, T, T>) -> T;
    fn foldl1(self, f: impl BifunT<T, T, T>) -> T;
}

/// Derive foldr1 from foldr
pub fn foldr1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Foldable<Maybe<T>, Pointed = T>,
    T: Term,
{
    let f = f.to_bifun();
    match this.foldr(
        |x, m| {
            Just(match m {
                Nothing => x,
                Just(y) => f(x, y),
            })
        },
        Nothing,
    ) {
        Just(t) => t,
        Nothing => panic!("foldr1: empty structure"),
    }
}

/// Derive foldl1 from foldl
pub fn foldl1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Foldable<Maybe<T>, Pointed = T>,
    T: Term,
{
    let f = f.to_bifun();
    match this.foldl(
        |m, y| {
            Just(match m {
                Nothing => y,
                Just(x) => f(x, y),
            })
        },
        Nothing,
    ) {
        Just(t) => t,
        Nothing => panic!("foldl1: empty structure"),
    }
}
