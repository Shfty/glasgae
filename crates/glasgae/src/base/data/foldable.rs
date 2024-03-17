//! Data structures that can be reduced to a summary value one element at a time.
//!
//! Strict left-associative folds are a good fit for space-efficient reduction,
//! while lazy right-associative folds are a good fit for corecursive iteration,
//! or for folds that short-circuit after processing an initial subsequence
//! of the structure's elements.
//!
//! A more detailed description can be found in the Overview section of Data.Foldable.
//!
//! For the class laws see the Laws section of Data.Foldable.

use crate::prelude::{Compose, Function, FunctionT, Maybe, Maybe::*, Monoid, Term};

use super::{
    function::{bifunction::BifunT, Curried},
    monoid::Endo,
};

pub trait Foldable<T, U>: Term {
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
    /// # use glasgae::{prelude::Foldr, base::grl::bool::Or};
    /// assert_eq!(vec![false, true, false].foldr(Or::or, false), true);
    /// ```
    /// ```
    /// # use glasgae::{prelude::Foldr, base::grl::bool::Or};
    /// assert_eq!(vec![].foldr(Or::or, false), false);
    /// ```
    /// ```
    /// # use glasgae::{prelude::Foldr, base::{data::list::Append, grl::bool::Or}};
    /// assert_eq!(
    ///     vec!['a', 'b', 'c', 'd']
    ///         .foldr(|c, acc| acc.append(c.to_string()), "foo".to_string()),
    ///         "foodcba".to_string()
    ///     );
    /// ```
    fn foldr(self, f: impl BifunT<T, U, U>, z: U) -> U;

    /// Left-associative fold of a structure, lazy in the accumulator. This is rarely what you want, but can work well for structures with efficient right-to-left sequencing and an operator that is lazy in its left argument.
    ///
    /// In the case of lists, foldl, when applied to a binary operator, a starting value (typically the left-identity of the operator), and a list, reduces the list using the binary operator, from left to right:
    ///
    /// foldl f z [x1, x2, ..., xn] == (...((z `f` x1) `f` x2) `f`...) `f` xn
    /// Note that to produce the outermost application of the operator the entire input list must be traversed. Like all left-associative folds, foldl will diverge if given an infinite list.
    ///
    /// If you want an efficient strict left-fold, you probably want to use foldl' instead of foldl. The reason for this is that the latter does not force the inner results (e.g. z `f` x1 in the above example) before applying them to the operator (e.g. to (`f` x2)). This results in a thunk chain O(n) elements long, which then must be evaluated from the outside-in.
    ///
    /// For a general Foldable structure this should be semantically identical to:
    ///
    /// foldl f z = foldl f z . toList
    /// Examples
    /// The first example is a strict fold, which in practice is best performed with foldl'.
    ///
    /// >>> foldl (+) 42 [1,2,3,4]
    /// 52
    /// Though the result below is lazy, the input is reversed before prepending it to the initial accumulator, so corecursion begins only after traversing the entire input string.
    ///
    /// >>> foldl (\acc c -> c : acc) "abcd" "efgh"
    /// "hgfeabcd"
    /// A left fold of a structure that is infinite on the right cannot terminate, even when for any finite input the fold just returns the initial accumulator:
    ///
    /// >>> foldl (\a _ -> a) 0 $ repeat 1
    /// * Hangs forever *
    /// WARNING: When it comes to lists, you always want to use either foldl' or foldr instead.
    fn foldl(self, f: impl BifunT<U, T, U>, z: U) -> U;
}

pub trait Foldable1<T>: Foldable<T, T> {
    fn foldr1(self, _f: impl BifunT<T, T, T>) -> T;
    fn foldl1(self, f: impl BifunT<T, T, T>) -> T;
}

/// Derive foldr from FoldMap
pub fn foldr_default<This, T, U>(this: This, f: impl BifunT<T, U, U>, z: U) -> U
where
    This: FoldMap<T, Endo<Function<U, U>>>,
    Endo<U>: Monoid,
    T: Term,
    U: Term,
{
    this.fold_map(f.to_bifun().curried().compose_clone(Endo::new))
        .app()(z)
}

/// Derive foldr1 from foldr
pub fn foldr1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Foldable<T, Maybe<T>>,
    T: Term,
{
    let f = f.to_bifun();
    match this.foldr(
        |x, m| {
            Just(match m {
                Nothing => x,
                Just(y) => f(x, y),
            })
        },
        Nothing,
    ) {
        Just(t) => t,
        Nothing => panic!("foldr1: empty structure"),
    }
}

/// Derive foldl from FoldMap
pub fn foldl_default<This, T, U>(this: This, f: impl BifunT<U, T, U>, z: U) -> U
where
    This: FoldMap<T, Endo<Function<U, U>>>,
    Endo<U>: Monoid,
    T: Term,
    U: Term,
{
    /*
     foldl f z t = appEndo (getDual (foldMap (Dual . Endo . flip f) t)) z
    */

    todo!()
}

/// Derive foldl1 from foldl
pub fn foldl1_default<This, T>(this: This, f: impl BifunT<T, T, T>) -> T
where
    This: Foldable<T, Maybe<T>>,
    T: Term,
{
    let f = f.to_bifun();
    match this.foldl(
        |m, y| {
            Just(match m {
                Nothing => y,
                Just(x) => f(x, y),
            })
        },
        Nothing,
    ) {
        Just(t) => t,
        Nothing => panic!("foldl1: empty structure"),
    }
}

pub trait FoldMap<T, U>: Term
where
    T: Term,
    U: Monoid,
{
    fn fold_map(self, f: impl FunctionT<T, U> + Clone) -> U;
}

/// Derive fold_map from Foldr
pub fn fold_map_default<This, T, U>(this: This, f: impl FunctionT<T, U>) -> U
where
    This: Foldable<T, U>,
    T: Term,
    U: Monoid,
{
    let f = f.to_function();
    this.foldr(|next, acc| f(next).assoc_s(acc), Monoid::mempty())
}
