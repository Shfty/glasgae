use super::function::bifunction::BifunT;

/// Data structures that can be reduced to a summary value one element at a time.
///
/// Strict left-associative folds are a good fit for space-efficient reduction,
/// while lazy right-associative folds are a good fit for corecursive iteration,
/// or for folds that short-circuit after processing an initial subsequence
/// of the structure's elements.
///
/// A more detailed description can be found in the Overview section of Data.Foldable.
///
/// For the class laws see the Laws section of Data.Foldable.
pub trait Foldable<T, U> {
    /// Right-associative fold of a structure, lazy in the accumulator.
    ///
    /// In the case of lists, foldr, when applied to a binary operator, a starting value (typically the right-identity of the operator), and a list, reduces the list using the binary operator, from right to left:
    /// ```text
    /// vec![x1, x2, ..., xn].foldr(f, z) == f(x1, f(x2,  ... (f(xn, z))...))
    /// ```
    ///
    /// Note that since the head of the resulting expression is produced by an application of the operator to the first element of the list, given an operator lazy in its right argument, foldr can produce a terminating expression from an unbounded list.
    ///
    /// For a general Foldable structure this should be semantically identical to,
    /// ```text
    /// foldr f z = foldr f z . toList
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use glasgae::{prelude::Foldable, base::grl::bool::Or};
    /// assert_eq!(vec![false, true, false].foldr(Or::or, false), true);
    /// ```
    /// ```
    /// # use glasgae::{prelude::Foldable, base::grl::bool::Or};
    /// assert_eq!(vec![].foldr(Or::or, false), false);
    /// ```
    /// ```
    /// # use glasgae::{prelude::Foldable, base::{data::list::Append, grl::bool::Or}};
    /// assert_eq!(
    ///     vec!['a', 'b', 'c', 'd']
    ///         .foldr(|c, acc| acc.append(c.to_string()), "foo".to_string()),
    ///         "foodcba".to_string()
    ///     );
    /// ```
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, z: U) -> U;
}
