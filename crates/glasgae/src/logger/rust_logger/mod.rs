use crate::{logger::MonadLogger, prelude::Term, prelude::IO};
use log::Level;
use std::fmt::Display;

pub fn init_env_logger() -> IO<()> {
    IO::<()>::new(env_logger::init)
}

pub fn rust_logger(level: Level, message: impl Term + Display) -> IO<()> {
    IO::new(move || log::log!(level, "{}", message))
}

pub trait RustLogger<MSG>: MonadLogger<Level, MSG> {
    fn trace(message: impl Into<MSG>) -> Self {
        Self::log(Level::Trace, message.into())
    }

    fn debug(message: impl Into<MSG>) -> Self {
        Self::log(Level::Debug, message.into())
    }

    fn info(message: impl Into<MSG>) -> Self {
        Self::log(Level::Info, message.into())
    }

    fn warn(message: impl Into<MSG>) -> Self {
        Self::log(Level::Warn, message.into())
    }

    fn error(message: impl Into<MSG>) -> Self {
        Self::log(Level::Error, message.into())
    }
}

impl<MSG, T> RustLogger<MSG> for T where T: MonadLogger<Level, MSG> {}
