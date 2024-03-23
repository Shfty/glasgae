/// The WriterT monad transformer, which adds collection of outputs
/// (such as a count or string output) to a given monad.
///
/// This monad transformer provides only limited access to the output
/// during the computation.
/// For more general access, use Control.Monad.Trans.State instead.
use std::marker::PhantomData;

use crate::{
    base::{
        control::monad::{
            io::MonadIO,
            morph::{HoistTupleT, MonadLower, MonadLoweredT},
        },
        data::{functor::identity::Identity, tuple::pair::Pair},
    },
    prelude::*,
};

use super::class::MonadTrans;

/// A writer monad parameterized by the type w of output to accumulate.
///
/// The return function produces the output mempty, while >>= combines the outputs of the subcomputations using mappend.
pub type Writer<W, A> = WriterT<W, Identity<(A, W)>>;

impl<W, A> Writer<W, A>
where
    W: Term,
    A: Term,
{
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
    pub fn map<B, W_>(self, f: impl FunctionT<(A, W), (B, W_)>) -> Writer<W_, B>
    where
        B: Term,
        W_: Term,
    {
        let f = f.to_function();
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

/// Utility alias for automatically hoisting `T` into the [`StateT`] transformer.
pub type HoistWriterT<W, T> = WriterT<W, HoistTupleT<T, W>>;

impl<W, M> WriterT<W, M>
where
    W: Term,
    M: Term,
{
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
        M: Monad<W, Pointed = (A, W), Chained = N>,
        N: ReturnM<Pointed = W>,
        A: Term,
    {
        self.run_t().chain_m(|(_, w)| ReturnM::return_m(w))
    }

    /// Map both the return value and output of a computation using the given function.
    ///
    /// runWriterT (mapWriterT f m) = f (runWriterT m)
    pub fn map_t<W_, N>(self, f: impl FunctionT<M, N>) -> WriterT<W_, N>
    where
        W_: Term,
        N: Term,
    {
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
        M: Monad<((A, W), W), Pointed = (A, W), Chained = N>,
        N: ReturnM<Pointed = ((A, W), W)>,
        A: Term,
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
    pub fn listens<N, A, B>(self, f: impl FunctionT<W, B>) -> WriterT<W, N>
    where
        M: Monad<((A, B), W), Pointed = (A, W), Chained = N>,
        N: ReturnM<Pointed = ((A, B), W)>,
        W: Term,
        A: Term,
        B: Term,
    {
        let f = f.to_function();
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
        M: Monad<(A, B), Pointed = ((A, F), W), Chained = N>,
        N: ReturnM<Pointed = (A, B)>,
        F: Term + FunctionT<W, B>,
        A: Term,
        B: Term,
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
    pub fn censor<A>(self, f: impl FunctionT<W, W>) -> Self
    where
        M: Clone + Monad<(A, W), Pointed = (A, W), Chained = M>,
        A: Term,
    {
        let f = f.to_function();
        WriterT::new_t(self.run_t().chain_m(|(a, w)| ReturnM::return_m((a, f(w)))))
    }
}

impl<W, MA, A> Pointed for WriterT<W, MA>
where
    W: Term,
    MA: Pointed<Pointed = (A, W)>,
    A: Term,
{
    type Pointed = A;
}

impl<W, M, A, B> WithPointed<B> for WriterT<W, M>
where
    W: Term,
    M: WithPointed<(B, W), Pointed = (A, W)>,
    A: Term,
    B: Term,
{
    type WithPointed = WriterT<W, M::WithPointed>;
}

impl<W, MA, A, MB, B> Functor<B> for WriterT<W, MA>
where
    MA: Functor<(B, W), Pointed = (A, W), Mapped = MB>,
    MB: Functor<(A, W), Pointed = (B, W), Mapped = MA>,
    W: Term,
    A: Term,
    B: Term,
{
    type Mapped = WriterT<W, MB>;

    fn fmap(self, f: impl FunctionT<Self::Pointed, B>) -> Self::Mapped {
        let f = f.to_function();
        self.map_t(|t| t.fmap(|(a, w)| (f(a), w)))
    }
}

impl<W, M, A> PureA for WriterT<W, M>
where
    W: Monoid,
    M: ReturnM<Pointed = (A, W)>,
    A: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        WriterT::new((t, Monoid::mempty()))
    }
}

impl<W, MF, F, MA, A, MB, B> AppA<A, B> for WriterT<W, MF>
where
    MF: Functor<Function<(A, W), (B, W)>, Pointed = (F, W)>
        + WithPointed<(A, W), WithPointed = MA>
        + WithPointed<(B, W), WithPointed = MB>,
    MF::Mapped: Applicative<(A, W), (B, W), WithA = MA, WithB = MB>,
    W: Semigroup,
    MA: Pointed<Pointed = (A, W)> + WithPointed<(F, W), WithPointed = MF>,
    MB: Pointed<Pointed = (B, W)> + WithPointed<(F, W), WithPointed = MF>,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = WriterT<W, MA>;
    type WithB = WriterT<W, MB>;

    fn app_a(self, v: WriterT<W, MA>) -> WriterT<W, MB> {
        let f = self;
        let k = (|(f, w): (F, W), (a, w_): (A, W)| (f(a), w.assoc_s(w_))).lift_a2();
        let f: MF = f.run_t();
        let v: WithPointedT<MF, (A, W)> = v.run_t();
        let out: WithPointedT<MF, (B, W)> = k(f, v);
        WriterT::new_t(out)
    }
}

impl<W, M, A> ReturnM for WriterT<W, M>
where
    W: Monoid,
    M: ReturnM<Pointed = (A, W)>,
    A: Term,
{
}

impl<W, MA, A, MB, B> ChainM<B> for WriterT<W, MA>
where
    W: Monoid,
    MA: Monad<(B, W), Pointed = (A, W), Chained = MB> + Monad<(A, W), Chained = MA>,
    MB: Monad<(B, W), Pointed = (B, W), Chained = MB> + Monad<(A, W), Chained = MA>,
    A: Term,
    B: Term,
{
    type Chained = WriterT<W, MB>;

    fn chain_m(self, k: impl FunctionT<Self::Pointed, WriterT<W, MB>>) -> WriterT<W, MB>
    where
        WriterT<W, MB>: Clone,
    {
        let m = self;
        let k = k.to_function();
        WriterT::new_t(ChainM::<(B, W)>::chain_m(m.run_t(), |(a, w)| {
            ChainM::<(B, W)>::chain_m(k(a).run_t(), |(b, w_)| {
                ReturnM::return_m((b, w.assoc_s(w_)))
            })
        }))
    }
}

impl<MA, W, A> MonadTrans<MA::Lowered> for WriterT<W, MA>
where
    MA: MonadLower<A, W> + ReturnM<Pointed = (A, W)>,
    MA::Lowered: Monad<(A, W), Pointed = A, Chained = MA>,
    W: Monoid,
    A: Term,
{
    fn lift(m: MA::Lowered) -> Self {
        WriterT::new_t(m.chain_m(|a| ReturnM::return_m((a, Monoid::mempty()))))
    }
}

impl<MA, W, A> MonadIO<A> for WriterT<W, MA>
where
    MA: MonadLower<A, W> + ReturnM<Pointed = (A, W)>,
    MA::Lowered: Monad<(A, W), Chained = MA> + MonadIO<A>,
    W: Monoid,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadLoweredT::<MA, A, W>::lift_io(m))
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
