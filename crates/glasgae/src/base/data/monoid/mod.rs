//! A [`Monoid`] is a [`Semigroup`] with the added requirement of a neutral element.
//!
//! Thus any [`Monoid`] is a [`Semigroup`], but not the other way around.
//!
//! A type a is a [`Monoid`] if it provides an associative function [`assoc_s`](Semigroup::assoc_s)
//! that lets you combine any two values of type a into one,
//! and a neutral element [`mempty`](Monoid::mempty) such that
//!
//! ```text
//! a.assoc_s(Monoid::mempty()) == Monoid::mempty().assoc_s(a) == a
//! ```
//!
//!
//!
//! ## Examples
//!
//! The [`Sum`] monoid is defined by the numerical addition operator and `0` as neutral element:
//!
//! ```
//! # use glasgae::{prelude::Monoid, base::data::monoid::Sum};
//! assert_eq!(
//!     Sum::<usize>::mempty(),
//!     Sum(0)
//! );
//! ```
//! ```
//! # use glasgae::{prelude::Semigroup, base::data::monoid::Sum};
//! assert_eq!(
//!     Sum(1)
//!         .assoc_s(Sum(2))
//!         .assoc_s(Sum(3))
//!         .assoc_s(Sum(4)),
//!     Sum(10)
//! );
//! ```
//!
//! We can combine multiple values in a list into a single value using the
//! [`mconcat`](Monoid::mconcat) function.
//!
//! Note that we have to specify the type here
//! since [`usize`] is a monoid under several different operations:
//! ```
//! # use glasgae::{prelude::{Functor, Monoid}, base::data::monoid::Sum};
//! assert_eq!(
//!     Sum::<usize>::mconcat(vec![1,2,3,4].fmap(Sum)),
//!     Sum(10)
//! );
//! ```
//! ```
//! # use glasgae::{prelude::Monoid, base::data::monoid::Sum};
//! assert_eq!(
//!     Sum::<usize>::mconcat(vec![]),
//!     Sum(0)
//! );
//! ```
//!
//! Another valid monoid instance of usize is [`Product`].
//!
//! It is defined by multiplication and `1` as neutral element:
//!
//! ```
//! # use glasgae::{prelude::Semigroup, base::data::monoid::Product};
//! assert_eq!(
//!     Product(1)
//!         .assoc_s(Product(2))
//!         .assoc_s(Product(3))
//!         .assoc_s(Product(4)),
//!     Product(24)
//! );
//! ```
//! ```
//! # use glasgae::{prelude::{Functor, Monoid}, base::data::monoid::Product};
//! assert_eq!(
//!     Product::<usize>::mconcat(vec![1,2,3,4].fmap(Product)),
//!     Product(24)
//! );
//! ```
//! ```
//! # use glasgae::{prelude::Monoid, base::data::monoid::Product};
//! assert_eq!(
//!     Product::<usize>::mconcat(vec![]),
//!     Product(1)
//! );
//! ```

mod endo;
mod product;
mod sum;

pub use endo::*;
pub use product::*;
pub use sum::*;

use super::{foldable::Foldr, semigroup::Semigroup};

/// The class of monoids (types with an associative binary operation that has an identity).
///
/// Instances should satisfy the following:
///
/// **Right identity**
/// ```text
/// x.assoc_s(Monoid::mempty()) == x
/// ```
///
/// **Left identity**
/// ```text
/// Monoid::mempty().assoc_s(x) == x
/// ```
///
/// **Associativity**
/// ```text
/// x.assoc_s(y.assoc_s(z)) == x.assoc_s(y).assoc_s(z)
/// ```
/// (Semigroup law)
///
/// **Concatenation**
/// ```text
/// Monoid::mconcat == foldr.assoc_s(mempty)
/// ```
/// You can alternatively define mconcat instead of mempty, in which case the laws are:
///
/// **Unit**
/// ```text
/// PureA::pure_a(x).mconcat() == x
/// ```
///
/// **Multiplication**
/// ```text
/// xss.join().mconcat() = xss.fmap(mconcat).mconcat()
/// ```
///
/// **Subclass**
/// ```text
/// xs.to_list().mconcat() = xs.concat_s()
/// ```
///
/// The method names refer to the monoid of lists under concatenation,
/// but there are many other instances.
///
/// Some types can be viewed as a monoid in more than one way,
/// e.g. both addition and multiplication on numbers.
///
/// In such cases we often define newtypes and make those instances of `Monoid`, e.g. `Sum` and `Product`.
pub trait Monoid: Semigroup {
    /// Identity of `assoc_s`
    fn mempty() -> Self {
        Monoid::mconcat(vec![])
    }

    /// Fold a list using the monoid.
    ///
    /// For most types, the default definition for mconcat will be used,
    /// but the function is included in the class definition
    /// so that an optimized version can be provided for specific types.
    ///
    /// ```
    /// # use glasgae::prelude::{Functor, Monoid, Show};
    /// assert_eq!(String::mconcat(vec!["Hello", " ", "Glasgae", "!"].fmap(Show::show)), "Hello Glasgae!".to_string());
    /// ```
    fn mconcat(list: Vec<Self>) -> Self {
        list.foldr(Semigroup::assoc_s, Monoid::mempty())
    }
}
