//! Functors representing data structures that can be transformed
//! to structures of the same shape by performing an `Applicative`
//! (or, therefore, `Monad`) action on each element from left to right.
//!
//! A more detailed description of what same shape means, the various methods,
//! how traversals are constructed, and example advanced use-cases can be found
//! in the Overview section of Data.Traversable.
//!
//! For the class laws see the Laws section of Data.Traversable.

use crate::prelude::*;

/// Map each element of a structure to an action, evaluate these actions from left to right,
/// and collect the results.
///
/// For a version that ignores the results see traverse_.
///
/// # Note
///
/// The `A2` parameter is specific to types whose traversals
/// require a generically-specifiable intermediary functor,
/// and thus should be constrained to `()` in cases where
/// this is not necessary.
///
/// # Examples
///
/// Basic usage:
///
/// In the first two examples we show each evaluated action mapping to the output structure.
/// ```
/// # use glasgae::prelude::{Either::*, TraverseT, identity};
/// assert_eq!(vec![1,2,3,4].traverse_t(Some), Some(vec![1, 2, 3, 4]));
/// assert_eq!(vec![Right::<String, usize>(1), Right(2), Right(3), Right(4)].traverse_t(identity), Right(vec![1,2,3,4]))
/// ```
///
/// In the next examples, we show that None and Left values short circuit the created structure.
/// ```
/// # use glasgae::{prelude::{Either::*, TraverseT, identity, r#const}, base::grl::num::Odd};
/// assert_eq!(vec![1, 2, 3, 4].traverse_t(r#const(None::<usize>)), None);
/// assert_eq!(
///     vec![1,2,3,4].traverse_t(|x| {
///         if x.odd() {
///             Some(x)
///         } else {
///             None
///         }
///     }),
///     None
/// );
/// assert_eq!(
///     vec![
///         Right(1),
///         Right(2),
///         Right(3),
///         Right(4),
///         Left(0)
///     ].traverse_t(identity),
///     Left(0)
/// );
/// ```
pub trait TraverseT<A1, A2, A3>: Pointed
where
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A3;
}

pub fn traverse_t_default<This, A1, A2, A3>(this: This, f: impl FunctionT<This::Pointed, A1>) -> A3
where
    This: Functor<A1>,
    This::WithPointed: SequenceA<A2, A3>,
    A1: Term,
{
    this.fmap(f).sequence_a()
}

/// Evaluate each action in the structure from left to right, and collect the results.
///
/// For a version that ignores the results see sequenceA_.
///
/// Examples
/// Basic usage:
///
/// For the first two examples we show sequenceA fully evaluating a a structure and collecting the results.
///
/// ```
/// # use glasgae::prelude::{Either::*, SequenceA};
/// assert_eq!(vec![Some(1), Some(2), Some(3)].sequence_a(), Some(vec![1,2,3]));
/// assert_eq!(vec![Right::<String, usize>(1), Right(2), Right(3)].sequence_a(), Right(vec![1,2,3]));
/// ```
///
/// The next two examples show None and Some will short circuit the resulting structure if present in the input. For more context, check the Traversable instances for Either and Maybe.
/// ```
/// use glasgae::prelude::{Either::*, SequenceA};
/// assert_eq!(vec![Some(1), Some(2), Some(3), None].sequence_a(), None);
/// assert_eq!(vec![Right(1), Right(2), Right(3), Left(4)].sequence_a(), Left(4));
/// ```
pub trait SequenceA<A2, A3>: Pointed {
    fn sequence_a(self) -> A3;
}

pub fn sequence_a_default<This, A1, A2, A3>(this: This) -> A3
where
    This: TraverseT<A1, A2, A3, Pointed = A1>,
    A1: Term,
    A2: Term,
    A3: Term,
{
    this.traverse_t(identity)
}

/// SequenceA with additional Monad semantic
pub trait Sequence<A2, A3>: SequenceA<A2, A3> {
    /// Evaluate each monadic action in the structure from left to right, and collect the results. For a version that ignores the results see sequence_.
    ///
    /// Examples
    /// Basic usage:
    ///
    /// The first two examples are instances where the input and and output of sequence are isomorphic.
    ///  ```
    /// # use glasgae::prelude::{Either::*, Sequence};
    /// assert_eq!(
    ///     Right(vec![1, 2, 3, 4]).sequence(),
    ///     vec![
    ///         Right::<usize, usize>(1),
    ///         Right(2),
    ///         Right(3),
    ///         Right(4)
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         Right::<usize, usize>(1),
    ///         Right(2),
    ///         Right(3),
    ///         Right(4)
    ///     ].sequence(),
    ///     Right(vec![1, 2, 3, 4])
    /// );
    /// ```
    ///
    /// The following examples demonstrate short circuit behavior for sequence.
    /// ```
    /// # use glasgae::prelude::{Either::*, Sequence};
    /// assert_eq!(Left::<Vec<usize>, Vec<usize>>(vec![1,2,3,4]), Left(vec![1,2,3,4]));
    /// assert_eq!(vec![Left::<usize, usize>(0), Right(1), Right(2), Right(3), Right(4)].sequence(), Left(0))
    /// ```
    fn sequence(self) -> A3;
}

impl<T, B, C> Sequence<B, C> for T
where
    T: SequenceA<B, C>,
{
    fn sequence(self) -> C {
        self.sequence_a()
    }
}

/// TraverseT with additional Monad semantic
pub trait MapM<A, B, C>: TraverseT<A, B, C>
where
    A: Term,
{
    /// Map each element of a structure to a monadic action, evaluate these actions from left to right, and collect the results. For a version that ignores the results see mapM_.
    ///
    /// Examples
    /// mapM is literally a traverse with a type signature restricted to Monad. Its implementation may be more efficient due to additional power of Monad.
    fn map_m(self, f: impl FunctionT<Self::Pointed, A>) -> C {
        self.traverse_t(f)
    }
}

impl<T, A, B, C> MapM<A, B, C> for T
where
    T: TraverseT<A, B, C>,
    A: Term,
{
}
