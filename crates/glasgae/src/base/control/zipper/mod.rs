//! Continuation-based zipper.
//!
//! Based on the zipper described in [Final Zipper](https://okmij.org/ftp/continuations/zipper.html) by Oleg Kiselyov.

mod travel;
mod zip_travel;

use std::panic::UnwindSafe;

pub use travel::*;
pub use zip_travel::*;

use crate::{base::data::function::bifunction::BifunT, prelude::*};

use crate::transformers::cont::Cont;

#[derive(Clone)]
pub enum Zipper<T, D>
where
    T: 'static,
    D: 'static,
{
    Zipper(T, Function<(Option<T>, D), Zipper<T, D>>),
    ZipDone(T),
}

impl<T, D> std::fmt::Debug for Zipper<T, D>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZipDone(arg0) => f.debug_tuple("ZipDone").field(arg0).finish(),
            Self::Zipper(arg0, _) => f.debug_tuple("Zipper").field(arg0).finish(),
        }
    }
}

impl<T, D> Zipper<T, D> {
    pub fn zip(a: T, f: impl FunctionT<(Option<T>, D), Zipper<T, D>> + 'static) -> Self {
        Zipper::Zipper(a, f.boxed())
    }

    pub fn done(t: T) -> Self {
        Zipper::ZipDone(t)
    }

    // Returns Ok if the zipper is done, Err otherwise
    pub fn try_unwrap(self) -> Result<T, T> {
        match self {
            Zipper::Zipper(t, _) => Err(t),
            Zipper::ZipDone(t) => Ok(t),
        }
    }

    // Returns the contents of the zipper, panics if the zipper is not done
    pub fn unwrap(self) -> T {
        match self {
            Zipper::Zipper(_, _) => panic!("Zipper is not Done"),
            Zipper::ZipDone(t) => t,
        }
    }

    // Returns the contents of the zipper, regardless of whether it is done or not
    pub fn unwrap_unchecked(self) -> T {
        match self {
            Zipper::Zipper(t, _) => t,
            Zipper::ZipDone(t) => t,
        }
    }
}

impl<T, D> Pointed for Zipper<T, D> {
    type Pointed = T;
}

impl<T, U, D> WithPointed<U> for Zipper<T, D>
where
    U: 'static,
{
    type WithPointed = Zipper<U, D>;
}

impl<T, D> Functor<T> for Zipper<T, D>
where
    T: Clone + UnwindSafe,
    D: Default,
{
    fn fmap(self, f: impl FunctionT<T, T> + Clone) -> Zipper<T, D> {
        match self {
            Zipper::Zipper(t, n) => n((Some(f.clone()(t)), Default::default())),
            Zipper::ZipDone(t) => Zipper::ZipDone(f(t)),
        }
    }
}

impl<T, D> PureA for Zipper<T, D> {
    fn pure_a(t: Self::Pointed) -> Self {
        Zipper::done(t)
    }
}

impl<F, T, D> AppA<Zipper<T, D>, Zipper<T, D>> for Zipper<F, D>
where
    Zipper<T, D>: Pointed<Pointed = T> + Functor<T, WithPointed = Zipper<T, D>>,
    F: FunctionT<T, T> + Clone,
    T: Clone + UnwindSafe,
    D: Default,
{
    fn app_a(self, a: Zipper<T, D>) -> Zipper<T, D> {
        let f = self.unwrap_unchecked();
        a.fmap(f)
    }
}

impl<T, D> ReturnM for Zipper<T, D> {}

impl<T, U, D> ChainM<Zipper<U, D>> for Zipper<T, D> {
    fn chain_m(self, f: impl FunctionT<T, Zipper<U, D>> + Clone) -> Zipper<U, D> {
        f(self.unwrap_unchecked())
    }
}

// FIXME: Not useful with Default::default direction, useful semantic is Next
impl<T, U, D> Foldr<T, U> for Zipper<T, D>
where
    D: Default,
{
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, init: U) -> U {
        match self {
            Zipper::Zipper(t, n) => f.clone()(t, n((None, Default::default())).foldr(f, init)),
            Zipper::ZipDone(t) => f(t, init),
        }
    }
}

pub trait MakeZipper<D>: Sized {
    fn make_zipper(
        self,
        trav: impl BifunT<
                Self,
                Function<Self, Cont<Zipper<Self, D>, (Option<Self>, D)>>,
                Cont<Zipper<Self, D>, Self>,
            > + Clone,
    ) -> Cont<Zipper<Self, D>>;
}

impl<T, D> MakeZipper<D> for T
where
    T: Clone + UnwindSafe,
    D: Clone,
{
    fn make_zipper(
        self,
        trav: impl BifunT<
                Self,
                Function<Self, Cont<Zipper<Self, D>, (Option<Self>, D)>>,
                Cont<Zipper<Self, D>, Self>,
            > + Clone,
    ) -> Cont<Zipper<Self, D>> {
        trav(
            self.clone(),
            (|term: T| Cont::shift((|k| ReturnM::return_m(Zipper::zip(term, k))).boxed())).boxed(),
        )
        .chain_m(|t| ReturnM::return_m(Zipper::done(t)))
        .reset()
    }
}
