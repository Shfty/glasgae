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
pub trait TraverseT<A1, T, A2>: Pointed
where
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2;
}

pub fn traverse_t_default<This, A1, T, A2>(this: This, f: impl FunctionT<This::Pointed, A1>) -> A2
where
    This: Fmap<A1>,
    This::WithPointed: SequenceA<T, A2>,
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
pub trait SequenceA<A_, A2>: Pointed {
    fn sequence_a(self) -> A2;
}

pub fn sequence_a_default<This, A1, A_, A2>(this: This) -> A2
where
    This: TraverseT<A1, A_, A2, Pointed = A1>,
    A1: Term,
    A_: Term,
    A2: Term,
{
    this.traverse_t(identity)
}

/// SequenceA with additional Monad semantic
pub trait Sequence<A_, A2>: SequenceA<A_, A2> {
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
    fn sequence(self) -> A2;
}

impl<T, A_, B> Sequence<A_, B> for T
where
    T: SequenceA<A_, B>,
{
    fn sequence(self) -> B {
        self.sequence_a()
    }
}

/// TraverseT with additional Monad semantic
pub trait MapM<A, T, B>: TraverseT<A, T, B>
where
    A: Term,
{
    /// Map each element of a structure to a monadic action, evaluate these actions from left to right, and collect the results. For a version that ignores the results see mapM_.
    ///
    /// Examples
    /// mapM is literally a traverse with a type signature restricted to Monad. Its implementation may be more efficient due to additional power of Monad.
    fn map_m(self, f: impl FunctionT<Self::Pointed, A>) -> B {
        self.traverse_t(f)
    }
}

impl<T, A, U, B> MapM<A, U, B> for T
where
    T: TraverseT<A, U, B>,
    A: Term,
{
}
