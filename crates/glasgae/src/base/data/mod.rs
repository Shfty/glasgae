//! # Data

pub mod bipointed;
pub mod with_bipointed;

pub mod bifunctor;
pub mod either;
pub mod function;
pub mod functor;
pub mod list;
pub mod map;
pub mod maybe;
pub mod monoid;
pub mod pointed;
pub mod term;
pub mod traversable;
pub mod tuple;

mod boxed;
mod foldable;
mod semigroup;

pub use boxed::*;
pub use foldable::*;
pub use semigroup::*;
