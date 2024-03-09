//! Monadic I/O operations.

use std::{
    io::{ErrorKind, Read, Write},
    panic::UnwindSafe,
    path::Path,
    process::Termination,
};

use crate::{
    base::{
        control::exception::{throw, Exception},
        data::function::{Nullary, NullaryT},
    },
    prelude::{FunctionT, *},
};

/// A value of type [`IO<A>`] is a computation which, when performed,
/// interacts with the outside world before returning a value of type `A`.
///
/// In practice, this boils down to executing a closure (specifically, a [`NullaryT`])
/// which performs some action that would not be representible in pure functional code,
/// such as interacting with stdio, performing FFI, or interfacing with a hardware peripheral.
///
/// # Usage
///
/// [`IO`] is a monad, so actions can be combined
/// using the [`ThenM::then_m`] and [`ChainM::chain_m`] operations
/// from the [`monad`](crate::base::control::monad) module.
///
/// In addition, [`IO<A>`] derives a [`Termination`] implementation from the `A`,
/// allowing it to be returned from `main` in order to lift the whole program
/// into a pure functional environment:
/// ```
/// use glasgae::prelude::{IO, print};
///
/// fn main() -> IO<()> {
///     print("Hello, Pure Functional World!")
/// }
/// ```
///
/// # Purity
///
/// Unlike pure functional languages where side effects are external to the program by necessity,
/// Rust is free to perform them from an arbitrary function.
///
/// Purity can be thought of as a parallel to the `unsafe` semantic in regular Rust;
/// pure functions may only call other pure functions, and given some specific input,
/// must always produce the same output.
///
/// As such, care should be taken to respect the boundaries between pure and impure code.
///
/// (For more details, see the documentation of [`IO::run`].)
#[derive(Clone)]
pub struct IO<A>(Nullary<A>)
where
    A: 'static;

impl<T> IO<T>
where
    T: UnwindSafe,
{
    /// Construct a new I/O action from a nullary function.
    pub fn new(f: impl NullaryT<T> + Clone) -> Self {
        IO(f.boxed())
    }

    /// Run the I/O action, returning a value of type `T`.
    ///
    /// # Safety
    ///
    /// Intrinsically, this implements no functionality that would be
    /// considered 'unsafe' in regular Rust; it simply calls the underlying [`Nullary`].
    ///
    /// However, it has been marked as such to better illustrate the
    /// semantic boundary between pure and impure code;
    /// since the underlying calculation is impure, it should only
    /// be invoked from call sites that are themselves
    /// considered impure - i.e. ones outside the pure functional environment.
    ///
    /// ```
    /// # use glasgae::prelude::{IO, print};
    /// // This function - being a regular Rust function - is considered
    /// // semantically impure, by virtue of access to
    /// // the language's full range of imperative power.
    /// //
    /// // Indeed, it takes no input and returns no output,
    /// // so in functional terms, should do nothing at all.
    /// fn impure() {
    ///     // Here, we execute a pure function to
    ///     // produce an I/O computation that
    ///     // can be run in our impure environment.
    ///     let io: IO<()> = pure();
    ///
    ///     // Since this part of the program is impure,
    ///     // it's safe to call IO::run and collapse
    ///     // the computation into a final result,
    ///     // producing output as a side-effect.
    ///     unsafe { io.run() }
    /// }
    ///
    /// // This function technically has all of the
    /// // same computational powers as the one above,
    /// // but is rendered semantically pure
    /// // by not - directly or indirectly - calling IO::run
    /// // or other methods that may result in impure side-effects.
    /// fn pure() -> IO<()> {
    ///     // Return an IO action which,
    ///     // when run outside of the pure environment,
    ///     // will print some text to standard output.
    ///     print("Hello, Pure Functional World!")
    /// }
    /// ```
    pub unsafe fn run(self) -> T {
        self.0()
    }
}

impl<T> Termination for IO<T>
where
    T: UnwindSafe + Termination,
{
    fn report(self) -> std::process::ExitCode {
        let out = unsafe { self.run() };
        out.report()
    }
}

impl<T> Pointed for IO<T> {
    type Pointed = T;
}

impl<T, U> WithPointed<U> for IO<T>
where
    U: 'static,
{
    type WithPointed = IO<U>;
}

impl<T, U> Functor<U> for IO<T>
where
    T: Clone + UnwindSafe,
    U: 'static + Clone + UnwindSafe,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, U> + Clone) -> Self::WithPointed {
        IO::new(|| f(unsafe { self.run() }))
    }
}

impl<T> PureA for IO<T>
where
    T: Clone + UnwindSafe,
{
    fn pure_a(t: Self::Pointed) -> Self {
        IO::new(|| t)
    }
}

impl<F, A, B> AppA<IO<A>, IO<B>> for IO<F>
where
    F: Clone + UnwindSafe + FunctionT<A, B>,
    A: Clone + UnwindSafe,
    B: Clone + UnwindSafe,
{
    fn app_a(self, a: IO<A>) -> IO<B> {
        IO::new(|| unsafe { self.run()(a.run()) })
    }
}

impl<T> ReturnM for IO<T> where T: UnwindSafe + Clone {}

impl<T, U> ChainM<IO<U>> for IO<T>
where
    T: Clone + UnwindSafe,
    U: UnwindSafe,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, IO<U>> + Clone) -> IO<U> {
        IO::new(|| unsafe { f(self.run()).run() })
    }
}

impl<T> Semigroup for IO<T>
where
    T: Clone + UnwindSafe + Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        IO::new(|| unsafe { self.run().assoc_s(a.run()) })
    }
}

impl<T> Monoid for IO<T>
where
    T: Clone + UnwindSafe + Monoid,
{
    fn mempty() -> Self {
        IO::new(|| T::mempty())
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.foldr(Semigroup::assoc_s, Monoid::mempty())
    }
}

/// Within the [`IO`] monad, Unwrap the [`Right`] variant of [`Either<E, T>`],
/// or [`throw`] its [`Left`] variant as an exception.
pub fn unwrap_either<E, T>(t: Either<E, T>) -> IO<T>
where
    E: Clone + UnwindSafe + Exception,
    T: Clone + UnwindSafe,
{
    match t {
        Left(e) => throw(e),
        Right(t) => ReturnM::return_m(t),
    }
}

pub fn put_char(c: char) -> IO<()> {
    IO::new(move || print!("{c}"))
}

pub fn put_str(t: String) -> IO<()> {
    IO::new(move || print!("{t}"))
}

pub fn put_str_ln(t: String) -> IO<()> {
    IO::new(move || println!("{t}"))
}

pub fn print(t: impl Show + Clone + UnwindSafe + 'static) -> IO<()> {
    IO::new(move || println!("{}", t.show()))
}

pub fn try_get_char() -> IO<Either<ErrorKind, char>> {
    IO::new(|| {
        let mut buf = [0; 1];
        match std::io::stdin().read_exact(&mut buf) {
            Ok(_) => Right(buf[0] as char),
            Err(e) => Left(e.kind()),
        }
    })
}

pub fn get_char() -> IO<char> {
    try_get_char().chain_m(unwrap_either)
}

pub fn try_get_line() -> IO<Either<ErrorKind, String>> {
    IO::new(|| {
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => Right(buf),
            Err(e) => Left(e.kind()),
        }
    })
}

pub fn get_line() -> IO<String> {
    try_get_line().chain_m(unwrap_either)
}

pub fn try_get_contents() -> IO<Either<ErrorKind, String>> {
    IO::new(|| {
        let mut buf = String::new();
        match std::io::stdin().read_to_string(&mut buf) {
            Ok(_) => Right(buf),
            Err(e) => Left(e.kind()),
        }
    })
}

pub fn get_contents() -> IO<String> {
    try_get_contents().chain_m(unwrap_either)
}

pub fn try_read_file(
    path: impl AsRef<Path> + Clone + UnwindSafe + 'static,
) -> IO<Either<ErrorKind, String>> {
    IO::new(move || match std::fs::read_to_string(path) {
        Ok(t) => Right(t),
        Err(e) => Left(e.kind()),
    })
}

pub fn read_file(path: impl AsRef<Path> + Clone + UnwindSafe + 'static) -> IO<String> {
    try_read_file(path).chain_m(unwrap_either)
}

pub fn try_write_file(
    path: impl AsRef<Path> + Clone + UnwindSafe + 'static,
    string: impl AsRef<[u8]> + Clone + UnwindSafe + 'static,
) -> IO<Either<ErrorKind, ()>> {
    IO::new(move || std::fs::write(path, string).map_err(|e| e.kind()).into())
}

pub fn write_file(
    path: impl AsRef<Path> + Clone + UnwindSafe + 'static,
    string: impl AsRef<[u8]> + Clone + UnwindSafe + 'static,
) -> IO<()> {
    try_write_file(path, string).chain_m(unwrap_either)
}

pub fn try_append_file(
    path: impl AsRef<Path> + Clone + UnwindSafe + 'static,
    string: impl AsRef<[u8]> + Clone + UnwindSafe + 'static,
) -> IO<Either<ErrorKind, ()>> {
    IO::new(move || {
        std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .and_then(|mut file| file.write_all(string.as_ref()))
            .map_err(|e| e.kind())
            .into()
    })
}

pub fn append_file(
    path: impl AsRef<Path> + Clone + UnwindSafe + 'static,
    string: impl AsRef<[u8]> + Clone + UnwindSafe + 'static,
) -> IO<()> {
    try_append_file(path, string).chain_m(unwrap_either)
}

pub fn interact(f: impl FunctionT<String, String> + Clone) -> IO<String> {
    get_contents().fmap(f)
}
