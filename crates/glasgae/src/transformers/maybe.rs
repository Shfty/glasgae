//! The MaybeT monad transformer extends a monad with the ability to exit the computation without returning a value.
//!
//! A sequence of actions produces a value only if all the actions in the sequence do. If one exits, the rest of the sequence is skipped and the composite action exits.
//!
//! For a variant allowing a range of exception values, see Control.Monad.Trans.Except.

use crate::{
    base::{control::monad::io::MonadIO, data::FoldMap},
    prelude::{
        AppA, ChainM, FunctionT, Functor, Maybe, Maybe::*, Monoid, Pointed, PureA, ReturnM,
        SequenceA, TraverseT, WithPointed, IO,
    },
};

use super::class::MonadTrans;

/// The parameterizable maybe monad, obtained by composing an arbitrary monad with the Maybe monad.
///
/// Computations are actions that may produce a value or exit.
///
/// The return function yields a computation that produces that value, while >>= sequences two subcomputations, exiting if either computation does.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeT<MA>(MA);

impl<MA> MaybeT<MA> {
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
    pub fn map<MB>(self, f: impl FunctionT<MA, MB>) -> MaybeT<MB> {
        MaybeT(f(self.run()))
    }
}

impl<MA, T> Pointed for MaybeT<MA>
where
    MA: Pointed<Pointed = Maybe<T>>,
{
    type Pointed = T;
}

impl<MA, T, U> WithPointed<U> for MaybeT<MA>
where
    MA: WithPointed<Maybe<U>, Pointed = Maybe<T>>,
{
    type WithPointed = MaybeT<MA::WithPointed>;
}

impl<MA, A, B> Functor<B> for MaybeT<MA>
where
    MA: Functor<Maybe<B>, Pointed = Maybe<A>>,
    A: Clone,
    B: Clone,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<A, B> + Clone) -> Self::WithPointed {
        self.map(|t| t.fmap(|t| t.fmap(f)))
    }
}

impl<MA, A> PureA for MaybeT<MA>
where
    MA: ReturnM<Pointed = Maybe<A>>,
{
    fn pure_a(t: Self::Pointed) -> Self {
        MaybeT(ReturnM::return_m(Just(t)))
    }
}

impl<MF, MA, MB, F, A, B> AppA<MaybeT<MA>, MaybeT<MB>> for MaybeT<MF>
where
    MF: ChainM<MB, Pointed = Maybe<F>>,
    MA: 'static + Clone + ChainM<MB, Pointed = Maybe<A>>,
    MB: ReturnM<Pointed = Maybe<B>>,
    F: Clone + FunctionT<A, B>,
{
    fn app_a(self, mx: MaybeT<MA>) -> MaybeT<MB> {
        let mf = self;
        MaybeT({
            mf.run().chain_m(|mb_f| match mb_f {
                Nothing => ReturnM::return_m(Nothing),
                Just(f) => mx.run().chain_m(|mb_x| match mb_x {
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
{
    fn return_m(t: Self::Pointed) -> Self
    where
        Self: Sized,
    {
        MaybeT(ReturnM::return_m(Just(t)))
    }
}

impl<MA, MB, A, B> ChainM<MaybeT<MB>> for MaybeT<MA>
where
    MA: ChainM<MB, Pointed = Maybe<A>>,
    MB: ReturnM<Pointed = Maybe<B>>,
{
    fn chain_m(self, f: impl FunctionT<Self::Pointed, MaybeT<MB>> + Clone) -> MaybeT<MB> {
        let x = self;
        MaybeT({
            x.run().chain_m(|v| match v {
                Nothing => ReturnM::return_m(Nothing),
                Just(y) => f(y).run(),
            })
        })
    }
}

impl<MA, A, B> FoldMap<A, B> for MaybeT<MA>
where
    MA: FoldMap<A, B>,
    A: FoldMap<A, B>,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B> + Clone) -> B {
        self.run().fold_map(|t| t.fold_map(f))
    }
}

impl<A, MA, A1, T, A2> TraverseT<A1, T, A2> for MaybeT<MA>
where
    MA: Pointed<Pointed = Maybe<A>>,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        todo!()
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for MaybeT<A1>
where
    Self: TraverseT<A1, A_, A2>,
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
    A: 'static,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}
