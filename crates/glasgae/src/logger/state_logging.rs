extern crate self as glasgae;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::{function::bifunction::BifunT, tuple::pair::Pair},
    },
    derive_functor_unary,
    prelude::*,
    transformers::{
        class::MonadTrans,
        state::{HoistStateT, StateT},
    },
};

use super::{LoggingT, MonadLogger};

pub type StateLoggingT<LVL, MSG, S, MA> = StateT<S, LoggingT<LVL, (MSG, S), MA>>;

pub type HoistStateLoggingT<LVL, MSG, S, MA> = HoistStateT<S, LoggingT<LVL, (MSG, S), MA>>;

impl<LVL, MSG, S, MA> MonadLogger<LVL, MSG> for StateLoggingT<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: ReturnM<Pointed = ((), S)> + WithPointed<(S, S)> + WithPointed<()>,
    WithPointedT<MA, (S, S)>: Monad<((), S), Pointed = (S, S), WithPointed = MA>,
    WithPointedT<MA, ()>: Monad<((), S), WithPointed = MA> + MonadIO<()>,
{
    fn log(level: LVL, message: MSG) -> Self {
        StateLoggingT::<LVL, MSG, S, WithPointedT<MA, (S, S)>>::get().chain_m(move |s| {
            MonadTrans::lift(LoggingT::<LVL, (MSG, S), WithPointedT<MA, ()>>::log(
                level,
                (message, s),
            ))
        })
    }
}

#[derive(Clone)]
pub struct StateLogger<LVL, MSG, S, MA>(StateLoggingT<LVL, MSG, S, MA>)
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Term;

impl<LVL, MSG, S, MA> StateLogger<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Term,
{
    pub fn new_t(m: StateLoggingT<LVL, MSG, S, MA>) -> Self {
        StateLogger(m)
    }

    pub fn run_t(self) -> StateLoggingT<LVL, MSG, S, MA> {
        self.0
    }

    pub fn run<A>(self, f: impl BifunT<LVL, (MSG, S), IO<()>>) -> MA::WithPointed
    where
        S: Default,
        MA: ChainM<A, Pointed = (A, S)>,
        MA::WithPointed: ReturnM<Pointed = A>,
        A: Term,
    {
        let f = f.to_bifun();
        self.run_t()
            .run_t(Default::default())
            .run_t(f)
            .chain_m(Pair::fst.compose_clone(ReturnM::return_m))
    }
}

impl<LVL, MSG, MA> StateLogger<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<usize, Pointed = ((), usize)>,
    MA::WithPointed: Monad<((), usize), WithPointed = MA>,
{
    pub fn indent() -> StateLogger<LVL, MSG, usize, MA> {
        StateLogger(StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
            LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s + 1)
        }))
    }

    pub fn unindent() -> Self {
        StateLogger(StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
            LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s - 1)
        }))
    }
}

impl<LVL, MSG, S, MA> Pointed for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: Pointed,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    type Pointed = PointedT<StateLoggingT<LVL, MSG, S, MA>>;
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
    StateLoggingT<LVL, MSG, S, MA>:
        Functor<B, Pointed = A, WithPointed = StateLoggingT<LVL, MSG, S, MB>>,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Monad<(B, S), Pointed = (A, S), WithPointed = MB>,
    MB: Term,
    A: Term,
    B: Term,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, B>) -> Self::WithPointed {
        StateLogger::new_t(self.run_t().fmap(f))
    }
}

impl<LVL, MSG, S, MA> PureA for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: PureA,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    fn pure_a(t: Self::Pointed) -> Self {
        StateLogger::new_t(PureA::pure_a(t))
    }
}

impl<LVL, MSG, S, MA, A1, A2> AppA<A1, A2> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: AppA<A1, A2>,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    fn app_a(self, a: A1) -> A2 {
        self.run_t().app_a(a)
    }
}

impl<LVL, MSG, S, MA> ReturnM for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: ReturnM,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Pointed,
{
    fn return_m(t: Self::Pointed) -> Self {
        StateLogger::new_t(ReturnM::return_m(t))
    }
}

impl<LVL, MSG, S, MA, A, B> ChainM<B> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>:
        Monad<B, Pointed = A, WithPointed = StateLoggingT<LVL, MSG, S, MA>>,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: WithPointed<(B, S), Pointed = (A, S), WithPointed = MA>,
    A: Term,
    B: Term,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Self::WithPointed>) -> Self::WithPointed {
        let f = f.to_function();
        StateLogger::new_t(self.run_t().chain_m(|t| f(t).run_t()))
    }
}

impl<LVL, MSG, S, MA, T> MonadIO<T> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: MonadIO<T>,
    LVL: Term,
    MSG: Term,
    MA: Pointed,
    S: Term,
    T: Term,
{
    fn lift_io(m: IO<T>) -> Self {
        Self::new_t(MonadIO::lift_io(m))
    }
}

impl<LVL, MSG, S, MA, T> MonadTrans<T> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: MonadTrans<T>,
    LVL: Term,
    MSG: Term,
    MA: Pointed,
    S: Term,
    T: Term,
{
    fn lift(m: T) -> Self {
        StateLogger::new_t(StateLoggingT::lift(m))
    }
}

impl<LVL, MSG, S, MA> MonadLogger<LVL, MSG> for StateLogger<LVL, MSG, S, MA>
where
    StateLoggingT<LVL, MSG, S, MA>: MonadLogger<LVL, MSG>,
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: Term,
{
    fn log(level: LVL, message: MSG) -> Self {
        StateLogger(StateLoggingT::log(level, message))
    }
}

pub trait RunStateLogging<LVL, MSG, MA>: Term {
    fn run(self, f: impl BifunT<LVL, MSG, IO<()>>) -> MA;
}

impl<LVL, MSG, MA, MB, S, T> RunStateLogging<LVL, (MSG, S), MB> for StateLoggingT<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<T, Pointed = (T, S), WithPointed = MB>,
    MB: ReturnM<Pointed = T>,
    S: Term + Default,
    T: Term,
{
    fn run(self, f: impl BifunT<LVL, (MSG, S), IO<()>>) -> MB {
        self.run_t(Default::default())
            .run_t(f)
            .chain_m(Pair::fst.compose_clone(ReturnM::return_m))
    }
}

pub trait Indent {
    fn indent() -> Self;
}

impl<LVL, MSG, MA> Indent for StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<usize, Pointed = ((), usize)>,
    MA::WithPointed: Monad<((), usize), Pointed = usize, WithPointed = MA>,
{
    fn indent() -> Self {
        StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
            LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s + 1)
        })
    }
}

pub trait Unindent {
    fn unindent() -> Self;
}

impl<LVL, MSG, MA> Unindent for StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Monad<usize, Pointed = ((), usize)>,
    MA::WithPointed: Monad<((), usize), Pointed = usize, WithPointed = MA>,
{
    fn unindent() -> Self {
        StateLoggingT::modify_m(|s| LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s - 1))
    }
}

pub trait LogScope: Term {
    fn log_scope(m: Self) -> Self;
}

impl<LVL, MSG, T> LogScope for StateLoggingT<LVL, MSG, usize, IO<(T, usize)>>
where
    LVL: Term,
    MSG: Term,
    T: Term,
{
    fn log_scope(
        m: StateLoggingT<LVL, MSG, usize, IO<(T, usize)>>,
    ) -> StateLoggingT<LVL, MSG, usize, IO<(T, usize)>> {
        _do! {
            StateLoggingT::<LVL, MSG, usize, IO<((), usize)>>::indent();
            out <- m;
            StateLoggingT::<LVL, MSG, usize, IO<((), usize)>>::unindent();
            StateLoggingT::<LVL, MSG, usize, IO<(T, usize)>>::return_m(out)
        }
    }
}

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
