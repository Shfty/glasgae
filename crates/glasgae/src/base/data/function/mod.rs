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

pub use app::*;
pub use compose::*;
pub use curried::*;
pub use curry::*;
pub use curry_flipped::*;
pub use flip::*;
pub use nullary::*;
pub use until::*;

use crate::prelude::*;

use super::term::TermBase;

pub trait FunctionT<A, B>: TermBase + FnOnce(A) -> B
where
    A: Term,
    B: Term,
{
    fn to_function(&self) -> Function<A, B>;
}

impl<F, A, B> FunctionT<A, B> for F
where
    F: Term + FnOnce(A) -> B,
    A: Term,
    B: Term,
{
    fn to_function(&self) -> Function<A, B> {
        self.clone().boxed()
    }
}

pub type Function<A, B> = Box<dyn FunctionT<A, B>>;

impl<A, B> Clone for Function<A, B>
where
    A: Term,
    B: Term,
{
    fn clone(&self) -> Self {
        (**self).to_function()
    }
}

impl<A, B> Kinded for Function<A, B>
where
    A: Term,
    B: Term,
{
    type Kinded = B;
}

impl<A, B, C> WithKinded<C> for Function<A, B>
where
    A: Term,
    B: Term,
    C: Term,
{
    type WithKinded = Function<A, C>;
}

impl<A, B> Pointed for Function<A, B>
where
    A: Term,
    B: Term,
{
    type Pointed = B;
}

impl<A, B, C> WithPointed<C> for Function<A, B>
where
    A: Term,
    B: Term,
    C: Term,
{
    type WithPointed = Function<A, C>;
}

impl<A, B, C> Functor<C> for Function<A, B>
where
    A: Term,
    B: Term,
    C: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, C>) -> Function<A, C> {
        self.compose_clone(f.to_function()).boxed()
    }
}

impl<A, B> PureA for Function<A, B>
where
    A: Term,
    B: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        r#const(t).boxed()
    }
}

impl<I, F, A, B> AppA<Function<I, A>, Function<I, B>> for Function<I, F>
where
    F: Term + FunctionT<A, B>,
    I: Term,
    A: Term,
    B: Term,
{
    fn app_a(self, g: Function<I, A>) -> Function<I, B> {
        let f = self;
        (|x: I| f(x.clone())(g(x))).boxed()
    }
}

impl<A, B> ReturnM for Function<A, B>
where
    A: Term,
    B: Term,
{
}

impl<A, B, C> ChainM<C> for Function<A, B>
where
    A: Term,
    B: Term,
    C: Term,
{
    fn chain_m(self, k: impl FunctionT<Self::Pointed, Function<A, C>>) -> Function<A, C> {
        let f = self;
        let k = k.to_function();
        (|r: A| k(f(r.clone()))(r)).boxed()
    }
}

pub fn r#const<T, U>(t: U) -> impl FunctionT<T, U>
where
    T: Term,
    U: Term,
{
    move |_| t
}
