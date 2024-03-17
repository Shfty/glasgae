//! The MaybeT monad transformer extends a monad with the ability to exit the computation without returning a value.
//!
//! A sequence of actions produces a value only if all the actions in the sequence do. If one exits, the rest of the sequence is skipped and the composite action exits.
//!
//! For a variant allowing a range of exception values, see Control.Monad.Trans.Except.

use crate::{
    base::{
        control::monad::io::MonadIO,
        data::{traversable::traverse_t_default, FoldMap},
    },
    prelude::*,
};

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

impl<MA, A, B> Fmap<B> for MaybeT<MA>
where
    MA: Fmap<Maybe<B>, Pointed = Maybe<A>>,
    A: Term,
    B: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<A, B>) -> Self::WithPointed {
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

impl<MF, MA, MB, F, A, B> AppA<MaybeT<MA>, MaybeT<MB>> for MaybeT<MF>
where
    MF: ChainM<MB, Pointed = Maybe<F>>,
    MA: ChainM<MB, Pointed = Maybe<A>>,
    MB: ReturnM<Pointed = Maybe<B>>,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
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
    A: Term,
{
    fn return_m(t: Self::Pointed) -> Self {
        MaybeT(ReturnM::return_m(Just(t)))
    }
}

impl<MA, MB, A, B> ChainM<MaybeT<MB>> for MaybeT<MA>
where
    MA: ChainM<MB, Pointed = Maybe<A>>,
    MB: ReturnM<Pointed = Maybe<B>>,
    A: Term,
    B: Term,
{
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

impl<A, MA, A1, T, A2> TraverseT<A1, T, A2> for MaybeT<MA>
where
    Self: Fmap<A1>,
    WithPointedT<Self, A1>: SequenceA<T, A2>,
    MA: Pointed<Pointed = Maybe<A>>,
    A: Term,
    A1: Term,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1>) -> A2 {
        traverse_t_default(self, f)
    }
}

impl<A1, A_, A2> SequenceA<A_, A2> for MaybeT<A1>
where
    Self: TraverseT<A1, A_, A2>,
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
