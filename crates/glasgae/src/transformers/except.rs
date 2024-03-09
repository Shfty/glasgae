//! This monad transformer extends a monad with the ability to throw exceptions.
//!
//! A sequence of actions terminates normally, producing a value, only if none of the actions in the sequence throws an exception. If one throws an exception, the rest of the sequence is skipped and the composite action exits with that exception.
//!
//! If the value of the exception is not required, the variant in Control.Monad.Trans.Maybe may be used instead.

use std::panic::UnwindSafe;

use crate::{
    base::{
        control::monad::{io::MonadIO, LiftM},
        data::{functor::identity::Identity, FoldMap},
    },
    prelude::{
        AppA, ChainM, Either, Either::*, Foldr, FunctionT, Functor, Monoid, Pointed, PureA,
        ReturnM, SequenceA, ThenM, TraverseT, WithPointed, IO,
    },
};

use super::class::MonadTrans;

/// The parameterizable exception monad.
///
/// Computations are either exceptions or normal values.
///
/// The [`ReturnM::return_m`] function returns a normal value,
/// while [`ChainM::chain_m`] exits on the first exception.
///
/// For a variant that continues after an error and collects all the errors, see
/// [`Errors`](crate::base::control::applicative::errors::Errors).
pub type Except<E, A> = ExceptT<Identity<Either<E, A>>>;

impl<E, A> Except<E, A>
where
    E: UnwindSafe,
    A: UnwindSafe,
{
    /// Extractor for computations in the exception monad. (The inverse of [`ExceptT::new`]).
    pub fn run(self) -> Either<E, A> {
        self.run_t().run()
    }

    /// Map the unwrapped computation using the given function.
    /// ```text
    /// m.map(f).run() == f(m.run())
    /// ```
    pub fn map<B>(self, f: impl FunctionT<A, B> + Clone) -> Except<E, B>
    where
        E: Clone,
        B: Clone + UnwindSafe,
    {
        self.map_t(|t| {
            t.fmap(|t| match t {
                Left(e) => Left(e),
                Right(x) => Right(f(x)),
            })
        })
    }

    /// Transform any exceptions thrown by the computation using the given function
    /// (a specialization of [`ExceptT::with_t`]).
    pub fn with<E_>(self, f: impl FunctionT<E, E_> + Clone) -> Except<E_, A>
    where
        E_: Clone + UnwindSafe,
        E: Clone,
        A: Clone,
    {
        self.with_t(f)
    }
}

/// The parameterizable exception monad.
///
/// Computations are either exceptions or normal values.
///
/// The return function returns a normal value, while >>= exits on the first exception. For a variant that continues after an error and collects all the errors, see Errors.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExceptT<MA>(MA);

impl<MA> ExceptT<MA>
where
    MA: UnwindSafe,
{
    pub fn new_t(ma: MA) -> Self {
        ExceptT(ma)
    }

    /// Constructor for computations in the exception monad. (The inverse of [`Except::run`]).
    pub fn new<E, A>(either: Either<E, A>) -> Self
    where
        MA: ReturnM<Pointed = Either<E, A>>,
    {
        ExceptT(ReturnM::return_m(either))
    }

    /// The inverse of [`ExceptT::new_t`].
    pub fn run_t(self) -> MA {
        self.0
    }

    /// Map the unwrapped computation using the given function.
    /// ```text
    /// m.map(f).run() == f(m.run())
    /// ```
    pub fn map_t<MB>(self, f: impl FunctionT<MA, MB>) -> ExceptT<MB> {
        ExceptT(f(self.run_t()))
    }

    /// Transform any exceptions thrown by the computation using the given function.
    pub fn with_t<MB, E, E_, A>(self, f: impl FunctionT<E, E_> + Clone) -> ExceptT<MB>
    where
        MA: Functor<Either<E_, A>, Pointed = Either<E, A>, WithPointed = MB>,
        MB: Clone + Pointed<Pointed = Either<E_, A>>,
        E_: Clone + UnwindSafe,
        A: Clone + UnwindSafe,
    {
        self.map_t(|t| {
            t.fmap(|t| match t {
                Left(l) => Left(f(l)),
                Right(r) => Right(r),
            })
        })
    }

    /// Signal an exception value `e`.
    /// ```text
    /// ExceptT::throw(e).run() == ReturnM::return_m(Left(e))
    /// ```
    /// ```text
    /// ExceptT::throw(e).chain_m(m) == ExceptT::throw(e)
    /// ```
    pub fn throw<E, A>(e: E) -> Self
    where
        MA: ReturnM<Pointed = Either<E, A>>,
    {
        ExceptT(ReturnM::return_m(Left(e)))
    }

    /// Handle an exception.
    /// ```text
    /// ExceptT::lift(m).catch(h) == ExceptT::lift(m)
    /// ```
    /// ```text
    /// ExceptT::throw(e).catch(h) == h(e)
    /// ```
    pub fn catch<MB, E, E_, A>(self, h: impl FunctionT<E, ExceptT<MB>> + Clone) -> ExceptT<MB>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: UnwindSafe + ReturnM<Pointed = Either<E_, A>>,
    {
        let m = self;
        ExceptT(m.run_t().chain_m(|a| match a {
            Left(l) => h(l).run_t(),
            Right(r) => ReturnM::return_m(Right(r)),
        }))
    }

    /// The same as `ExceptT::catch.flip()`,
    /// which is useful in situations where the code for the handler is shorter.
    pub fn handle<MB, E, E_, A>(
        h: impl FunctionT<E, ExceptT<MB>> + Clone,
        this: Self,
    ) -> ExceptT<MB>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: UnwindSafe + ReturnM<Pointed = Either<E_, A>>,
    {
        this.catch(h)
    }

    /// Similar to [`ExceptT::catch`], but returns an [`Either`] result
    /// which is `Right(a)` if no exception was thown,
    /// or `Left(ex)` if an exception `ex` was thrown.
    pub fn r#try<MB, MC, E, A>(self) -> ExceptT<MC>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: UnwindSafe + ReturnM<Pointed = Either<E, Either<E, A>>> + ChainM<MC>,
        MC: UnwindSafe + ReturnM<Pointed = Either<E, Either<E, A>>>,
        E: 'static,
        A: 'static,
    {
        Right.lift_m()(self).catch(|t| ReturnM::return_m(Left(t)))
    }

    /// `a.finally(b)` executes computation `a` followed by computation `b`,
    /// even if `a` exits early by throwing an exception.
    ///
    /// In the latter case, the exception is re-thrown after `b` has been executed.
    pub fn finally<MB, MC, E, A>(self, closer: ExceptT<MC>) -> Self
    where
        MA: 'static + Clone + ChainM<MB, Pointed = Either<E, A>> + ReturnM,
        MB: Clone
            + UnwindSafe
            + ReturnM<Pointed = Either<E, Either<E, A>>>
            + ChainM<MB>
            + ChainM<MA>,
        MC: 'static + Clone + UnwindSafe + ChainM<MA, Pointed = Either<E, ()>>,
        E: 'static,
        A: 'static,
    {
        let m: ExceptT<MA> = self;
        let m: ExceptT<MB> = m.r#try();
        m.chain_m(|res| {
            closer.then_m(match res {
                Left(e) => ExceptT::throw(e),
                Right(x) => ReturnM::return_m(x),
            })
        })
    }
}

impl<MA, E, A> Pointed for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
{
    type Pointed = A;
}

impl<MA, E, A, B> WithPointed<B> for ExceptT<MA>
where
    MA: WithPointed<Either<E, B>, Pointed = Either<E, A>>,
    MA::WithPointed: Pointed<Pointed = Either<E, B>>,
{
    type WithPointed = ExceptT<MA::WithPointed>;
}

impl<MA, E, A, B> Functor<B> for ExceptT<MA>
where
    MA: UnwindSafe + Functor<Either<E, B>, Pointed = Either<E, A>>,
    E: Clone + UnwindSafe,
    A: Clone,
    B: Clone + UnwindSafe,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<A, B> + Clone) -> Self::WithPointed {
        ExceptT(self.run_t().fmap(|t| t.fmap(f)))
    }
}

impl<MA, E, A> PureA for ExceptT<MA>
where
    MA: PureA<Pointed = Either<E, A>>,
{
    fn pure_a(t: Self::Pointed) -> Self {
        ExceptT(PureA::pure_a(Right(t)))
    }
}

impl<MF, MA, MB, E, F, A, B> AppA<ExceptT<MA>, ExceptT<MB>> for ExceptT<MF>
where
    MF: ChainM<MB, Pointed = Either<E, F>>,
    MA: 'static + Clone + UnwindSafe + ChainM<MB, Pointed = Either<E, A>>,
    MB: ReturnM<Pointed = Either<E, B>>,
    F: FunctionT<A, B> + Clone,
{
    fn app_a(self, ExceptT(v): ExceptT<MA>) -> ExceptT<MB> {
        let ExceptT(f) = self;
        ExceptT(f.chain_m(|mf| match mf {
            Left(e) => ReturnM::return_m(Left(e)),
            Right(k) => v.chain_m(|mv| match mv {
                Left(e) => ReturnM::return_m(Left(e)),
                Right(x) => ReturnM::return_m(Right(k(x))),
            }),
        }))
    }
}

impl<MA, E, A> ReturnM for ExceptT<MA>
where
    MA: ReturnM<Pointed = Either<E, A>>,
{
    fn return_m(t: Self::Pointed) -> Self
    where
        Self: Sized,
    {
        ExceptT(ReturnM::return_m(Right(t)))
    }
}

impl<MA, MB, E, A, B> ChainM<ExceptT<MB>> for ExceptT<MA>
where
    MA: UnwindSafe + ChainM<MB, Pointed = Either<E, A>>,
    MB: UnwindSafe + ReturnM<Pointed = Either<E, B>>,
{
    fn chain_m(self, k: impl FunctionT<A, ExceptT<MB>> + Clone) -> ExceptT<MB> {
        let m = self;
        ExceptT(m.run_t().chain_m(|a| match a {
            Left(e) => ReturnM::return_m(Left(e)),
            Right(x) => k(x).run_t(),
        }))
    }
}

impl<MA, E, A, B> FoldMap<A, B> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B> + Clone) -> B {
        todo!()
    }
}

impl<MA, E, A, B> Foldr<A, B> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
{
    fn foldr(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<A, B, B> + Clone,
        z: B,
    ) -> B {
        todo!()
    }
}

impl<MA, A1, T, A2, E, A> TraverseT<A1, T, A2> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        todo!()
    }
}

impl<A1, T, A2, E, A> SequenceA<T, A2> for ExceptT<A1>
where
    A1: Pointed<Pointed = Either<E, A>>,
{
    fn sequence_a(self) -> A2 {
        todo!()
    }
}

impl<MA, MB, E, A> MonadTrans<MB> for ExceptT<MA>
where
    MA: UnwindSafe + ReturnM<Pointed = Either<E, A>>,
    MB: Pointed<Pointed = A> + ChainM<MA>,
{
    fn lift(m: MB) -> Self {
        ExceptT::new_t(m.chain_m(|t| ReturnM::return_m(Right(t))))
    }
}

trait LowerEither<E, A>: Pointed<Pointed = Either<E, A>> + WithPointed<A> {
    type Lowered: Pointed<Pointed = A>;
}

impl<T, E, A> LowerEither<E, A> for T
where
    T: Pointed<Pointed = Either<E, A>> + WithPointed<A>,
{
    type Lowered = T::WithPointed;
}

impl<MA, E, A> MonadIO<A> for ExceptT<MA>
where
    Self: MonadTrans<MA::Lowered>,
    MA: LowerEither<E, A> + Pointed<Pointed = Either<E, A>>,
    MA::Lowered: MonadIO<A>,
    A: 'static,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(<MA as LowerEither<E, A>>::Lowered::lift_io(m))
    }
}
