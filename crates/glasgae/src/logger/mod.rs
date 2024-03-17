use std::fmt::Display;

use log::Level;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::function::bifunction::{Bifun, BifunT},
    },
    prelude::*,
    transformers::class::MonadTrans,
};

use self::indent::Indent;

pub mod indent;
pub mod rust_logger;
pub mod state_logging;

#[derive(Clone)]
pub struct LoggingT<LVL, MSG, MA>(Function<Bifun<LVL, MSG, IO<()>>, MA>)
where
    LVL: Term,
    MSG: Term,
    MA: Term;

impl<LVL, MSG, MA> LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Pointed,
{
    pub fn new_t(f: impl FunctionT<Bifun<LVL, MSG, IO<()>>, MA>) -> Self {
        LoggingT(f.to_function())
    }

    pub fn run_t(self, f: impl BifunT<LVL, MSG, IO<()>>) -> MA {
        self.0(f.boxed())
    }

    pub fn map_t<MB>(self, f: impl FunctionT<MA, MB>) -> LoggingT<LVL, MSG, MB>
    where
        LVL: Term,
        MSG: Term,
        MA: Term,
        MB: Pointed,
    {
        let f = f.to_function();
        LoggingT::new_t(|g| f(self.run_t(g)))
    }

    pub fn log(level: LVL, message: MSG) -> Self
    where
        LVL: Term,
        MSG: Term,
        MA: MonadIO<()>,
    {
        LoggingT::new_t(move |f| MonadIO::lift_io(f(level, message)))
    }
}

impl<LVL, MSG, MA> Pointed for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<LVL, MSG, MA, B> WithPointed<B> for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: WithPointed<B>,
{
    type WithPointed = LoggingT<LVL, MSG, MA::WithPointed>;
}

impl<LVL, MSG, MA, A, B> Fmap<B> for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Fmap<B, Pointed = A>,
    A: Term,
    B: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, B>) -> Self::WithPointed {
        let f = f.to_function();
        LoggingT::new_t(|g| self.run_t(g).fmap(f))
    }
}

impl<LVL, MSG, MA> PureA for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: PureA,
{
    fn pure_a(t: Self::Pointed) -> Self {
        LoggingT::new_t(r#const(PureA::pure_a(t)))
    }
}

impl<LVL, MSG, MF, MA, MB> AppA<LoggingT<LVL, MSG, MA>, LoggingT<LVL, MSG, MB>>
    for LoggingT<LVL, MSG, MF>
where
    LVL: Term,
    MSG: Term,
    MF: AppA<MA, MB>,
    MA: Pointed,
    MB: Pointed,
{
    fn app_a(self, log_a: LoggingT<LVL, MSG, MA>) -> LoggingT<LVL, MSG, MB> {
        let log_f = self;
        LoggingT::new_t(|f| log_f.run_t(f.clone()).app_a(log_a.run_t(f)))
    }
}

impl<LVL, MSG, MA> ReturnM for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: ReturnM,
{
    fn return_m(t: Self::Pointed) -> Self
    where
        Self: Sized,
    {
        LoggingT::new_t(r#const(ReturnM::return_m(t)))
    }
}

impl<LVL, MSG, MA, MB, A, B> ChainM<LoggingT<LVL, MSG, MB>> for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: ChainM<MB, Pointed = A>,
    MB: Pointed<Pointed = B>,
    A: Term,
{
    fn chain_m(self, f: impl FunctionT<A, LoggingT<LVL, MSG, MB>>) -> LoggingT<LVL, MSG, MB> {
        let f = f.to_function();
        LoggingT::new_t(|r| self.0(r.clone()).chain_m(|a| f(a).0(r)))
    }
}

impl<LVL, MSG, MA> MonadTrans<MA> for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: Pointed,
{
    fn lift(m: MA) -> Self {
        LoggingT::new_t(r#const(m))
    }
}

impl<LVL, MSG, MA, A> MonadIO<A> for LoggingT<LVL, MSG, MA>
where
    Self: MonadTrans<IO<A>>,
    LVL: Term,
    MSG: Term,
    MA: MonadIO<A>,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}

pub trait MonadLogger<LVL, MSG>: Term {
    fn log(level: LVL, message: MSG) -> Self;
}

impl<LVL, MSG, MA> MonadLogger<LVL, MSG> for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: MonadIO<()>,
{
    fn log(level: LVL, message: MSG) -> Self {
        Self::log(level, message)
    }
}

pub trait MonadLoggerIO: Term {
    fn ask() -> Self;
}

impl<LVL, MSG, MA> MonadLoggerIO for LoggingT<LVL, MSG, MA>
where
    LVL: Term,
    MSG: Term,
    MA: ReturnM<Pointed = Bifun<LVL, MSG, IO<()>>>,
{
    fn ask() -> Self {
        LoggingT::new_t(ReturnM::return_m)
    }
}

pub fn print_logger(level: Level, message: impl Term + Display) -> IO<()> {
    print(format!("[{level}] {message}"))
}

pub fn indent_logger<T>(
    f: impl BifunT<Level, Indent<T>, IO<()>>,
) -> impl BifunT<Level, (T, usize), IO<()>>
where
    T: Term,
{
    let f = f.to_bifun();
    |level, (message, depth)| f(level, Indent::new(message, depth))
}

#[cfg(test)]
mod test {

    use log::Level;

    use crate::{base::control::monad::io::MonadIO, prelude::*, transformers::class::MonadTrans};

    use super::{
        indent_logger, print_logger,
        rust_logger::rust_logger,
        state_logging::{LogScope, RunStateLogging, StateLogging},
        LoggingT, MonadLogger,
    };

    #[test]
    fn test_monad_logger() -> IO<()> {
        LoggingT::<Level, &str, IO<()>>::return_m(())
            .then_m(MonadLogger::log(Level::Trace, "sssh"))
            .then_m(MonadLogger::log(Level::Debug, "hey listen"))
            .then_m(MonadLogger::log(Level::Info, "hey alright"))
            .then_m(MonadLogger::log(Level::Warn, "uh oh"))
            .then_m(MonadLogger::log(Level::Error, "OH NO"))
            .run_t(print_logger)
    }

    #[test]
    fn test_monad_logger_state() -> IO<()> {
        StateLogging::lift(MonadIO::lift_io(IO::<()>::new(env_logger::init)))
            .then_m(StateLogging::log(Level::Trace, "hmm..."))
            .then_m(
                StateLogging::log_scope(
                    StateLogging::log(Level::Debug, "hmm..?")
                        .then_m(StateLogging::log_scope(StateLogging::log(
                            Level::Info,
                            "hmm?",
                        )))
                        .then_m(StateLogging::log(Level::Warn, "ah!")),
                )
                .then_m(StateLogging::log(Level::Error, "aha!")),
            )
            .run(indent_logger(rust_logger))
    }
}
