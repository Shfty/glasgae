//! Stateful logger monad.
//!
//! Abstracts over [`StateT`] and [`LoggingT`],
//! transparently lifting the message type into `(Message, State)`
//! for provision to custom logging implementations.

extern crate self as glasgae;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::{function::bifunction::BifunT, tuple::pair::Pair},
    },
    prelude::*,
    transformers::{
        class::MonadTrans,
        state::{HoistStateT, StateT},
    },
};

use super::{LoggingT, MonadLogger};

// Base Monads
// -----------------------------------------------------------------------------
pub type StateLoggerT<LVL, MSG, S, MA> = StateT<S, LoggingT<LVL, (MSG, S), MA>>;

// Newtype
// -----------------------------------------------------------------------------
pub type HoistStateLoggingT<LVL, MSG, S, MA> = HoistStateT<S, LoggingT<LVL, (MSG, S), MA>>;

#[derive(Clone)]
pub struct StateLogger<LVL, MSG, S, MA>(StateLoggerT<LVL, MSG, S, MA>)
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Term;

// Generalized impl
// -----------------------------------------------------------------------------
impl<LVL, MSG, S, MA> StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Term,
{
    pub fn new_t(m: StateLoggerT<LVL, MSG, S, MA>) -> Self {
        StateLogger(m)
    }

    pub fn run_t(self) -> StateLoggerT<LVL, MSG, S, MA> {
        self.0
    }

    pub fn run<A>(self, f: impl BifunT<LVL, (MSG, S), IO<()>>) -> MA::Chained
    where
        S: Default,
        MA: ChainM<A, Pointed = (A, S)>,
        MA::Chained: ReturnM<Pointed = A>,
        A: Term,
    {
        let f = f.to_bifun();
        self.run_t()
            .run_t(Default::default())
            .run_t(f)
            .chain_m(Pair::fst.compose_clone(ReturnM::return_m))
    }

    pub fn map_t<MB>(
        self,
        f: impl FunctionT<StateLoggerT<LVL, MSG, S, MA>, StateLoggerT<LVL, MSG, S, MB>>,
    ) -> StateLogger<LVL, MSG, S, MB>
    where
        MB: Term,
    {
        StateLogger::new_t(f(self.run_t()))
    }

    pub fn map_state<MB>(
        self,
        f: impl FunctionT<StateLoggerT<LVL, MSG, S, MA>, StateLoggerT<LVL, MSG, S, MB>>,
    ) -> StateLogger<LVL, MSG, S, MB>
    where
        MB: Term,
    {
        self.map_t(f)
    }

    pub fn map_logger<MB>(
        self,
        f: impl FunctionT<LoggingT<LVL, (MSG, S), MA>, LoggingT<LVL, (MSG, S), MB>>,
    ) -> StateLogger<LVL, MSG, S, MB>
    where
        MB: Term,
    {
        let f = f.to_function();
        self.map_state(|t| t.map_t(f))
    }

    pub fn map_inner<MB>(self, f: impl FunctionT<MA, MB>) -> StateLogger<LVL, MSG, S, MB>
    where
        MB: Term,
    {
        let f = f.to_function();
        self.map_logger(|t| t.map_t(f))
    }

    pub fn lift_t<A, MB>(m: MB) -> StateLogger<LVL, MSG, S, MA>
    where
        LVL: Term,
        MSG: Term,
        S: Term,
        MB: Monad<(A, S), Pointed = A, Chained = MA>,
        MA: Monad<A, Pointed = (A, S), Chained = MB>,
        A: Term,
        (A, S): Lower<A, S, Lowered = A>,
    {
        StateLogger::new_t(StateT::lift(LoggingT::lift(m)))
    }
}

// Specialized impl
// -----------------------------------------------------------------------------
impl<LVL, MSG, MA> StateLogger<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<usize, Pointed = ((), usize)>,
    MA::Chained: Monad<((), usize), Chained = MA>,
{
    pub fn indent() -> StateLogger<LVL, MSG, usize, MA> {
        StateLogger(StateLoggerT::<LVL, MSG, usize, MA>::modify_m(|s| {
            LoggingT::<LVL, (MSG, usize), MA::Chained>::return_m(s + 1)
        }))
    }

    pub fn unindent() -> Self {
        StateLogger(StateLoggerT::<LVL, MSG, usize, MA>::modify_m(|s| {
            LoggingT::<LVL, (MSG, usize), MA::Chained>::return_m(s - 1)
        }))
    }
}

// Basic typeclasses
// -----------------------------------------------------------------------------
impl<LVL, MSG, S, MA> Pointed for StateLogger<LVL, MSG, S, MA>
where
    StateLoggerT<LVL, MSG, S, MA>: Pointed,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    type Pointed = PointedT<StateLoggerT<LVL, MSG, S, MA>>;
}

impl<LVL, MSG, S, MA, A, B> WithPointed<B> for StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: WithPointed<(B, S), Pointed = (A, S)>,
    A: Term,
    B: Term,
{
    type WithPointed = StateLogger<LVL, MSG, S, MA::WithPointed>;
}

impl<LVL, MSG, S, MA, A, MB, B> Functor<B> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggerT<LVL, MSG, S, MA>:
        Functor<B, Pointed = A, Mapped = StateLoggerT<LVL, MSG, S, MB>>,
    StateLoggerT<LVL, MSG, S, MB>:
        Functor<A, Pointed = B, Mapped = StateLoggerT<LVL, MSG, S, MA>>,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Functor<(B, S), Pointed = (A, S), Mapped = MB>,
    MB: Functor<(A, S), Pointed = (B, S), Mapped = MA>,
    A: Term,
    B: Term,
{
    type Mapped = StateLogger<LVL, MSG, S, MB>;

    fn fmap(self, f: impl FunctionT<Self::Pointed, B>) -> Self::Mapped {
        StateLogger::new_t(self.run_t().fmap(f))
    }
}

impl<LVL, MSG, S, MA> PureA for StateLogger<LVL, MSG, S, MA>
where
    StateLoggerT<LVL, MSG, S, MA>: PureA,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    fn pure_a(t: Self::Pointed) -> Self {
        StateLogger::new_t(PureA::pure_a(t))
    }
}

impl<LVL, MSG, S, MF, F, MA, A, MB, B> AppA<A, B> for StateLogger<LVL, MSG, S, MF>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MF: Applicative<A, B, WithA = MA, WithB = MB>
        + Pointed<Pointed = (F, S)>
        + Monad<(A, S), Chained = MA>
        + Monad<(B, S), Chained = MB>,
    MA: Monad<(F, S), Pointed = (A, S), Chained = MF> + Monad<(B, S), Chained = MB>,
    MB: Monad<(F, S), Pointed = (B, S), Chained = MF> + Monad<(A, S), Chained = MA>,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = StateLogger<LVL, MSG, S, MA>;
    type WithB = StateLogger<LVL, MSG, S, MB>;

    fn app_a(self, a: Self::WithA) -> Self::WithB {
        StateLogger::new_t(self.run_t().app_a(a.run_t()))
    }
}

impl<LVL, MSG, S, MA> ReturnM for StateLogger<LVL, MSG, S, MA>
where
    StateLoggerT<LVL, MSG, S, MA>: ReturnM,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    fn return_m(t: Self::Pointed) -> Self {
        StateLogger::new_t(ReturnM::return_m(t))
    }
}

impl<LVL, MSG, S, MA, A, MB, B> ChainM<B> for StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<(B, S), Pointed = (A, S), Chained = MB>,
    MB: Monad<(A, S), Pointed = (B, S), Chained = MA>,
    S: Term,
    A: Term,
    B: Term,
{
    type Chained = StateLogger<LVL, MSG, S, MA::Chained>;

    fn chain_m(self, f: impl FunctionT<Self::Pointed, Self::Chained>) -> Self::Chained {
        let f = f.to_function();
        StateLogger::new_t(self.run_t().chain_m(|t| f(t).run_t()))
    }
}

// Lifting
// -----------------------------------------------------------------------------
impl<LVL, MSG, S, MA> MonadTrans<StateT<S, LoggingT<LVL, (MSG, S), MA>>>
    for StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Term,
    S: Term,
{
    fn lift(m: StateLoggerT<LVL, MSG, S, MA>) -> Self {
        StateLogger::new_t(m)
    }
}

impl<LVL, MSG, S, MA> MonadTrans<LoggingT<LVL, (MSG, S), MA>> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggerT<LVL, MSG, S, MA>: MonadTrans<LoggingT<LVL, (MSG, S), MA>>,
    LVL: Term,
    MSG: Term,
    MA: Pointed,
    S: Term,
{
    fn lift(m: LoggingT<LVL, (MSG, S), MA>) -> Self {
        MonadTrans::lift(StateLoggerT::lift(m))
    }
}

impl<LVL, MSG, S, T> MonadTrans<IO<T>> for StateLogger<LVL, MSG, S, IO<(T, S)>>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    T: Pointed,
{
    fn lift(m: IO<T>) -> Self {
        StateLogger::lift_t(m)
    }
}

impl<LVL, MSG, S, T> MonadIO<T> for StateLogger<LVL, MSG, S, IO<(T, S)>>
where
    Self: MonadTrans<IO<T>>,
    LVL: Term,
    MSG: Term,
    S: Term,
    T: Term,
{
    fn lift_io(m: IO<T>) -> Self {
        MonadTrans::lift(m)
    }
}

// Logging
// -----------------------------------------------------------------------------
impl<LVL, MSG, S, MA> MonadLogger<LVL, MSG> for StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Monad<(S, S), Pointed = ((), S)> + Monad<()>,
    MA::Pointed: Lower<(), S, Lowered = ()>,
    ChainedT<MA, (S, S)>: ReturnM<Pointed = (S, S)>,
    ChainedT<MA, ()>: ReturnM<Pointed = ()> + MonadIO<()>,
{
    fn log(level: LVL, message: MSG) -> Self {
        StateLogger::new_t(
            StateT::get().chain_m(move |s| MonadTrans::lift(LoggingT::log(level, (message, s)))),
        )
    }
}

// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod test {
    extern crate self as glasgae;

    use glasgae_macros::_do;
    use log::Level;

    use crate::{
        logger::{
            indent_logger,
            rust_logger::{init_env_logger, rust_logger, RustLogger},
        },
        prelude::IO,
    };

    use super::StateLogger;

    #[test]
    fn test_state_logging() -> IO<()> {
        type IndentRustLogger<MSG, M> = StateLogger<Level, MSG, usize, M>;
        type S<T> = IndentRustLogger<&'static str, IO<T>>;
        _do! {
            init_env_logger();
            _do! {
                S::info("Start");
                S::indent();
                S::info("Middle");
                S::unindent();
                S::info("End")
            }
            .run(indent_logger(rust_logger))
        }
    }
}
