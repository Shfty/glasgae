//! # Prelude

pub use std::convert::identity;

pub use crate::{
    base::{
        control::{
            applicative::*,
            monad::{io::*, morph::*, *},
        },
        data::{
            bifoldable::*,
            bifoldable1::*,
            bifunctor::*,
            bipointed::*,
            bitraversable::*,
            boxed::*,
            collection::*,
            either::{Either::*, *},
            foldable::*,
            foldable1::*,
            function::{bifunction::*, *},
            functor::{identity::*, r#const::*, *},
            maybe::{Maybe::*, *},
            monoid::*,
            pointed::*,
            semigroup::*,
            term::*,
            traversable::*,
            with_bipointed::*,
            with_pointed::*,
        },
        grl::{io::*, Read, Show},
    },
    macros::*,
};
