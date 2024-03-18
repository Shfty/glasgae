//! # Data

pub mod tree;

pub mod bifoldable;
pub mod bifoldable1;
pub mod bifunctor;
pub mod bipointed;
pub mod bitraversable;
pub mod with_bipointed;

pub mod either;
pub mod function;
pub mod functor;
pub mod list;
pub mod map;
pub mod set;
pub mod maybe;
pub mod monoid;
pub mod pointed;
pub mod term;
pub mod traversable;
pub mod tuple;

mod boxed;
mod foldable;
mod foldable1;
mod semigroup;

pub use bifoldable::*;
pub use bifoldable1::*;
pub use bitraversable::*;
pub use boxed::*;
pub use foldable::*;
pub use foldable1::*;
pub use semigroup::*;
