//! [`MonadWriter`] trait generalizing [`WriterT`] functionality.
//!
//! Inspired by the paper
//! Functional Programming with Overloading and Higher-Order Polymorphism,
//! Mark P Jones (<http://web.cecs.pdx.edu/~mpj/pubs/springschool.html>)
//! Advanced School of Functional Programming, 1995.

use crate::prelude::*;

use crate::transformers::{class::MonadTrans, reader::ReaderT, state::StateT, writer::WriterT};

pub trait MonadWriter<W, A> {
    /// writer (a,w) embeds a simple writer action.
    fn writer(a: A, w: W) -> Self;
}

pub trait MonadTell<W, A> {
    /// tell w is an action that produces the output w.
    fn tell(w: W) -> Self;
}

pub trait MonadListen<MO> {
    /// listen m is an action that executes the action m and adds its output to the value of the computation.
    fn listen(self) -> MO;
}

pub trait MonadPass<MF> {
    /// pass m is an action that executes the action m, which returns a value and a function, and returns the value, applying the function to the output.
    fn pass(self) -> MF;
}

// WriterT impl
impl<MA, W, A> MonadWriter<W, A> for WriterT<W, MA>
where
    MA: ReturnM<Pointed = (A, W)>,
{
    fn writer(a: A, w: W) -> Self {
        WriterT::new((a, w))
    }
}

impl<MA, W, A> MonadTell<W, A> for WriterT<W, MA>
where
    MA: ReturnM<Pointed = ((), W)>,
{
    fn tell(w: W) -> Self {
        WriterT::tell(w)
    }
}

impl<MA, MO, W, A> MonadListen<WriterT<W, MO>> for WriterT<W, MA>
where
    W: Clone,
    MA: ChainM<MO, Pointed = (A, W)>,
    MO: Clone + ReturnM<Pointed = ((A, W), W)>,
{
    fn listen(self) -> WriterT<W, MO> {
        self.listen()
    }
}

impl<MA, MB, W, A, F, B> MonadPass<WriterT<W, MB>> for WriterT<W, MA>
where
    MA: ChainM<MB, Pointed = ((A, F), W)>,
    MB: Clone + ReturnM<Pointed = (A, B)>,
    F: Clone + FunctionT<W, B>,
{
    fn pass(self) -> WriterT<W, MB> {
        self.pass()
    }
}

// ReaderT impl
impl<MA, R, W, A> MonadWriter<W, A> for ReaderT<R, MA>
where
    MA: Clone + MonadWriter<W, A>,
{
    fn writer(a: A, w: W) -> Self {
        Self::lift(MA::writer(a, w))
    }
}

impl<MA, R, W, A> MonadTell<W, A> for ReaderT<R, MA>
where
    MA: Clone + MonadTell<W, A>,
{
    fn tell(w: W) -> Self {
        Self::lift(MA::tell(w))
    }
}

impl<MA, MB, R> MonadListen<ReaderT<R, MB>> for ReaderT<R, MA>
where
    MA: Clone + MonadListen<MB>,
    R: Clone,
{
    fn listen(self) -> ReaderT<R, MB> {
        self.map_t(MA::listen)
    }
}

impl<MA, MB, R> MonadPass<ReaderT<R, MB>> for ReaderT<R, MA>
where
    MA: Clone + MonadPass<MB>,
    R: Clone,
{
    fn pass(self) -> ReaderT<R, MB> {
        self.map_t(MA::pass)
    }
}

// StateT impl
impl<MA, S, W, A> MonadWriter<W, A> for StateT<S, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadWriter<W, A>,
{
    fn writer(a: A, w: W) -> Self {
        Self::lift(MA::writer(a, w))
    }
}

impl<MA, R, W, A> MonadTell<W, A> for StateT<R, MA>
where
    Self: MonadTrans<MA>,
    MA: MonadTell<W, A>,
{
    fn tell(w: W) -> Self {
        Self::lift(MA::tell(w))
    }
}

impl<MA, MB, R> MonadListen<StateT<R, MB>> for StateT<R, MA>
where
    MA: Clone + MonadListen<MB>,
    R: Clone,
{
    fn listen(self) -> StateT<R, MB> {
        self.map_t(MA::listen)
    }
}

impl<MA, MB, R> MonadPass<StateT<R, MB>> for StateT<R, MA>
where
    MA: Clone + MonadPass<MB>,
    R: Clone,
{
    fn pass(self) -> StateT<R, MB> {
        self.map_t(MA::pass)
    }
}

/// Support functions
pub trait Listens<W, MA, A, B, MAB>: MonadWriter<W, A> {
    /// listens f m is an action that executes the action m and adds the result of applying f to the output to the value of the computation.
    ///
    /// listens f m = liftM (id *** f) (listen m)
    fn listens(self, f: impl FunctionT<W, B> + Clone) -> MAB;
}

impl<W, MA, A, B, T, MAB> Listens<W, MA, A, B, MAB> for T
where
    T: MonadWriter<W, A> + MonadListen<MA>,
    MA: ChainM<MAB, Pointed = (A, W)>,
    MAB: Clone + ReturnM<Pointed = (A, B)>,
    W: Clone,
{
    fn listens(self, f: impl FunctionT<W, B> + Clone) -> MAB {
        self.listen().chain_m(|(a, w)| ReturnM::return_m((a, f(w))))
    }
}

pub trait Censor<W, B, MA, MF> {
    /// censor f m is an action that executes the action m and applies the function f to its output, leaving the return value unchanged.
    ///
    /// censor f m = pass (liftM (\x -> (x,f)) m)
    fn censor(self, f: impl FunctionT<W, W> + Clone) -> MF;
}

impl<T, W, B, MA, MF> Censor<W, B, MA, MF> for T
where
    T: MonadPass<MA> + ChainM<MA>,
    MA: MonadPass<MF> + ReturnM<Pointed = (T::Pointed, Function<W, W>)>,
{
    fn censor(self, f: impl FunctionT<W, W> + Clone) -> MF {
        MA::pass(self.chain_m(|a| ReturnM::return_m((a, (f.boxed() as Box<dyn FunctionT<_, _>>)))))
    }
}
