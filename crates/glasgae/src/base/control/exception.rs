use std::{
    any::Any,
    panic::{catch_unwind, resume_unwind},
};

use crate::{base::data::function::Term, prelude::*};

pub trait Exception: Sized + Any + Send + Show {
    fn to_exception(self) -> SomeException {
        Box::new(self)
    }

    fn from_exception(b: SomeException) -> Result<Self, SomeException> {
        b.downcast::<Self>().map(|a| *a)
    }

    fn display_exception(self) -> String {
        self.show()
    }
}

/// The SomeException type is the root of the exception type hierarchy.
///
/// When an exception of type e is thrown, behind the scenes it is encapsulated in a SomeException.
pub type SomeException = Box<dyn Any + Send>;

impl Show for SomeException {
    fn show(self) -> String {
        "SomeException".to_string()
    }
}

impl Exception for SomeException {
    fn to_exception(self) -> SomeException {
        self
    }

    fn from_exception(b: SomeException) -> Result<Self, SomeException> {
        Ok(b)
    }
}

/// [`String`] is the underlying type of exceptions thrown by [`panic!()`]
impl Exception for String {}

/// [`std::io::ErrorKind`] is thrown in lieu of the unwind-unsafe [`std::io::Error`] type.
impl Exception for std::io::ErrorKind {}

/// Throw an exception inside the IO monad
pub fn throw<T, E>(e: E) -> IO<T>
where
    T: Term,
    E: Term + Exception,
{
    IO::new(move || std::panic::panic_any(e))
}

/// Catch an exception inside the IO monad
pub fn catch<E, T>(io: IO<T>, handler: impl FunctionT<E, IO<T>>) -> IO<T>
where
    T: Term,
    E: Term + Exception,
{
    let handler = handler.to_function();
    IO::new(move || match catch_unwind(move || unsafe { io.run() }) {
        Ok(t) => t,
        Err(a) => match E::from_exception(a) {
            Ok(e) => unsafe { handler(e).run() },
            Err(a) => resume_unwind(a),
        },
    })
}

/// [`catch`] with the arguments swapped around.
///
/// Useful in situations where the code for the handler is shorter.
pub fn handle<E, T>(handler: impl FunctionT<E, IO<T>>, io: IO<T>) -> IO<T>
where
    T: Term,
    E: Term + Exception,
{
    catch(io, handler)
}

/// Similar to [`catch`], but returns an Either result which is [`Right(a)`] if no exception of type e was raised,
/// or [`Left(ex)`] if an exception of type `e` was raised and its value is `ex`.
///
/// If any other type of exception is raised then it will be propagated
/// up to the next enclosing exception handler.
/// ```
/// r#try(a) == catch(Right.lift_m(a), Left::compose_clone(ReturnM::return_m))
/// ```
pub fn r#try<E, T>(io: IO<T>) -> IO<Either<E, T>>
where
    T: Term,
    E: Term,
{
    IO::new(move || match catch_unwind(move || unsafe { io.run() }) {
        Ok(t) => Right(t),
        Err(a) => match a.downcast::<E>() {
            Ok(e) => Left(*e),
            Err(a) => resume_unwind(a),
        },
    })
}
