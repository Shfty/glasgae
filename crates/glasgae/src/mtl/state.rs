//! [`MonadState`] trait generalizing [`StateT`] functionality.
//!
//! This module is inspired by the paper
//! Functional Programming with Overloading and Higher-Order Polymorphism,
//! Mark P Jones (<http://web.cecs.pdx.edu/~mpj/>) Advanced School of Functional Programming, 1995.

use std::panic::UnwindSafe;

use crate::{
    base::data::pointed::{Lower, LoweredT},
    prelude::*,
    transformers::{
        class::MonadTrans, cont::ContT, reader::ReaderT, state::StateT, writer::WriterT,
    },
};

pub trait StateTypes: Pointed {
    type State;
}

pub type StateTypesStateT<T> = <T as StateTypes>::State;

pub trait MonadGet: StateTypes {
    /// Return the state from the internals of the monad.
    fn get() -> Self;
}

pub trait MonadPut: StateTypes {
    /// Replace the state inside the monad.
    fn put(s: Self::State) -> Self;
}

pub trait MonadState: StateTypes {
    /// Embed a simple state action into the monad.
    fn state(f: impl FunctionT<Self::State, (Self::Pointed, Self::State)> + Clone) -> Self;
}

// StateT impl
impl<ME, S> MonadGet for StateT<S, ME>
where
    S: Clone,
    ME: ReturnM<Pointed = (S, S)>,
{
    fn get() -> Self {
        Self::get()
    }
}

impl<S, MA> StateTypes for StateT<S, MA>
where
    StateT<S, MA>: Pointed,
{
    type State = S;
}

impl<S, MA> MonadPut for StateT<S, MA>
where
    S: Clone,
    MA: 'static + ReturnM<Pointed = ((), S)>,
{
    fn put(s: S) -> Self {
        Self::put(s)
    }
}

impl<MA, S, A> MonadState for StateT<S, MA>
where
    S: UnwindSafe,
    MA: UnwindSafe + ReturnM<Pointed = (A, S)>,
    A: 'static,
{
    fn state(f: impl FunctionT<S, (A, S)> + Clone) -> Self {
        Self::new(f)
    }
}

// ReaderT impl
impl<ME, R> MonadGet for ReaderT<R, ME>
where
    R: UnwindSafe,
    ME: Clone + UnwindSafe + MonadGet,
{
    fn get() -> Self {
        Self::lift(ME::get())
    }
}

impl<R, MA> StateTypes for ReaderT<R, MA>
where
    MA: StateTypes,
{
    type State = MA::State;
}

impl<R, MA> MonadPut for ReaderT<R, MA>
where
    MA: Clone + UnwindSafe + MonadPut,
{
    fn put(s: Self::State) -> Self {
        Self::lift(MA::put(s))
    }
}

impl<ME, R> MonadState for ReaderT<R, ME>
where
    ME: Clone + UnwindSafe + MonadState,
{
    fn state(f: impl FunctionT<ME::State, (ME::Pointed, ME::State)> + Clone) -> Self {
        Self::lift(ME::state(f))
    }
}

// WriterT impl
impl<W, MA> StateTypes for WriterT<W, MA>
where
    Self: Pointed,
    MA: StateTypes,
{
    type State = MA::State;
}

impl<W, MA, A> MonadGet for WriterT<W, MA>
where
    Self: Pointed<Pointed = A> + MonadTrans<LoweredT<MA, W, A>>,
    MA: StateTypes + Lower<W, A>,
    MA::Lowered: MonadGet<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
{
    fn get() -> Self {
        Self::lift(LoweredT::<MA, W, A>::get())
    }
}

impl<MA, W, A> MonadPut for WriterT<W, MA>
where
    Self: Pointed<Pointed = A> + MonadTrans<LoweredT<MA, W, A>>,
    MA: StateTypes + Lower<W, A>,
    MA::Lowered: MonadPut<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
{
    fn put(s: Self::State) -> Self {
        Self::lift(LoweredT::<MA, W, A>::put(s))
    }
}

impl<MA, W, A> MonadState for WriterT<W, MA>
where
    Self: Pointed<Pointed = A> + MonadTrans<LoweredT<MA, W, A>>,
    MA: StateTypes + Lower<W, A>,
    MA::Lowered: MonadState<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
{
    fn state(f: impl FunctionT<Self::State, (Self::Pointed, Self::State)> + Clone) -> Self {
        Self::lift(LoweredT::<MA, W, A>::state(f))
    }
}

// ContT impl
impl<R, MA> StateTypes for ContT<R, MA>
where
    MA: StateTypes,
{
    type State = MA::State;
}

impl<R, MA> MonadGet for ContT<R, MA>
where
    Self: MonadTrans<MA>,
    R: Clone + Pointed,
    MA: Pointed + MonadGet,
{
    fn get() -> Self {
        ContT::lift(MA::get())
    }
}

impl<R, MA> MonadPut for ContT<R, MA>
where
    Self: MonadTrans<MA>,
    R: Monoid,
    MA: Pointed + MonadPut,
{
    fn put(s: Self::State) -> Self {
        Self::lift(MA::put(s))
    }
}

impl<MA, R> MonadState for ContT<R, MA>
where
    Self: MonadTrans<MA>,
    R: Monoid,
    MA: Clone + MonadState + Pointed,
{
    fn state(f: impl FunctionT<MA::State, (MA::Pointed, MA::State)> + Clone) -> Self {
        Self::lift(MA::state(f))
    }
}

// Support functions
pub trait Modify<S, MA>
where
    MA: WithPointed<((), S)>,
{
    /// Monadic state transformer.
    ///
    /// Maps an old state to a new state inside a state monad. The old state is thrown away.
    ///
    /// Main :t modify ((+1) :: Int -> Int)
    /// modify (...) :: (MonadState Int a) => a ()
    ///
    /// This says that modify (+1) acts over any Monad that is a member of the MonadState class, with an Int state.
    fn modify(f: impl FunctionT<S, S> + Clone) -> StateT<S, MA::WithPointed>;
}

impl<T, S, MA> Modify<S, MA> for T
where
    MA: WithPointed<((), S)>,
    MA::WithPointed: ReturnM,
    S: UnwindSafe,
{
    fn modify(f: impl FunctionT<S, S> + Clone) -> StateT<S, MA::WithPointed> {
        StateT::new(|s| ((), f(s)))
    }
}

pub trait Gets<S, A> {
    /// Gets specific component of the state, using a projection function supplied.
    fn gets(f: impl FunctionT<S, A> + Clone) -> Self;
}

impl<T, S, A> Gets<S, A> for T
where
    StateT<S, S>: ChainM<Self, Pointed = S>,
    S: 'static + Clone + ReturnM<Pointed = (S, S)>,
    T: Pointed<Pointed = A> + ReturnM,
{
    fn gets(f: impl FunctionT<S, A> + Clone) -> Self {
        StateT::<S, S>::get().chain_m(|s| ReturnM::return_m(f(s)))
    }
}
