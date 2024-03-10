use std::panic::UnwindSafe;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::{
            function::{bifunction::BifunT, Term},
            tuple::pair::Pair,
        },
    },
    prelude::*,
    transformers::{class::MonadTrans, state::StateT},
};

use super::{LoggingT, MonadLogger};

pub type StateLogging<LVL, MSG, S, T> = StateLoggingT<LVL, MSG, S, IO<(T, S)>>;

pub type StateLoggingT<LVL, MSG, S, MA> = StateT<S, LoggingT<LVL, (MSG, S), MA>>;

impl<LVL, MSG, S, MA> MonadLogger<LVL, MSG> for StateLoggingT<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    S: Term,
    MA: ReturnM<Pointed = ((), S)> + WithPointed<(S, S)> + WithPointed<()>,
    WithPointedT<MA, (S, S)>: ReturnM<Pointed = (S, S)> + ChainM<MA>,
    WithPointedT<MA, ()>: MonadIO<()> + ChainM<MA>,
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

pub trait RunStateLogging<LVL, MSG, MA> {
    fn run(self, f: impl BifunT<LVL, MSG, IO<()>>) -> MA;
}

impl<LVL, MSG, MA, MB, S, T> RunStateLogging<LVL, (MSG, S), MB> for StateLoggingT<LVL, MSG, S, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Pointed<Pointed = (T, S)> + ChainM<MB>,
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

pub fn indent<LVL, MSG, MA>() -> StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: ReturnM<Pointed = usize> + ChainM<MA>,
{
    StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
        LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s + 1)
    })
}

pub fn unindent<LVL, MSG, MA>() -> StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Term,
    MSG: Term,
    MA: ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: ReturnM<Pointed = usize> + ChainM<MA>,
{
    StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
        LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s - 1)
    })
}

pub trait LogScope {
    fn log_scope(m: Self) -> Self;
}

impl<LVL, MSG, MA> LogScope for StateLoggingT<LVL, MSG, usize, MA>
where
    Self: Pointed + ThenM<StateLoggingT<LVL, MSG, usize, MA>>,
    PointedT<Self>: Term,
    LVL: Term,
    MSG: Term,
    MA: ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: ReturnM<Pointed = usize> + ChainM<MA>,
{
    fn log_scope(m: Self) -> Self {
        indent().then_m(m).then_m(unindent())
    }
}
