//! # Prelude

pub use std::convert::identity;

pub use crate::base::{
    control::{
        applicative::{AppA, LiftA2, PureA},
        monad::{ChainM, ReturnM, ThenM},
    },
    data::{
        either::{Either, Either::*},
        function::{r#const, App, Compose, Curry, Flip, Function, FunctionT, Until},
        functor::Functor,
        list::{Append, Filter},
        maybe::{Maybe, Maybe::*},
        monoid::Monoid,
        pointed::{Pointed, PointedT, WithPointed, WithPointedT},
        traversable::{MapM, Sequence, SequenceA, TraverseT},
        Boxed, Foldr, Semigroup,
    },
    grl::{
        io::{
            append_file, get_char, get_contents, get_line, interact, print, put_char, put_str,
            put_str_ln, read_file, try_append_file, try_get_char, try_get_contents, try_get_line,
            try_read_file, try_write_file, write_file, IO,
        },
        Read, Show,
    },
};
