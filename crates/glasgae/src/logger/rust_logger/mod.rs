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
    fn trace(message: MSG) -> Self {
        Self::log(Level::Trace, message)
    }

    fn debug(message: MSG) -> Self {
        Self::log(Level::Debug, message)
    }

    fn info(message: MSG) -> Self {
        Self::log(Level::Info, message)
    }

    fn warn(message: MSG) -> Self {
        Self::log(Level::Warn, message)
    }

    fn error(message: MSG) -> Self {
        Self::log(Level::Error, message)
    }
}

impl<MSG, T> RustLogger<MSG> for T where T: MonadLogger<Level, MSG> {}
