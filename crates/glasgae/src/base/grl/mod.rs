//! Equivalent to Haskell's `Base.GHC` module.

pub mod lazy;
pub mod bool;
pub mod r#do;
pub mod io;
pub mod num;

mod read;
pub use read::*;

mod show;
pub use show::*;
