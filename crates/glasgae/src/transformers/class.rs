//! The class of monad transformers.
//!
//! A monad transformer makes a new monad out of an existing monad,
//! such that computations of the old monad may be embedded in the new one.
//! To construct a monad with a desired set of features,
//! one typically starts with a base monad,
//! such as Identity or Vec<_>, and applies a sequence of monad transformers.

/// The class of monad transformers.
/// Instances should satisfy the following laws,
/// which state that lift is a monad transformation:
///
/// lift . return = return
/// lift (m >>= f) = lift m >>= (lift . f)
pub trait MonadTrans<MI> {
    /// Lift a computation from the argument monad to the constructed monad.
    fn lift(m: MI) -> Self;
}
