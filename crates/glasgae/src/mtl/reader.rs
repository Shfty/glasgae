//! MonadReader trait.
//!
//! Computation type:
//! Computations which read values from a shared environment.
//!
//! Binding strategy:
//! Monad values are functions from the environment to a value. The bound function is applied to the bound value, and both have access to the shared environment.
//!
//! Useful for:
//! Maintaining variable bindings, or other shared environment.
//!
//! Zero and plus:
//! None.
//!
//! Example type:
//! Reader [(String,Value)] a
//!
//! The Reader monad (also called the Environment monad).
//! Represents a computation, which can read values from a shared environment,
//! pass values from function to function, and execute sub-computations in a modified environment.
//! Using Reader monad for such computations is often clearer and easier than using the State monad.
//!
//! Inspired by the paper Functional Programming with Overloading and Higher-Order Polymorphism,
//! Mark P Jones (<http://web.cecs.pdx.edu/~mpj/>) Advanced School of Functional Programming, 1995.
//!
//! See examples in Control.Monad.Reader. Note, the partially applied function type (->) r is a simple reader monad. See the instance declaration below.
use crate::prelude::*;

use crate::transformers::{
    class::MonadTrans, cont::ContT, reader::ReaderT, state::StateT, writer::WriterT,
};

pub trait MonadAsk<MR, R, A> {
    /// Retrieves the monad environment.
    fn ask() -> Self;
}

pub trait MonadLocal<MR, R> {
    /// Executes a computation in a modified environment.
    ///
    /// Self: Reader to run in the modified environment.
    /// f: The function to modify the environment.
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self;
}

pub trait MonadReader<R, A> {
    /// Retrieves a function of the current environment.
    ///
    /// f: The selector function to apply to the environment.
    fn reader(f: impl FunctionT<R, A> + Clone) -> Self;
}

// Fun impl
impl<R> MonadAsk<(), R, R> for Function<R, R>
where
    R: 'static,
{
    fn ask() -> Self {
        identity.boxed()
    }
}

impl<R> MonadLocal<(), R> for Function<R, R>
where
    R: 'static,
{
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self {
        f.compose_clone(self).boxed()
    }
}

impl<R> MonadReader<R, R> for Function<R, R>
where
    R: 'static,
{
    fn reader(_: impl FunctionT<R, R> + Clone) -> Self {
        identity.boxed()
    }
}

// ReaderT impl
impl<MR, R> MonadAsk<MR, R, R> for ReaderT<R, MR>
where
    MR: ReturnM<Pointed = R>,
{
    fn ask() -> Self {
        ReaderT::ask()
    }
}

impl<MR, R> MonadLocal<MR, R> for ReaderT<R, MR>
where
    R: Clone,
    MR: Clone,
{
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self {
        self.local(f)
    }
}

impl<MA, R, A> MonadReader<R, A> for ReaderT<R, MA>
where
    MA: ReturnM<Pointed = A>,
{
    fn reader(f: impl FunctionT<R, A> + Clone) -> Self {
        ReaderT::<R, MA>::new(f)
    }
}

// WriterT impl
impl<MR, MA, W, R, A> MonadAsk<MR, R, A> for WriterT<W, MA>
where
    Self: MonadTrans<MA>,
    W: Clone + Monoid,
    MA: Clone + MonadAsk<MR, R, A> + ReturnM<Pointed = (A, W)> + ChainM<MA>,
{
    fn ask() -> Self {
        Self::lift(MA::ask())
    }
}

impl<MA, MR, S, R> MonadLocal<MR, R> for WriterT<S, MA>
where
    S: Clone,
    MA: Clone + MonadLocal<MR, R>,
{
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self {
        self.map_t(|t| t.local(f))
    }
}

impl<MA, S, R, A> MonadReader<R, A> for WriterT<S, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadReader<R, A>,
{
    fn reader(f: impl FunctionT<R, A> + Clone) -> Self {
        Self::lift(MA::reader(f))
    }
}

// StateT impl
impl<MR, MA, S, R, A> MonadAsk<MR, R, A> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadAsk<MR, R, A>,
{
    fn ask() -> Self {
        Self::lift(MA::ask())
    }
}

impl<MA, MR, S, R> MonadLocal<MR, R> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Clone,
    MA: Clone + MonadLocal<MR, R>,
{
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self {
        self.map_t(|t| t.local(f))
    }
}

impl<MA, S, R, A> MonadReader<R, A> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Clone,
    MA: MonadReader<R, A>,
{
    fn reader(f: impl FunctionT<R, A> + Clone) -> Self {
        Self::lift(MA::reader(f))
    }
}

// ContT impl
impl<MR, MA, S, R, A> MonadAsk<MR, R, A> for ContT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Clone,
    MA: Clone + MonadAsk<MR, R, A> + Pointed,
{
    fn ask() -> Self {
        Self::lift(MA::ask())
    }
}

impl<MR, MR_, MA, R> MonadLocal<MR_, R> for ContT<MR, MA>
where
    MR: Clone + Pointed<Pointed = R> + MonadAsk<MR_, R, R> + MonadLocal<MR_, R> + ChainM<MR>,
    MR::Pointed: Clone,
    MR_: 'static,
    R: 'static,
    MA: Clone + Pointed,
    MA::Pointed: Clone,
{
    fn local(self, f: impl FunctionT<R, R> + Clone) -> Self {
        self.lift_local(MR::ask(), MR::local, f)
    }
}

impl<MA, S, R, A> MonadReader<R, A> for ContT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Clone,
    MA: MonadReader<R, A> + Pointed,
{
    fn reader(f: impl FunctionT<R, A> + Clone) -> Self {
        Self::lift(MA::reader(f))
    }
}

// Support functions
pub trait Asks<R, A>: Sized + MonadReader<R, A> {
    /// Retrieves a function of the current environment.
    ///
    /// f: The selector function to apply to the environment.
    fn asks(f: impl FunctionT<R, A> + Clone) -> Self {
        Self::reader(f)
    }
}

impl<R, A, T> Asks<R, A> for T where T: MonadReader<R, A> {}
