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
