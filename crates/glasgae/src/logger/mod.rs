use std::fmt::Display;

use log::Level;

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::function::bifunction::{Bifun, BifunT},
    },
    prelude::{
        print, r#const, AppA, Boxed, ChainM, Function, FunctionT, Functor, Pointed, PureA, ReturnM,
        WithPointed, IO,
    },
    transformers::class::MonadTrans,
};

use self::indent::Indent;

pub mod indent;
pub mod state_logging;

#[derive(Clone)]
pub struct LoggingT<LVL, MSG, MA>(Function<Bifun<LVL, MSG, IO<()>>, MA>)
where
    LVL: 'static,
    MSG: 'static,
    MA: 'static;

impl<LVL, MSG, MA> LoggingT<LVL, MSG, MA>
where
    MA: Pointed,
{
    pub fn new_t(f: impl FunctionT<Bifun<LVL, MSG, IO<()>>, MA> + Clone) -> Self {
        LoggingT(f.boxed())
    }

    pub fn run_t(self, f: impl BifunT<LVL, MSG, IO<()>> + Clone) -> MA {
        self.0(f.boxed())
    }

    pub fn map_t<MB>(self, f: impl FunctionT<MA, MB> + Clone) -> LoggingT<LVL, MSG, MB>
    where
        LVL: Clone,
        MSG: Clone,
        MA: Clone,
        MB: Pointed,
    {
        LoggingT::new_t(|g| f(self.run_t(g)))
    }

    pub fn log(level: LVL, message: MSG) -> Self
    where
        LVL: Clone,
        MSG: Clone,
        MA: MonadIO<()>,
    {
        LoggingT::new_t(move |f| MonadIO::lift_io(f(level, message)))
    }
}

impl<LVL, MSG, MA> Pointed for LoggingT<LVL, MSG, MA>
where
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<LVL, MSG, MA, B> WithPointed<B> for LoggingT<LVL, MSG, MA>
where
    MA: WithPointed<B>,
    MA::WithPointed: 'static,
{
    type WithPointed = LoggingT<LVL, MSG, MA::WithPointed>;
}

impl<LVL, MSG, MA, A, B> Functor<B> for LoggingT<LVL, MSG, MA>
where
    LVL: Clone,
    MSG: Clone,
    MA: Clone + Functor<B, Pointed = A>,
    B: 'static + Clone,
{
    fn fmap(
        self,
        f: impl crate::prelude::FunctionT<Self::Pointed, B> + Clone,
    ) -> Self::WithPointed {
        LoggingT::new_t(|g| self.run_t(g).fmap(f))
    }
}

impl<LVL, MSG, MA> PureA for LoggingT<LVL, MSG, MA>
where
    MA: Clone + PureA,
    MA::Pointed: Clone,
{
    fn pure_a(t: Self::Pointed) -> Self {
        LoggingT::new_t(r#const(PureA::pure_a(t)))
    }
}

impl<LVL, MSG, MF, MA, MB> AppA<LoggingT<LVL, MSG, MA>, LoggingT<LVL, MSG, MB>>
    for LoggingT<LVL, MSG, MF>
where
    LVL: Clone,
    MSG: Clone,
    MF: Clone + Pointed + AppA<MA, MB>,
    MA: Clone + Pointed,
    MB: Pointed,
{
    fn app_a(self, log_a: LoggingT<LVL, MSG, MA>) -> LoggingT<LVL, MSG, MB> {
        let log_f = self;
        LoggingT::new_t(|f| log_f.run_t(f.clone()).app_a(log_a.run_t(f)))
    }
}

impl<LVL, MSG, MA> ReturnM for LoggingT<LVL, MSG, MA>
where
    MA: Clone + ReturnM,
    MA::Pointed: Clone,
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
    MA: ChainM<MB, Pointed = A>,
    MB: Pointed<Pointed = B>,
{
    fn chain_m(
        self,
        f: impl FunctionT<A, LoggingT<LVL, MSG, MB>> + Clone,
    ) -> LoggingT<LVL, MSG, MB> {
        LoggingT::new_t(|r| self.0(r.clone()).chain_m(|a| f(a).0(r)))
    }
}

impl<LVL, MSG, MA> MonadTrans<MA> for LoggingT<LVL, MSG, MA>
where
    MA: Clone + Pointed,
{
    fn lift(m: MA) -> Self {
        LoggingT::new_t(r#const(m))
    }
}

impl<LVL, MSG, MA, A> MonadIO<A> for LoggingT<LVL, MSG, MA>
where
    MA: MonadIO<A>,
    Self: MonadTrans<IO<A>>,
    A: 'static,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}

pub trait MonadLogger<LVL, MSG> {
    fn log(level: LVL, message: MSG) -> Self;
}

impl<LVL, MSG, MA> MonadLogger<LVL, MSG> for LoggingT<LVL, MSG, MA>
where
    LVL: Clone,
    MSG: Clone,
    MA: MonadIO<()>,
{
    fn log(level: LVL, message: MSG) -> Self {
        Self::log(level, message)
    }
}

pub trait MonadLoggerIO {
    fn ask() -> Self;
}

impl<LVL, MSG, MA> MonadLoggerIO for LoggingT<LVL, MSG, MA>
where
    MA: ReturnM<Pointed = Bifun<LVL, MSG, IO<()>>>,
{
    fn ask() -> Self {
        LoggingT::new_t(ReturnM::return_m)
    }
}

pub fn print_logger(level: Level, message: impl Display + Clone + 'static) -> IO<()> {
    print(format!("[{level}] {message}"))
}

pub fn rust_logger(level: Level, message: impl Display + Clone + 'static) -> IO<()> {
    IO::new(move || log::log!(level, "{}", message))
}

pub fn indent_logger<T>(
    f: impl BifunT<Level, Indent<T>, IO<()>> + Clone,
) -> impl BifunT<Level, (T, usize), IO<()>> + Clone {
    move |level, (message, depth)| f(level, Indent::new(message, depth))
}

#[cfg(test)]
mod test {

    use log::Level;

    use crate::{
        base::{control::monad::io::MonadIO, data::function::bifunction::BifunT},
        prelude::*,
        transformers::class::MonadTrans,
    };

    use super::{
        indent::Indent,
        indent_logger, print_logger, rust_logger,
        state_logging::{log_scope, RunStateLogging, StateLoggingT},
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
        StateLoggingT::lift(LoggingT::lift_io(IO::<()>::new(env_logger::init)))
            .then_m(StateLoggingT::log(Level::Trace, "hmm..."))
            .then_m(
                log_scope(
                    StateLoggingT::log(Level::Debug, "hmm..?")
                        .then_m(log_scope(StateLoggingT::log(Level::Info, "hmm?")))
                        .then_m(StateLoggingT::log(Level::Warn, "ah!")),
                )
                .then_m(StateLoggingT::log(Level::Error, "aha!")),
            )
            .run(indent_logger(rust_logger))
    }
}
