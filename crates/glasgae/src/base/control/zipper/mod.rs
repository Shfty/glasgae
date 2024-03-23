//! Continuation-based zipper.
//!
//! Based on the zipper described in [Final Zipper](https://okmij.org/ftp/continuations/zipper.html) by Oleg Kiselyov.

mod travel;
mod zip_travel;

pub use travel::*;
pub use zip_travel::*;

use crate::transformers::cont::Cont;
use crate::{derive_applicative, derive_functor, derive_pointed, derive_with_pointed, prelude::*};

#[derive(Clone)]
pub enum Zipper<T, D>
where
    T: Term,
    D: Term,
{
    Zipper(T, Function<(Option<T>, D), Zipper<T, D>>),
    ZipDone(T),
}

impl<T, D> std::fmt::Debug for Zipper<T, D>
where
    T: Term + std::fmt::Debug,
    D: Term,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZipDone(arg0) => f.debug_tuple("ZipDone").field(arg0).finish(),
            Self::Zipper(arg0, _) => f.debug_tuple("Zipper").field(arg0).finish(),
        }
    }
}

impl<T, D> Zipper<T, D>
where
    T: Term,
    D: Term,
{
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

derive_pointed!(Zipper<(T), D>);
derive_with_pointed!(Zipper<(T), D>);

impl<T, D> Functor<T> for Zipper<T, D>
where
    T: Term,
    D: Term + Default,
{
    type Mapped = Zipper<T, D>;

    fn fmap(self, f: impl FunctionT<T, T>) -> Zipper<T, D> {
        match self {
            Zipper::Zipper(t, n) => n((Some(f.to_function()(t)), Default::default())),
            Zipper::ZipDone(t) => Zipper::ZipDone(f(t)),
        }
    }
}

impl<T, D> PureA for Zipper<T, D>
where
    T: Term,
    D: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Zipper::done(t)
    }
}

impl<F, T, D> AppA<T, T> for Zipper<F, D>
where
    F: Term + FunctionT<T, T>,
    T: Term,
    D: Term + Default,
{
    type WithA = Zipper<T, D>;
    type WithB = Zipper<T, D>;

    fn app_a(self, a: Zipper<T, D>) -> Zipper<T, D> {
        let f = self.unwrap_unchecked();
        a.fmap(f)
    }
}

impl<T, D> ReturnM for Zipper<T, D>
where
    T: Term,
    D: Term,
{
}

impl<T, U, D> ChainM<U> for Zipper<T, D>
where
    T: Term,
    U: Term,
    D: Term,
{
    type Chained = Zipper<U, D>;

    fn chain_m(self, f: impl FunctionT<T, Zipper<U, D>>) -> Zipper<U, D> {
        f(self.unwrap_unchecked())
    }
}

// FIXME: Not useful with Default::default direction, useful semantic is Next
impl<T, U, D> Foldable<U> for Zipper<T, D>
where
    T: Term,
    D: Term + Default,
    U: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, init: U) -> U {
        let f = f.to_bifun();
        match self {
            Zipper::Zipper(t, n) => f.clone()(t, n((None, Default::default())).foldr(f, init)),
            Zipper::ZipDone(t) => f(t, init),
        }
    }

    fn foldl(self, f: impl BifunT<U, T, U>, init: U) -> U {
        let f = f.to_bifun();
        match self {
            Zipper::Zipper(t, n) => f.clone()(n((None, Default::default())).foldl(f, init), t),
            Zipper::ZipDone(t) => f(init, t),
        }
    }
}

impl<T, D> Foldable1<T> for Zipper<T, D>
where
    T: Term,
    D: Term + Default,
{
    fn foldr1(self, f: impl BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}

pub trait MakeZipper<D>: Term
where
    D: Term,
{
    fn make_zipper(
        self,
        trav: impl BifunT<
            Self,
            Function<Self, Cont<Zipper<Self, D>, (Option<Self>, D)>>,
            Cont<Zipper<Self, D>, Self>,
        >,
    ) -> Cont<Zipper<Self, D>>;
}

impl<T, D> MakeZipper<D> for T
where
    T: Term,
    D: Term,
{
    fn make_zipper(
        self,
        trav: impl BifunT<
            Self,
            Function<Self, Cont<Zipper<Self, D>, (Option<Self>, D)>>,
            Cont<Zipper<Self, D>, Self>,
        >,
    ) -> Cont<Zipper<Self, D>> {
        trav(
            self.clone(),
            (|term: T| Cont::shift((|k| ReturnM::return_m(Zipper::zip(term, k))).boxed())).boxed(),
        )
        .chain_m(|t| ReturnM::return_m(Zipper::done(t)))
        .reset()
    }
}
