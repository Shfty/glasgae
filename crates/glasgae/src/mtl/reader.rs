//! [`MonadReader`] trait generalizing [`ReaderT`] functionality.
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
use crate::{
    base::control::monad::morph::MonadLower,
    prelude::*,
    transformers::{
        class::MonadTrans, cont::ContT, reader::ReaderT, state::StateT, writer::WriterT,
    },
};

pub trait MonadAsk<MR, R, A>: Term {
    /// Retrieves the monad environment.
    fn ask() -> Self;
}

pub trait MonadLocal<MR, R>: Term
where
    R: Term,
{
    /// Executes a computation in a modified environment.
    ///
    /// Self: Reader to run in the modified environment.
    /// f: The function to modify the environment.
    fn local(self, f: impl FunctionT<R, R>) -> Self;
}

pub trait MonadReader<R, A>: Term
where
    R: Term,
    A: Term,
{
    /// Retrieves a function of the current environment.
    ///
    /// f: The selector function to apply to the environment.
    fn reader(f: impl FunctionT<R, A>) -> Self;
}

// Fun impl
impl<R> MonadAsk<(), R, R> for Function<R, R>
where
    R: Term,
{
    fn ask() -> Self {
        identity.boxed()
    }
}

impl<R> MonadLocal<(), R> for Function<R, R>
where
    R: Term,
{
    fn local(self, f: impl FunctionT<R, R>) -> Self {
        f.to_function().compose_clone(self).boxed()
    }
}

impl<R> MonadReader<R, R> for Function<R, R>
where
    R: Term,
{
    fn reader(_: impl FunctionT<R, R>) -> Self {
        identity.boxed()
    }
}

// ReaderT impl
impl<MR, R> MonadAsk<MR, R, R> for ReaderT<R, MR>
where
    MR: ReturnM<Pointed = R>,
    R: Term,
{
    fn ask() -> Self {
        ReaderT::ask()
    }
}

impl<MA, R> MonadLocal<MA, R> for ReaderT<R, MA>
where
    R: Term,
    MA: Pointed,
{
    fn local(self, f: impl FunctionT<R, R>) -> Self {
        self.local(f.to_function())
    }
}

impl<MA, R, A> MonadReader<R, A> for ReaderT<R, MA>
where
    MA: ReturnM<Pointed = A>,
    R: Term,
    A: Term,
{
    fn reader(f: impl FunctionT<R, A>) -> Self {
        ReaderT::<R, MA>::new(f)
    }
}

// WriterT impl
impl<MR, MA, W, R, A> MonadAsk<MR, R, A> for WriterT<W, MA>
where
    MA: MonadLower<A, W> + ReturnM<Pointed = (A, W)>,
    MA::Lowered: ChainM<MA, Pointed = A> + MonadAsk<MR, R, A>,
    W: Monoid,
    A: Term,
{
    fn ask() -> Self {
        WriterT::lift(<MA::Lowered>::ask())
    }
}

impl<MA, MR, S, R> MonadLocal<MR, R> for WriterT<S, MA>
where
    MA: MonadLocal<MR, R>,
    S: Term,
    R: Term,
{
    fn local(self, f: impl FunctionT<R, R>) -> Self {
        let f = f.to_function();
        self.map_t(|t| t.local(f))
    }
}

impl<MA, W, R, A> MonadReader<R, A> for WriterT<W, MA>
where
    MA: MonadLower<A, W> + ReturnM<Pointed = (A, W)>,
    MA::Lowered: MonadReader<R, A> + ChainM<MA, Pointed = A>,
    W: Monoid,
    R: Term,
    A: Term,
{
    fn reader(f: impl FunctionT<R, A>) -> Self {
        Self::lift(<MA::Lowered>::reader(f))
    }
}

// StateT impl
impl<MR, MA, S, R, A> MonadAsk<MR, R, A> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Term,
    MA: MonadAsk<MR, R, A>,
{
    fn ask() -> Self {
        Self::lift(MA::ask())
    }
}

impl<MA, MR, S, R> MonadLocal<MR, R> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadLocal<MR, R>,
    S: Term,
    R: Term,
{
    fn local(self, f: impl FunctionT<R, R>) -> Self {
        let f = f.to_function();
        self.map_t(|t| t.local(f))
    }
}

impl<MA, S, R, A> MonadReader<R, A> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadReader<R, A>,
    S: Term,
    R: Term,
    A: Term,
{
    fn reader(f: impl FunctionT<R, A>) -> Self {
        let f = f.to_function();
        Self::lift(MA::reader(f))
    }
}

// ContT impl
impl<MR, MA, S, R, A> MonadAsk<MR, R, A> for ContT<S, MA>
where
    Self: MonadTrans<MA>,
    S: Pointed,
    MA: MonadAsk<MR, R, A> + Pointed,
{
    fn ask() -> Self {
        Self::lift(MA::ask())
    }
}

impl<MR, MR_, MA, R> MonadLocal<MR_, R> for ContT<MR, MA>
where
    MR: Pointed<Pointed = R> + MonadAsk<MR_, R, R> + MonadLocal<MR_, R> + ChainM<MR>,
    MR_: Term,
    R: Term,
    MA: Pointed,
{
    fn local(self, f: impl FunctionT<R, R>) -> Self {
        self.lift_local(MR::ask(), MR::local, f)
    }
}

impl<MA, MS, R, A> MonadReader<R, A> for ContT<MS, MA>
where
    MA: MonadReader<R, A> + ChainM<MS>,
    MS: Pointed,
    R: Term,
    A: Term,
{
    fn reader(f: impl FunctionT<R, A>) -> Self {
        Self::lift(MA::reader(f))
    }
}

// Support functions
pub trait Asks<R, A>: Sized + MonadReader<R, A>
where
    R: Term,
    A: Term,
{
    /// Retrieves a function of the current environment.
    ///
    /// f: The selector function to apply to the environment.
    fn asks(f: impl FunctionT<R, A>) -> Self {
        Self::reader(f)
    }
}

impl<R, A, T> Asks<R, A> for T
where
    T: MonadReader<R, A>,
    R: Term,
    A: Term,
{
}
