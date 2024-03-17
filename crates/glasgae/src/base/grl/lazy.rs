//! Lazy computations.

use crate::{
    base::data::function::{Nullary, NullaryT},
    prelude::*,
};

/// A value of type [`Lazy<T>`] represents a yet-to-be computed value,
/// allowing expensive computations to be deferred until their output is required.
/// 
/// Mechanically, it is similar to [`IO<T>`] absent the impure semantic;
/// a closure of arity zero that produces some value when executed.
///
/// As such, care should be taken to use this only in pure contexts;
/// if the inner closure creates side-effects, [`IO`] should be used instead.
#[derive(Clone)]
pub struct Lazy<A: Term>(Nullary<A>);

impl<T> Lazy<T>
where
    T: Term,
{
    /// Construct a new lazy value from a nullary function.
    pub fn new(f: impl NullaryT<T>) -> Self {
        Lazy(f.boxed())
    }

    /// Evaluate the lazy value, producing a concrete value
    pub fn run(self) -> T {
        self.0()
    }
}

impl<T> Pointed for Lazy<T>
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U> WithPointed<U> for Lazy<T>
where
    T: Term,
    U: Term,
{
    type WithPointed = Lazy<U>;
}

impl<T, U> Fmap<U> for Lazy<T>
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, U>) -> Self::WithPointed {
        let f = f.to_function();
        Lazy::new(|| f(self.run()))
    }
}

impl<T> PureA for Lazy<T>
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Lazy::new(|| t)
    }
}

impl<F, A, B> AppA<Lazy<A>, Lazy<B>> for Lazy<F>
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: Lazy<A>) -> Lazy<B> {
        Lazy::new(|| self.run()(a.run()))
    }
}

impl<T> ReturnM for Lazy<T> where T: Term {}

impl<T, U> ChainM<Lazy<U>> for Lazy<T>
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Lazy<U>>) -> Lazy<U> {
        let f = f.to_function();
        Lazy::new(|| f(self.run()).run())
    }
}

impl<T> Semigroup for Lazy<T>
where
    T: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        Lazy::new(|| self.run().assoc_s(a.run()))
    }
}

impl<T> Monoid for Lazy<T>
where
    T: Monoid,
{
    fn mempty() -> Self {
        Lazy::new(|| T::mempty())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.foldr(Semigroup::assoc_s, Monoid::mempty())
    }
}

