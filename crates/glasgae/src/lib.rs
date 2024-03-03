//! # Glasgae
//!
//! Comprehensive functional programming library for Rust, derived from Haskell.
//!
//! Aims to fill the gaps that exist between the Rust and Haskell standard libraries.
//!
//! * Functionality only reimplemented if semantically necessary.
//!
//! Documentation has been derived from the respective Haskell libraries,
//! with Rust-specific amendments where necessary.
//!
//! Unless noted otherwise, it's safe to assume that the methods
//! herein are implemented in terms of strict evaluation,
//! given that Rust is a strictly-evaluated language.
//!
//! Functionality specific to lazy evaluation has been omitted out of necessity,
//! but may be reintroduced in the future if an appropriate implementation
//! (i.e. infinite lists via iterators) presents itself.
//! 
//! # Quick Start
//!
//! First, consult [`base::data::function`] to learn about `glasgae`'s function type conventions.
//!
//! Then, peruse [`base::data`] to discover the provided functional data types.
//!
//! Continue to [`base::control`] for machinery to orchestrate said data.
//!
//! And finally, [`base::grl::io::IO`] ties everything together by providing
//! a pure functional interface to the outside world,
//! and means for `main` to run a pure functional program.
//!
//! # Further Reading
//!
//! [`transformers`] contains various monad transformers
//! which can be used to build complex functional programs
//! from stacks of granular, modular functionality.
//!
//! [`mtl`] contains interfaces generalizing over monad transformers,
//! smoothing over the process of composing large monad stacks.

pub mod base;
pub mod mtl;
pub mod transformers;

pub mod prelude;

// Prototyping
pub mod zipper_cont;
