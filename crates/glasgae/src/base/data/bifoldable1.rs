use crate::prelude::{Maybe, Maybe::*, Term};

use super::{bifoldable::Bifoldable, function::bifunction::BifunT};

pub trait Bifoldable1<T>: Bifoldable<T> {
    fn bifoldr1(self, _f: impl BifunT<T, T, T>) -> T;
    fn bifoldl1(self, f: impl BifunT<T, T, T>) -> T;
}

/// Derive foldr1 from foldr
pub fn bifoldr1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Bifoldable<Maybe<T>, Bipointed = T, Pointed = T>,
    T: Term,
{
    let f = f.to_bifun();
    match this.bifoldr(
        {
            let f = f.clone();
            move |x, m| {
                Just(match m {
                    Nothing => x,
                    Just(y) => f(x, y),
                })
            }
        },
        move |x, m| {
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
pub fn bifoldl1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Bifoldable<Maybe<T>, Bipointed = T, Pointed = T>,
    T: Term,
{
    let f = f.to_bifun();
    match this.bifoldl(
        {
            let f = f.clone();
            |m, y| {
                Just(match m {
                    Nothing => y,
                    Just(x) => f(x, y),
                })
            }
        },
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
