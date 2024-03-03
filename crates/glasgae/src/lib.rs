//! # Glasgae
//!
//! Comprehensive functional programming library for Rust, derived from Haskell.
//!
//! * Aims to fill the gaps that exist between the Rust and Haskell standard libraries.
//!   * Functionality only reimplemented if semantically necessary
//!
//! * Documentation derived from Haskell stdlib, with Rust-specific amendments where needed.
//!
//! * Unless noted otherwise, it's safe to assume that the methods
//!   herein are implemented in terms of strict evaluation,
//!   given that Rust is a strictly-evaluated language.
//!   * Functionality specific to lazy evaluation has been omitted out of necessity,
//!     but may be reintroduced if an appropriate implementation
//!     (i.e. infinite lists via iterators) presents itself
//! 
//! # Quick Start
//!   * [`base::data::function`] for function type conventions.
//!   * [`base::data`] for functional datatypes.
//!   * [`base::control`] for machinery to orchestrate said data.
//!
//! # Further Reading
//!   * [`transformers`] for monad transformers.
//!   * [`mtl`] for generalized transformer interface.

pub mod base;
pub mod mtl;
pub mod transformers;

pub mod prelude;

// Prototyping
pub mod zipper_cont;
