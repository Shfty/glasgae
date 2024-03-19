//! [`MonadState`] trait generalizing [`StateT`] functionality.
//!
//! This module is inspired by the paper
//! Functional Programming with Overloading and Higher-Order Polymorphism,
//! Mark P Jones (<http://web.cecs.pdx.edu/~mpj/>) Advanced School of Functional Programming, 1995.

use crate::{
    prelude::*,
    transformers::{
        class::MonadTrans, cont::ContT, reader::ReaderT, state::StateT, writer::WriterT,
    },
};

pub trait StateTypes: Pointed {
    type State: Term;
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
    fn state(f: impl FunctionT<Self::State, (Self::Pointed, Self::State)>) -> Self;
}

// StateT impl
impl<ME, S> MonadGet for StateT<S, ME>
where
    S: Term,
    ME: ReturnM<Pointed = (S, S)>,
{
    fn get() -> Self {
        Self::get()
    }
}

impl<MA, S, A> StateTypes for StateT<S, MA>
where
    MA: ReturnM<Pointed = (A, S)>,
    S: Term,
    A: Term,
{
    type State = S;
}

impl<S, MA> MonadPut for StateT<S, MA>
where
    S: Term,
    MA: ReturnM<Pointed = ((), S)>,
{
    fn put(s: S) -> Self {
        Self::put(s)
    }
}

impl<MA, S, A> MonadState for StateT<S, MA>
where
    MA: ReturnM<Pointed = (A, S)>,
    S: Term,
    A: Term,
{
    fn state(f: impl FunctionT<S, (A, S)>) -> Self {
        Self::new(f.to_function())
    }
}

// ReaderT impl
impl<ME, R> MonadGet for ReaderT<R, ME>
where
    R: Term,
    ME: Pointed + MonadGet,
{
    fn get() -> Self {
        Self::lift(ME::get())
    }
}

impl<R, MA> StateTypes for ReaderT<R, MA>
where
    R: Term,
    MA: StateTypes,
{
    type State = MA::State;
}

impl<R, MA> MonadPut for ReaderT<R, MA>
where
    R: Term,
    MA: Pointed + MonadPut,
{
    fn put(s: Self::State) -> Self {
        Self::lift(MA::put(s))
    }
}

impl<ME, R> MonadState for ReaderT<R, ME>
where
    R: Term,
    ME: MonadState,
{
    fn state(f: impl FunctionT<ME::State, (ME::Pointed, ME::State)>) -> Self {
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
    Self: Pointed<Pointed = A> + MonadTrans<MonadLoweredT<MA, A, W>>,
    MA: StateTypes + MonadLower<A, W>,
    MA::Lowered: MonadGet<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
{
    fn get() -> Self {
        Self::lift(MonadLoweredT::<MA, A, W>::get())
    }
}

impl<MA, W, A> MonadPut for WriterT<W, MA>
where
    Self: Pointed<Pointed = A> + MonadTrans<MonadLoweredT<MA, A, W>>,
    MA: StateTypes + MonadLower<A, W>,
    MA::Lowered: MonadPut<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
{
    fn put(s: Self::State) -> Self {
        Self::lift(MonadLoweredT::<MA, A, W>::put(s))
    }
}

impl<MA, W, A> MonadState for WriterT<W, MA>
where
    Self: Pointed<Pointed = A> + MonadTrans<MonadLoweredT<MA, A, W>>,
    MA: StateTypes + MonadLower<A, W>,
    MA::Lowered: MonadState<State = StateTypesStateT<MA>, Pointed = A>,
    W: Monoid,
    A: Term,
{
    fn state(f: impl FunctionT<Self::State, (Self::Pointed, Self::State)>) -> Self {
        Self::lift(MonadLoweredT::<MA, A, W>::state(f))
    }
}

// ContT impl
impl<MR, MA> StateTypes for ContT<MR, MA>
where
    MR: Pointed,
    MA: StateTypes,
{
    type State = MA::State;
}

impl<MR, MA> MonadGet for ContT<MR, MA>
where
    Self: MonadTrans<MA>,
    MR: Pointed,
    MA: Pointed + MonadGet,
{
    fn get() -> Self {
        ContT::lift(MA::get())
    }
}

impl<MR, MA> MonadPut for ContT<MR, MA>
where
    Self: MonadTrans<MA>,
    MR: Pointed + Monoid,
    MA: Pointed + MonadPut,
{
    fn put(s: Self::State) -> Self {
        Self::lift(MA::put(s))
    }
}

impl<MA, MR> MonadState for ContT<MR, MA>
where
    Self: MonadTrans<MA>,
    MR: Pointed + Monoid,
    MA: MonadState + Pointed,
{
    fn state(f: impl FunctionT<MA::State, (MA::Pointed, MA::State)>) -> Self {
        Self::lift(MA::state(f))
    }
}

// Support functions
pub trait Modify<S, MA>: Term
where
    MA: WithPointed<((), S)>,
    S: Term,
{
    /// Monadic state transformer.
    ///
    /// Maps an old state to a new state inside a state monad. The old state is thrown away.
    ///
    /// Main :t modify ((+1) :: Int -> Int)
    /// modify (...) :: (MonadState Int a) => a ()
    ///
    /// This says that modify (+1) acts over any Monad that is a member of the MonadState class, with an Int state.
    fn modify(f: impl FunctionT<S, S>) -> StateT<S, MA::WithPointed>;
}

impl<T, S, MA> Modify<S, MA> for T
where
    MA: WithPointed<((), S)>,
    MA::WithPointed: ReturnM,
    S: Term,
    T: Term,
{
    fn modify(f: impl FunctionT<S, S>) -> StateT<S, MA::WithPointed> {
        let f = f.to_function();
        StateT::new(|s| ((), f(s)))
    }
}

pub trait Gets<S, A>: Term
where
    S: Term,
    A: Term,
{
    /// Gets specific component of the state, using a projection function supplied.
    fn gets(f: impl FunctionT<S, A>) -> Self;
}

impl<T, S, A> Gets<S, A> for T
where
    StateT<S, S>: ChainM<Self, Pointed = S>,
    S: ReturnM<Pointed = (S, S)>,
    T: ReturnM<Pointed = A>,
    A: Term,
{
    fn gets(f: impl FunctionT<S, A>) -> Self {
        let f = f.to_function();
        StateT::<S, S>::get().chain_m(|s| ReturnM::return_m(f(s)))
    }
}
