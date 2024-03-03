//! # Data

pub mod either;
pub mod function;
pub mod functor;
pub mod list;
pub mod maybe;
pub mod monoid;
pub mod pointed;
pub mod traversable;
pub mod tuple;

mod boxed;
mod foldable;
mod semigroup;

pub use boxed::*;
pub use foldable::*;
pub use semigroup::*;
