//! Canonical [`FunctionT`] trait and corresponding [`Function`] value type.
//!
//! Given that Rust's stable function interface spans
//! [`FnOnce`], [`FnMut`], [`Fn`] and [`fn`] to account
//! for ownership and mutability semantics,
//! a unified interface is necessary for a pure functional programming framework.
//!
//! To that end, this module defines [`FunctionT`];
//! an object-safe supertrait of [`FnOnce`] with `'static` lifetime,
//! and blanket implementation over all applicable types that
//! also implement [`Clone`].
//!
//! This allows functions and their captured variables to be boxed
//! and cloned where necessary, thus satisfying the by-value copy semantics
//! of a pure functional language.
//!
//! This gives rise to [`Function<A, B>`]; the canonical by-value function type,
//! for use in cases where `impl FunctionT<A, B> + Clone` is untenable
//! (such as when representing higher-arity curried functions.)

mod app;
mod compose;
mod curried;
mod curry;
mod curry_flipped;
mod flip;
mod nullary;
mod until;

pub mod bifunction;

use std::panic::UnwindSafe;

pub use app::*;
pub use compose::*;
pub use curried::*;
pub use curry::*;
pub use curry_flipped::*;
pub use flip::*;
pub use nullary::*;
pub use until::*;

use crate::prelude::*;

pub trait FunctionT<A, B>: FnOnce(A) -> B + UnwindSafe + 'static {
    fn clone_fun(&self) -> Function<A, B>;
}

impl<F, A, B> FunctionT<A, B> for F
where
    F: FnOnce(A) -> B + Clone + UnwindSafe + 'static,
{
    fn clone_fun(&self) -> Function<A, B> {
        self.clone().boxed()
    }
}

pub type Function<A, B> = Box<dyn FunctionT<A, B>>;

impl<A, B> Clone for Function<A, B>
where
    A: 'static,
    B: 'static,
{
    fn clone(&self) -> Self {
        (**self).clone_fun()
    }
}

impl<A, B> Pointed for Function<A, B> {
    type Pointed = B;
}

impl<A, B, C> WithPointed<C> for Function<A, B> {
    type WithPointed = Function<A, C>;
}

impl<A, B, C> Functor<C> for Function<A, B>
where
    A: 'static,
    B: 'static,
    C: 'static + Clone + UnwindSafe,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, C> + Clone) -> Function<A, C> {
        self.compose_clone(f).boxed()
    }
}

impl<A, B> PureA for Function<A, B>
where
    B: 'static + Clone + UnwindSafe,
{
    fn pure_a(t: Self::Pointed) -> Self {
        r#const(t).boxed()
    }
}

impl<I, F, A, B> AppA<Function<I, A>, Function<I, B>> for Function<I, F>
where
    F: FunctionT<A, B> + Clone,
    I: 'static + Clone,
    A: 'static,
{
    fn app_a(self, g: Function<I, A>) -> Function<I, B> {
        let f = self;
        (|x: I| f(x.clone())(g(x))).boxed()
    }
}

impl<A, B> ReturnM for Function<A, B> where B: 'static + Clone + UnwindSafe {}

impl<A, B, C> ChainM<Function<A, C>> for Function<A, B>
where
    A: 'static + Clone,
    B: 'static,
{
    fn chain_m(self, k: impl FunctionT<Self::Pointed, Function<A, C>> + Clone) -> Function<A, C> {
        let f = self;
        (|r: A| k(f(r.clone()))(r)).boxed()
    }
}

pub fn r#const<T, U>(t: U) -> impl FunctionT<T, U> + Clone
where
    U: 'static + Clone + UnwindSafe,
{
    move |_| t
}
