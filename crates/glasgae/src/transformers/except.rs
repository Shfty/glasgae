//! This monad transformer extends a monad with the ability to throw exceptions.
//!
//! A sequence of actions terminates normally, producing a value, only if none of the actions in the sequence throws an exception. If one throws an exception, the rest of the sequence is skipped and the composite action exits with that exception.
//!
//! If the value of the exception is not required, the variant in Control.Monad.Trans.Maybe may be used instead.

use crate::{
    base::{
        control::monad::{io::MonadIO, morph::HoistEitherT, LiftM},
        data::{
            foldl1_default, foldr1_default, function::bifunction::BifunT,
            functor::identity::Identity, FoldMap, Foldable1,
        },
    },
    prelude::*,
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
    E: Term,
    A: Term,
{
    /// Extractor for computations in the exception monad. (The inverse of [`ExceptT::new`]).
    pub fn run(self) -> Either<E, A> {
        self.run_t().run()
    }

    /// Map the unwrapped computation using the given function.
    /// ```text
    /// m.map(f).run() == f(m.run())
    /// ```
    pub fn map<B>(self, f: impl FunctionT<A, B>) -> Except<E, B>
    where
        E: Term,
        B: Term,
    {
        let f = f.to_function();
        self.map_t(|t| {
            t.fmap(|t| match t {
                Left(e) => Left(e),
                Right(x) => Right(f(x)),
            })
        })
    }

    /// Transform any exceptions thrown by the computation using the given function
    /// (a specialization of [`ExceptT::with_t`]).
    pub fn with<E_>(self, f: impl FunctionT<E, E_>) -> Except<E_, A>
    where
        E_: Term,
        E: Term,
        A: Term,
    {
        self.with_t(f.to_function())
    }
}

/// The parameterizable exception monad.
///
/// Computations are either exceptions or normal values.
///
/// The return function returns a normal value, while >>= exits on the first exception. For a variant that continues after an error and collects all the errors, see Errors.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExceptT<MA>(MA);

/// Utility alias for automatically hoisting `T` into the [`ExceptT`] transformer.
pub type HoistExceptT<E, T> = ExceptT<HoistEitherT<T, E>>;

impl<MA> ExceptT<MA>
where
    MA: Term,
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
    pub fn map_t<MB>(self, f: impl FunctionT<MA, MB>) -> ExceptT<MB>
    where
        MB: Term,
    {
        ExceptT(f(self.run_t()))
    }

    /// Transform any exceptions thrown by the computation using the given function.
    pub fn with_t<MB, E, E_, A>(self, f: impl FunctionT<E, E_>) -> ExceptT<MB>
    where
        MA: Functor<Either<E_, A>, Pointed = Either<E, A>, WithPointed = MB>,
        MB: Pointed<Pointed = Either<E_, A>>,
        E_: Term,
        A: Term,
        E: Term,
    {
        let f = f.to_function();
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
    pub fn catch<MB, E, E_, A>(self, h: impl FunctionT<E, ExceptT<MB>>) -> ExceptT<MB>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: ReturnM<Pointed = Either<E_, A>>,
        E: Term,
        A: Term,
    {
        let m = self;
        let h = h.to_function();
        ExceptT::new_t(m.run_t().chain_m(|a| match a {
            Left(l) => h(l).run_t(),
            Right(r) => ReturnM::return_m(Right(r)),
        }))
    }

    /// The same as `ExceptT::catch.flip()`,
    /// which is useful in situations where the code for the handler is shorter.
    pub fn handle<MB, E, E_, A>(h: impl FunctionT<E, ExceptT<MB>>, this: Self) -> ExceptT<MB>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: ReturnM<Pointed = Either<E_, A>>,
        E: Term,
        A: Term,
    {
        this.catch(h)
    }

    /// Similar to [`ExceptT::catch`], but returns an [`Either`] result
    /// which is `Right(a)` if no exception was thown,
    /// or `Left(ex)` if an exception `ex` was thrown.
    pub fn r#try<MB, MC, E, A>(self) -> ExceptT<MC>
    where
        MA: ChainM<MB, Pointed = Either<E, A>>,
        MB: ReturnM<Pointed = Either<E, Either<E, A>>> + ChainM<MC>,
        MC: ReturnM<Pointed = Either<E, Either<E, A>>>,
        E: Term,
        A: Term,
    {
        Right.lift_m()(self).catch(|t| ReturnM::return_m(Left(t)))
    }

    /// `a.finally(b)` executes computation `a` followed by computation `b`,
    /// even if `a` exits early by throwing an exception.
    ///
    /// In the latter case, the exception is re-thrown after `b` has been executed.
    pub fn finally<MB, MC, E, A>(self, closer: ExceptT<MC>) -> Self
    where
        MA: ChainM<MB, Pointed = Either<E, A>> + ReturnM,
        MB: ReturnM<Pointed = Either<E, Either<E, A>>> + ChainM<MB> + ChainM<MA>,
        MC: ChainM<MA, Pointed = Either<E, ()>>,
        E: Term,
        A: Term,
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
    A: Term,
{
    type Pointed = A;
}

impl<MA, E, A, B> WithPointed<B> for ExceptT<MA>
where
    MA: WithPointed<Either<E, B>, Pointed = Either<E, A>>,
    MA::WithPointed: Pointed<Pointed = Either<E, B>>,
    A: Term,
    B: Term,
{
    type WithPointed = ExceptT<MA::WithPointed>;
}

impl<MA, E, A, B> Functor<B> for ExceptT<MA>
where
    MA: Functor<Either<E, B>, Pointed = Either<E, A>>,
    E: Term,
    A: Term,
    B: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<A, B>) -> Self::WithPointed {
        let f = f.to_function();
        ExceptT(self.run_t().fmap(|t| t.fmap(f)))
    }
}

impl<MA, E, A> PureA for ExceptT<MA>
where
    MA: PureA<Pointed = Either<E, A>>,
    E: Term,
    A: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        ExceptT(PureA::pure_a(Right(t)))
    }
}

impl<MF, MA, MB, E, F, A, B> AppA<ExceptT<MA>, ExceptT<MB>> for ExceptT<MF>
where
    MF: ChainM<MB, Pointed = Either<E, F>>,
    MA: ChainM<MB, Pointed = Either<E, A>>,
    MB: ReturnM<Pointed = Either<E, B>>,
    F: Term + FunctionT<A, B>,
    E: Term,
    A: Term,
    B: Term,
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
    E: Term,
    A: Term,
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
    MA: ChainM<MB, Pointed = Either<E, A>>,
    MB: ReturnM<Pointed = Either<E, B>>,
    E: Term,
    A: Term,
{
    fn chain_m(self, k: impl FunctionT<A, ExceptT<MB>>) -> ExceptT<MB> {
        let m = self;
        let k = k.to_function();
        ExceptT(m.run_t().chain_m(|a| match a {
            Left(e) => ReturnM::return_m(Left(e)),
            Right(x) => k(x).run_t(),
        }))
    }
}

impl<MA, E, A, B> FoldMap<B> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
    A: Term,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B>) -> B {
        todo!()
    }
}

impl<MA, E, A, B> Foldable<B> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
    A: Term,
    B: Term,
{
    fn foldr(self, f: impl BifunT<A, B, B>, z: B) -> B {
        todo!()
    }

    fn foldl(self, f: impl BifunT<B, A, B>, z: B) -> B {
        todo!()
    }
}

impl<MA, E, A> Foldable1<A> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
    A: Term,
{
    fn foldr1(self, f: impl BifunT<A, A, A>) -> A {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<A, A, A>) -> A {
        foldl1_default(self, f)
    }
}

impl<MA, A1, T, A2, E, A> TraverseT<A1, T, A2> for ExceptT<MA>
where
    MA: Pointed<Pointed = Either<E, A>>,
    A: Term,
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        todo!()
    }
}

impl<A1, T, A2, E, A> SequenceA<T, A2> for ExceptT<A1>
where
    A1: Pointed<Pointed = Either<E, A>>,
    E: Term,
    A: Term,
{
    fn sequence_a(self) -> A2 {
        todo!()
    }
}

impl<MA, MB, E, A> MonadTrans<MB> for ExceptT<MA>
where
    MA: ReturnM<Pointed = Either<E, A>>,
    MB: Pointed<Pointed = A> + ChainM<MA>,
    A: Term,
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
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(<MA as LowerEither<E, A>>::Lowered::lift_io(m))
    }
}
