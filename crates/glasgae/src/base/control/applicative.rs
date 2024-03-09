//! A functor with application, providing operations to embed pure expressions ([`pure`](PureA::pure_a)),
//! and sequence computations and combine their results ([`app_a`](AppA::app_a) and
//! [`lift_a2`](LiftA2::lift_a2)).
//!
//! A minimal complete definition must include implementations of [`pure`](PureA::pure_a)
//! and of either [`app_a`](AppA::app_a) or [`lift_a2`](LiftA2::lift_a2).
//!
//! If it defines both, then they must behave the same as their default definitions:
//! ```text
//! app_a == identity.lift_a2()
//! ```
//! ```text
//! f.lift_a2(x, y) == f.fmap(x).app_a(y)
//! ```
//!
//! Further, any definition must satisfy the following:
//!
//! **Identity**
//! ```text
//! PureA::pure_a(identity).app_a(v) == v
//! ```
//!
//! **Composition**
//! ```text
//! PureA::pure_a(Compose::compose).app_a(u).app_a(v).app_a(w) == u.app_a(v.app_a(w))
//! ```
//!
//! **Homomorphism**
//! ```text
//! PureA::pure_a(f).app(PureA::pure_a(x)) == PureA::pure_a(f(x))
//! ```
//!
//! **Interchange**
//! ```text
//! u.app_a(PureA::pure_a(y)) == PureA::pure_a(|f| f(y)).app_a(u)
//! ```
//!
//! The other methods have the following default definitions, which may be overridden with equivalent specialized implementations:
//! ```text
//! u *> v = (id <$ u) <*> v
//! ```
//! ```text
//! u <* v = liftA2 const u v
//! ```
//!
//! As a consequence of these laws, the Functor instance for f will satisfy
//! ```text
//! fmap f x = pure f <*> x
//! ```
//!
//! It may be useful to note that supposing
//! ```text
//! forall x y. p (q x y) = f x . g y
//! ```
//!
//! it follows from the above that
//!
//! ```text
//! liftA2 p (liftA2 q u v) = liftA2 f u . liftA2 g v
//! ```
//!
//! If f is also a Monad, it should satisfy
//!
//! ```text
//! pure = return
//! ```
//! ```text
//! m1 <*> m2 = m1 >>= (\x1 -> m2 >>= (\x2 -> return (x1 x2)))
//! ```
//! ```text
//! (*>) = (>>)
//! ```
//! (which implies that pure and <*> satisfy the applicative functor laws).

use std::panic::UnwindSafe;

use crate::{base::data::function::bifunction::BifunT, prelude::*};

/// Lift a value.
pub trait PureA: Pointed {
    fn pure_a(t: Self::Pointed) -> Self;
}

/// Sequential application.
///
/// A few functors support an implementation of [`app_a`](AppA::app_a) that is more efficient than the default one.
///
/// # Example
///
/// Used in combination with [`fmap`](crate::prelude::Functor::fmap), [`app_a`](AppA::app_a) can be used to build a record.
///
/// ```ignore
/// struct Foo;
/// struct Bar;
/// struct Baz;
///
/// struct MyState {
///     arg1: Foo,
///     arg2: Bar,
///     arg3: Baz
/// }
///
/// impl MyState {
///     pub fn new(arg1: Foo, arg2: Bar, arg3: Baz) -> Self {
///         MyState {
///             arg1,
///             arg2,
///             arg3
///         }
///     }
/// }
///
/// fn produce_foo() -> <impl Applicative> {
///     ...
/// }
///
/// fn produce_bar() -> <impl Applicative> {
///     ...
/// }
///
/// fn produce_baz() -> <impl Applicative> {
///     ...
/// }
///
/// let mk_state = produce_foo()
///     .fmap(MyState::new.curried())
///     .app(produce_bar())
///     .app(produce_baz());
/// ```
pub trait AppA<A1, A2> {
    fn app_a(self, a: A1) -> A2;
}

/// Lift a binary function to actions.
///
/// Some functors support an implementation of [`lift_a2`](LiftA2::lift_a2)
/// that is more efficient than the default one.
///
/// In particular, if fmap is an expensive operation,
/// it is likely better to use [`lift_a2`](LiftA2::lift_a2)
/// than to [`fmap`](Functor::fmap) over the structure and then use [`app_a`](AppA::app_a).
///
/// # Example
/// ```
/// # use glasgae::{prelude::LiftA2, base::data::tuple::pair::Pair};
/// assert_eq!(
///     Pair::pair.lift_a2()(Some(3), Some(5)),
///     Some((3, 5))
/// )
/// ```
pub trait LiftA2<A1, A2, A3>:
    FnOnce(A1::Pointed, A2::Pointed) -> A3::Pointed + Clone + 'static
where
    Self: UnwindSafe,
    A1::Pointed: 'static + Clone + UnwindSafe,
    A1: Functor<Function<A2::Pointed, A3::Pointed>>,
    A1::WithPointed: AppA<A2, A3>,
    A2: Pointed,
    A2::Pointed: 'static,
    A3: Pointed,
    A3::Pointed: 'static,
{
    fn lift_a2(self) -> impl BifunT<A1, A2, A3> + Clone {
        |a1, a2| a1.fmap(|t| (|v| self(t, v)).boxed()).app_a(a2)
    }
}

impl<F, A1, A2, A3> LiftA2<A1, A2, A3> for F
where
    F: BifunT<A1::Pointed, A2::Pointed, A3::Pointed> + Clone,
    A1: Functor<Function<A2::Pointed, A3::Pointed>>,
    A1::Pointed: 'static + Clone + UnwindSafe,
    A1::WithPointed: AppA<A2, A3>,
    A2: Pointed,
    A2::Pointed: 'static,
    A3: Pointed,
    A3::Pointed: 'static,
{
}
