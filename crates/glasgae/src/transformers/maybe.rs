//! The MaybeT monad transformer extends a monad with the ability to exit the computation without returning a value.
//!
//! A sequence of actions produces a value only if all the actions in the sequence do. If one exits, the rest of the sequence is skipped and the composite action exits.
//!
//! For a variant allowing a range of exception values, see Control.Monad.Trans.Except.

use crate::prelude::*;

use super::class::MonadTrans;

/// The parameterizable maybe monad, obtained by composing an arbitrary monad with the Maybe monad.
///
/// Computations are actions that may produce a value or exit.
///
/// The return function yields a computation that produces that value, while >>= sequences two subcomputations, exiting if either computation does.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeT<MA>(MA);

impl<MA> MaybeT<MA>
where
    MA: Term,
{
    /// Convert a Maybe computation to MaybeT.
    pub fn new(t: MA::Pointed) -> Self
    where
        MA: PureA,
    {
        MaybeT(PureA::pure_a(t))
    }

    /// Extract the inner monad.
    pub fn run(self) -> MA {
        self.0
    }

    /// Transform the computation inside a MaybeT.
    /// ```text
    /// m.map(f).run() = f(m.run())
    /// ```
    pub fn map<MB>(self, f: impl FunctionT<MA, MB>) -> MaybeT<MB>
    where
        MB: Term,
    {
        MaybeT(f(self.run()))
    }
}

impl<MA, T> Pointed for MaybeT<MA>
where
    MA: Pointed<Pointed = Maybe<T>>,
    T: Term,
{
    type Pointed = T;
}

impl<MA, T, U> WithPointed<U> for MaybeT<MA>
where
    MA: WithPointed<Maybe<U>, Pointed = Maybe<T>>,
    T: Term,
    U: Term,
{
    type WithPointed = MaybeT<MA::WithPointed>;
}

impl<MA, A, MB, B> Functor<B> for MaybeT<MA>
where
    MA: Functor<Maybe<B>, Pointed = Maybe<A>, Mapped = MB>,
    MB: Functor<Maybe<A>, Pointed = Maybe<B>, Mapped = MA>,
    A: Term,
    B: Term,
{
    type Mapped = MaybeT<MA::Mapped>;

    fn fmap(self, f: impl crate::prelude::FunctionT<A, B>) -> Self::Mapped {
        let f = f.to_function();
        self.map(|t| t.fmap(|t| t.fmap(f)))
    }
}

impl<MA, A> PureA for MaybeT<MA>
where
    MA: ReturnM<Pointed = Maybe<A>>,
    A: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        MaybeT(ReturnM::return_m(Just(t)))
    }
}

impl<MF, F, MA, A, MB, B> AppA<A, B> for MaybeT<MF>
where
    MF: ReturnM<Pointed = Maybe<F>> + Monad<Maybe<A>, Chained = MA> + Monad<Maybe<B>, Chained = MB>,
    MA: Monad<Maybe<B>, Pointed = Maybe<A>, Chained = MB> + Monad<Maybe<F>, Chained = MF>,
    MB: Monad<Maybe<A>, Pointed = Maybe<B>, Chained = MA> + Monad<Maybe<F>, Chained = MF>,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = MaybeT<MA>;
    type WithB = MaybeT<MB>;

    fn app_a(self, mx: MaybeT<MA>) -> MaybeT<MB> {
        let mf = self;
        MaybeT({
            ChainM::<Maybe<B>>::chain_m(mf.run(), |mb_f| match mb_f {
                Nothing => ReturnM::return_m(Nothing),
                Just(f) => ChainM::<Maybe<B>>::chain_m(mx.run(), |mb_x| match mb_x {
                    Nothing => ReturnM::return_m(Nothing),
                    Just(x) => ReturnM::return_m(Just(f(x))),
                }),
            })
        })
    }
}

impl<MA, A> ReturnM for MaybeT<MA>
where
    MA: ReturnM<Pointed = Maybe<A>>,
    A: Term,
{
    fn return_m(t: Self::Pointed) -> Self {
        MaybeT(ReturnM::return_m(Just(t)))
    }
}

impl<MA, MB, A, B> ChainM<B> for MaybeT<MA>
where
    MA: Monad<Maybe<B>, Pointed = Maybe<A>, Chained = MB>,
    MB: Monad<Maybe<A>, Pointed = Maybe<B>, Chained = MA>,
    A: Term,
    B: Term,
{
    type Chained = MaybeT<MB>;

    fn chain_m(self, f: impl FunctionT<Self::Pointed, MaybeT<MB>>) -> MaybeT<MB> {
        let x = self;
        let f = f.to_function();
        MaybeT({
            x.run().chain_m(|v| match v {
                Nothing => ReturnM::return_m(Nothing),
                Just(y) => f(y).run(),
            })
        })
    }
}

impl<MA, A, B> Foldable<B> for MaybeT<MA>
where
    MA: Pointed<Pointed = Maybe<A>>,
    A: Term,
{
    fn foldr(self, f: impl crate::base::data::function::bifunction::BifunT<A, B, B>, z: B) -> B {
        todo!()
    }

    fn foldl(self, f: impl crate::base::data::function::bifunction::BifunT<B, A, B>, z: B) -> B {
        todo!()
    }
}

impl<MA, A, B> FoldMap<B> for MaybeT<MA>
where
    MA: FoldMap<B, Pointed = Maybe<A>>,
    Maybe<A>: FoldMap<B, Pointed = A>,
    A: Term,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B>) -> B {
        let f = f.to_function();
        self.run().fold_map(|t| t.fold_map(f))
    }
}

impl<A, MA, A1, A2> TraverseT<A1, (), A2> for MaybeT<MA>
where
    Self: Functor<A1>,
    MappedT<Self, A1>: SequenceA<(), A2>,
    MA: Pointed<Pointed = Maybe<A>>,
    A: Term,
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<A1, A2> SequenceA<(), A2> for MaybeT<A1>
where
    Self: TraverseT<A1, (), A2>,
    A1: Term,
{
    fn sequence_a(self) -> A2 {
        todo!()
    }
}

impl<MI, MA> MonadTrans<MI> for MaybeT<MA> {
    fn lift(m: MI) -> Self {
        todo!()
    }
}

impl<MA, A> MonadIO<A> for MaybeT<MA>
where
    Self: MonadTrans<IO<A>>,
    MA: Pointed<Pointed = Maybe<A>>,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}
