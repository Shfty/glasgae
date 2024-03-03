//! # Prelude

pub use std::convert::identity;

pub use crate::base::{
    control::{
        applicative::{AppA, LiftA2, PureA},
        monad::{ChainM, ReturnM, ThenM},
    },
    data::{
        either::Either,
        function::{r#const, App, Compose, Curry, Flip, Function, FunctionT, Until},
        functor::Functor,
        list::{Append, Filter},
        monoid::Monoid,
        pointed::{Pointed, PointedT, WithPointed, WithPointedT},
        traversable::{MapM, Sequence, SequenceA, TraverseT},
        Boxed, Foldable, Semigroup,
    },
    grl::{
        io::{
            append_file, get_char, get_contents, get_line, interact, put_char, put_str, put_str_ln, print,
            read_file, write_file, IO,
        },
        Read, Show,
    },
};
