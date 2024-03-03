/// The WriterT monad transformer, which adds collection of outputs
/// (such as a count or string output) to a given monad.
///
/// This monad transformer provides only limited access to the output
/// during the computation.
/// For more general access, use Control.Monad.Trans.State instead.
use std::marker::PhantomData;

use crate::{
    base::data::{functor::identity::Identity, pointed::Lower, tuple::pair::Pair},
    prelude::*,
};

use super::class::MonadTrans;

/// A writer monad parameterized by the type w of output to accumulate.
///
/// The return function produces the output mempty, while >>= combines the outputs of the subcomputations using mappend.
pub type Writer<W, A> = WriterT<W, Identity<(A, W)>>;

impl<W, A> Writer<W, A> {
    /// Unwrap a writer computation as a (result, output) pair. (The inverse of writer.)
    pub fn run(self) -> (A, W) {
        self.run_t().run()
    }

    /// Extract the output from a writer computation.
    ///
    /// execWriter m = snd (runWriter m)
    pub fn exec(self) -> W {
        self.run().snd()
    }

    /// Map both the return value and output of a computation using the given function.
    ///
    /// runWriter (mapWriter f m) = f (runWriter m)
    pub fn map<B, W_>(self, f: impl FunctionT<(A, W), (B, W_)> + Clone) -> Writer<W_, B> {
        self.map_t(|t| Identity(f(t.run())))
    }
}

/// A writer monad parameterized by:
///
/// w - the output to accumulate.
/// m - The inner monad.
///
/// The return function produces the output mempty, while >>= combines the outputs of the subcomputations using mappend.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriterT<W, MA>(MA, PhantomData<W>);

impl<W, M> WriterT<W, M> {
    pub fn new_t(m: M) -> Self {
        WriterT(m, PhantomData)
    }

    /// Construct a writer computation from a (result, output) pair. (The inverse of run.)
    pub fn new<A>((a, w): (A, W)) -> Self
    where
        M: ReturnM<Pointed = (A, W)>,
    {
        WriterT::new_t(ReturnM::return_m((a, w)))
    }

    pub fn run_t(self) -> M {
        self.0
    }

    /// Extract the output from a writer computation.
    ///
    /// execWriterT m = liftM snd (runWriterT m)
    pub fn exec_t<A, N>(self) -> N
    where
        M: Clone + ChainM<N, Pointed = (A, W)>,
        N: Clone + ReturnM<Pointed = W>,
    {
        self.run_t().chain_m(|(_, w)| ReturnM::return_m(w))
    }

    /// Map both the return value and output of a computation using the given function.
    ///
    /// runWriterT (mapWriterT f m) = f (runWriterT m)
    pub fn map_t<W_, N>(self, f: impl FunctionT<M, N>) -> WriterT<W_, N> {
        WriterT::new_t(f(self.run_t()))
    }

    /// tell w is an action that produces the output w.
    pub fn tell(w: W) -> Self
    where
        M: ReturnM<Pointed = ((), W)>,
    {
        WriterT::new(((), w))
    }

    /// listen m is an action that executes the action m and adds its output to the value of the computation.
    ///
    /// runWriterT (listen m) = liftM (\ (a, w) -> ((a, w), w)) (runWriterT m)
    pub fn listen<N, A>(self) -> WriterT<W, N>
    where
        M: ChainM<N, Pointed = (A, W)>,
        N: Clone + ReturnM<Pointed = ((A, W), W)>,
        W: Clone,
    {
        WriterT::new_t(
            self.run_t()
                .chain_m(|(a, w)| ReturnM::return_m(((a, w.clone()), w))),
        )
    }

    /// listens f m is an action that executes the action m and adds the result of applying f to the output to the value of the computation.
    ///
    /// listens f m = liftM (id *** f) (listen m)
    /// runWriterT (listens f m) = liftM (\ (a, w) -> ((a, f w), w)) (runWriterT m)
    pub fn listens<N, A, B>(self, f: impl FunctionT<W, B> + Clone) -> WriterT<W, N>
    where
        M: ChainM<N, Pointed = (A, W)>,
        N: Clone + ReturnM<Pointed = ((A, B), W)>,
        W: Clone,
    {
        WriterT::new_t(
            self.run_t()
                .chain_m(|(a, w)| ReturnM::return_m(((a, f(w.clone())), w))),
        )
    }

    /// pass m is an action that executes the action m, which returns a value and a function, and returns the value, applying the function to the output.
    ///
    /// runWriterT (pass m) = liftM (\ ((a, f), w) -> (a, f w)) (runWriterT m)
    pub fn pass<F, A, B, N>(self) -> WriterT<W, N>
    where
        M: ChainM<N, Pointed = ((A, F), W)>,
        N: Clone + ReturnM<Pointed = (A, B)>,
        F: FunctionT<W, B>,
    {
        WriterT::new_t(
            self.run_t()
                .chain_m(|((a, f), w)| ReturnM::return_m((a, f(w)))),
        )
    }

    /// censor f m is an action that executes the action m and applies the function f to its output, leaving the return value unchanged.
    ///
    /// censor f m = pass (liftM (\ x -> (x,f)) m)
    /// runWriterT (censor f m) = liftM (\ (a, w) -> (a, f w)) (runWriterT m)
    pub fn censor<A>(self, f: impl FunctionT<W, W> + Clone) -> Self
    where
        M: Clone + ChainM<M> + ReturnM<Pointed = (A, W)>,
    {
        WriterT::new_t(self.run_t().chain_m(|(a, w)| ReturnM::return_m((a, f(w)))))
    }
}

impl<W, MA, A> Pointed for WriterT<W, MA>
where
    MA: Pointed<Pointed = (A, W)>,
{
    type Pointed = A;
}

impl<W, M, A, B> WithPointed<B> for WriterT<W, M>
where
    M: WithPointed<(B, W), Pointed = (A, W)>,
    M::WithPointed: Pointed<Pointed = (B, W)>,
{
    type WithPointed = WriterT<W, M::WithPointed>;
}

impl<W, M, A, B> Functor<B> for WriterT<W, M>
where
    B: Clone,
    M: Functor<(B, W), Pointed = (A, W)>,
    M::WithPointed: Pointed<Pointed = (B, W)>,
    W: Clone,
{
    fn fmap(self, f: impl FunctionT<Self::Pointed, B> + Clone) -> Self::WithPointed {
        self.map_t(|t| t.fmap(|(a, w)| (f(a), w)))
    }
}

impl<W, M, A> PureA for WriterT<W, M>
where
    M: ReturnM<Pointed = (A, W)>,
    W: Monoid,
{
    fn pure_a(t: Self::Pointed) -> Self {
        WriterT::new((t, Monoid::mempty()))
    }
}

impl<MF, MA, MB, W, F, A, B> AppA<WriterT<W, MA>, WriterT<W, MB>> for WriterT<W, MF>
where
    MF: Clone + Functor<Function<(A, W), (B, W)>, Pointed = (F, W)>,
    MF::WithPointed: AppA<MA, MB>,
    MA: Pointed<Pointed = (A, W)>,
    MB: ReturnM<Pointed = (B, W)>,
    W: Clone + Monoid,
    F: Clone + FunctionT<A, B>,
    A: 'static,
    B: 'static,
{
    fn app_a(self, v: WriterT<W, MA>) -> WriterT<W, MB> {
        let k = (|(a, w): (F, W), (b, w_): (A, W)| (a(b), w.assoc_s(w_))).lift_a2();
        let f: MF = self.run_t();
        let v: MA = v.run_t();
        let out: MB = k(f, v);
        WriterT::new_t(out)
    }
}

impl<W, M, A> ReturnM for WriterT<W, M>
where
    M: ReturnM<Pointed = (A, W)>,
    W: Monoid,
{
}

impl<W, M, N, A, B> ChainM<WriterT<W, N>> for WriterT<W, M>
where
    W: Clone + Monoid,
    M: ReturnM<Pointed = (A, W)> + ChainM<N>,
    N: Clone + ReturnM<Pointed = (B, W)> + ChainM<N>,
{
    fn chain_m(self, k: impl FunctionT<Self::Pointed, WriterT<W, N>> + Clone) -> WriterT<W, N>
    where
        WriterT<W, N>: Clone,
    {
        let m = self;
        WriterT::new_t(m.run_t().chain_m(|(a, w)| {
            k(a).run_t()
                .chain_m(|(b, w_)| ReturnM::return_m((b, w.assoc_s(w_))))
        }))
    }
}

impl<MO, W, A> MonadTrans<MO::Lowered> for WriterT<W, MO>
where
    MO: Lower<W, A> + ReturnM<Pointed = (A, W)>,
    MO::Lowered: ChainM<MO>,
    W: Monoid,
{
    fn lift(m: MO::Lowered) -> Self {
        WriterT::new_t(m.chain_m(|a| ReturnM::return_m((a, Monoid::mempty()))))
    }
}

#[cfg(test)]
mod test {
    use crate::{base::data::monoid::Sum, prelude::*};

    use super::Writer;

    fn acc1_w(input: String) -> (String, Sum<usize>) {
        if input.len() % 2 == 0 {
            acc2_w(input).run()
        } else {
            acc3_w(input[1..].to_string())
                .chain_m(move |str1| {
                    acc4_w(input[..1].to_string()).chain_m(|str2| ReturnM::return_m(str1 + &str2))
                })
                .run()
        }
    }

    fn acc2_w(input: String) -> Writer<Sum<usize>, String> {
        if input.len() > 10 {
            Writer::<Sum<usize>, _>::tell(Sum(1)).then_m(acc4_w(input[..9].to_string()))
        } else {
            Writer::<Sum<usize>, _>::tell(Sum(10)).then_m(ReturnM::return_m(input))
        }
    }

    fn acc3_w(input: String) -> Writer<Sum<usize>, String> {
        if input.len() % 3 == 0 {
            Writer::<Sum<usize>, _>::tell(Sum(3)).then_m(acc2_w(input + "ab"))
        } else {
            Writer::<Sum<usize>, _>::tell(Sum(1)).then_m(ReturnM::return_m(input[1..].to_string()))
        }
    }

    fn acc4_w(input: String) -> Writer<Sum<usize>, String> {
        if input.len() < 10 {
            Writer::<Sum<usize>, _>::tell(Sum(input.len()))
                .then_m(ReturnM::return_m(format!("{input}{input}")))
        } else {
            Writer::<Sum<usize>, _>::tell(Sum(5)).then_m(ReturnM::return_m(input[..5].to_string()))
        }
    }

    #[test]
    fn test_acc_writer() {
        let out = acc1_w("Hello one two three".to_string());
        println!("{out:#?}");
    }
}
