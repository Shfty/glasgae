//! State monads, passing an updatable state through a computation.
//!
//! See below for examples.
//!
//! Some computations may not require the full power of state transformers:
//!
//! For a read-only state, see Control.Monad.Trans.Reader.
//! To accumulate a value without using it on the way, see Control.Monad.Trans.Writer.

use crate::{
    base::{
        control::monad::{
            io::MonadIO,
            morph::{HoistTupleT, MonadLower},
        },
        data::{functor::identity::Identity, tuple::pair::Pair},
    },
    prelude::*,
};

use super::class::MonadTrans;

/// A state monad parameterized by the type s of the state to carry.
///
/// The return function leaves the state unchanged, while >>= uses the final state of the first computation as the initial state of the second.
pub type State<S, A> = StateT<S, Identity<(A, S)>>;

/// Utility alias for automatically hoisting `T` into the [`State`] monad.
pub type HoistState<S, T> = StateT<S, HoistTupleT<Identity<T>, S>>;

impl<S, A> State<S, A>
where
    S: Term,
    A: Term,
{
    /// Unwrap a state monad computation as a function. (The inverse of new.)
    ///
    /// Self: State-passing computation to execute
    /// s: Initial state
    /// Return: Return value and final state
    pub fn run(self, s: S) -> (A, S) {
        self.run_t(s).run()
    }

    /// Evaluate a state computation with the given initial state and return the final value, discarding the final state.
    ///
    /// evalState m s = fst (runState m s)
    ///
    /// Self: State-passing computation to execute
    /// s: Initial state
    /// Return: Return value and final state
    pub fn eval(self, s: S) -> A
    where
        A: Clone,
    {
        self.eval_t(s).run()
    }

    /// Evaluate a state computation with the given initial state and return the final state, discarding the final value.
    ///
    /// execState m s = snd (runState m s)
    ///
    /// Self: State-passing computation to execute
    /// s: Initial value
    /// Return: Final state
    pub fn exec(self, s: S) -> S
    where
        S: Clone,
    {
        self.exec_t(s).run()
    }

    /// Map both the return value and final state of a computation using the given function.
    ///
    /// runState (mapState f m) = f . runState m
    pub fn map<B>(self, f: impl FunctionT<(A, S), (B, S)>) -> State<S, B>
    where
        B: Term,
    {
        let f = f.to_function();
        self.map_t(|t| Identity(f(t.run())))
    }

    /// withState f m executes action m on a state modified by applying f.
    ///
    /// withState f m = modify f >> m
    pub fn with(self, f: impl FunctionT<S, S>) -> Self
    where
        S: Clone,
        A: Clone,
    {
        self.with_t(f.to_function())
    }
}

/// A state transformer monad parameterized by:
///
/// s - The state.
/// m - The inner monad.
///
/// The return function leaves the state unchanged, while >>= uses the final state of the first computation as the initial state of the second.
#[derive(Clone)]
pub struct StateT<S, M>(Function<S, M>)
where
    S: Term,
    M: Term;

/// Utility alias for automatically hoisting `T` into the [`StateT`] transformer.
pub type HoistStateT<S, T> = StateT<S, HoistTupleT<T, S>>;

impl<S, M> StateT<S, M>
where
    S: Term,
    M: Term,
{
    pub fn new_t(f: impl FunctionT<S, M>) -> Self {
        StateT(f.boxed())
    }

    /// Construct a state monad computation from a function. (The inverse of runState.)
    ///
    /// f: Pure state transformer
    /// return: Equivalent state-passing computation
    pub fn new<A>(f: impl FunctionT<S, (A, S)>) -> Self
    where
        M: ReturnM<Pointed = (A, S)>,
        A: Term,
    {
        StateT::new_t(f.to_function().compose_clone(ReturnM::return_m))
    }

    pub fn run_t(self, s: S) -> M {
        self.0(s)
    }

    /// Evaluate a state computation with the given initial state and return the final value, discarding the final state.
    ///
    /// evalStateT m s = liftM fst (runStateT m s)
    pub fn eval_t<A, N>(self, s: S) -> N
    where
        M: Monad<A, Pointed = (A, S), Chained = N>,
        N: ReturnM<Pointed = A>,
        A: Term,
    {
        let m = self;
        m.run_t(s)
            .chain_m(Pair::fst.compose_clone(ReturnM::return_m))
    }

    /// Evaluate a state computation with the given initial state and return the final state, discarding the final value.
    ///
    /// execStateT m s = liftM snd (runStateT m s)
    pub fn exec_t<A, N>(self, s: S) -> N
    where
        M: Monad<S, Pointed = (A, S), Chained = N>,
        N: ReturnM<Pointed = S>,
        A: Term,
    {
        let m = self;
        m.run_t(s)
            .chain_m(Pair::snd.compose_clone(ReturnM::return_m))
    }

    /// Map both the return value and final state of a computation using the given function.
    ///
    /// runStateT (mapStateT f m) = f . runStateT m
    pub fn map_t<N>(self, f: impl FunctionT<M, N>) -> StateT<S, N>
    where
        N: Term,
    {
        let f = f.to_function();
        StateT::new_t(|t| f(self.run_t(t)))
    }

    /// withStateT f m executes action m on a state modified by applying f.
    ///
    /// withStateT f m = modify f >> m
    pub fn with_t<A>(self, f: impl FunctionT<S, S>) -> Self
    where
        M: ReturnM<Pointed = (A, S)>,
        A: Term,
    {
        let f = f.to_function();
        StateT::new_t(|t| self.run_t(f(t)))
    }

    /// Fetch the current value of the state within the monad.
    pub fn get() -> Self
    where
        S: Clone,
        M: ReturnM<Pointed = (S, S)>,
    {
        StateT::new(|s: S| (s.clone(), s))
    }

    /// put s sets the state within the monad to s.
    pub fn put(s: S) -> Self
    where
        M: ReturnM<Pointed = ((), S)>,
        S: Clone,
    {
        StateT::new(|_| ((), s))
    }

    /// modify f is an action that updates the state to the result of applying f to the current state.
    ///
    /// modify f = get >>= (put . f)
    pub fn modify(f: impl FunctionT<S, S>) -> State<S, ()>
    where
        S: Clone,
    {
        let f = f.to_function();
        StateT::new(|s| ((), f(s)))
    }

    pub fn modify_m<N>(f: impl FunctionT<S, N>) -> StateT<S, M>
    where
        N: Monad<((), S), Pointed = S, Chained = M>,
        M: ReturnM<Pointed = ((), S)>,
    {
        let f = f.to_function();
        StateT::new_t(|s| f(s).chain_m(|s_| ReturnM::return_m(((), s_))))
    }

    /// Get a specific component of the state, using a projection function supplied.
    ///
    /// gets f = liftM f get
    pub fn gets<A>(f: impl FunctionT<S, A>) -> State<S, A>
    where
        A: Term,
    {
        let f = f.to_function();
        StateT::new(|s: S| (f(s.clone()), s))
    }
}

impl<S, M, A> Pointed for StateT<S, M>
where
    S: Term,
    M: Pointed<Pointed = (A, S)>,
    A: Term,
{
    type Pointed = A;
}

impl<S, M, A, T> WithPointed<T> for StateT<S, M>
where
    S: Term,
    M: WithPointed<(T, S), Pointed = (A, S)>,
    A: Term,
    T: Term,
{
    type WithPointed = StateT<S, M::WithPointed>;
}

impl<S, MA, A, MB, B> Functor<B> for StateT<S, MA>
where
    MA: Functor<(B, S), Pointed = (A, S), Mapped = MB>,
    MB: Functor<(A, S), Pointed = (B, S), Mapped = MA>,
    B: Term,
    S: Term,
    A: Term,
{
    type Mapped = StateT<S, MA::Mapped>;

    fn fmap(self, f: impl FunctionT<A, B>) -> Self::Mapped {
        let m = self;
        let f = f.to_function();
        StateT::new_t(|s: S| m.run_t(s).fmap(|(a, s_)| (f(a), s_)))
    }
}

impl<S, M, A> PureA for StateT<S, M>
where
    M: ReturnM<Pointed = (A, S)>,
    S: Term,
    A: Term,
{
    fn pure_a(a: Self::Pointed) -> Self {
        StateT::new_t(|s| ReturnM::return_m((a, s)))
    }
}

impl<S, MF, F, MA, A, MB, B> AppA<A, B> for StateT<S, MF>
where
    MF: ReturnM<Pointed = (F, S)> + Monad<(A, S), Chained = MA> + Monad<(B, S), Chained = MB>,
    MA: Monad<(B, S), Pointed = (A, S), Chained = MB> + Monad<(F, S), Chained = MF>,
    MB: Monad<(A, S), Pointed = (B, S), Chained = MA> + Monad<(F, S), Chained = MF>,
    S: Term,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = StateT<S, MA>;
    type WithB = StateT<S, MB>;

    fn app_a(self, mx: WithPointedT<Self, A>) -> WithPointedT<Self, B> {
        let StateT(mf) = self;
        let StateT(mx) = mx;
        StateT::new_t(|s| {
            ChainM::<(B, S)>::chain_m(mf(s), |(f, s_)| {
                ChainM::<(B, S)>::chain_m(mx(s_), |(x, s__)| ReturnM::return_m((f(x), s__)))
            })
        })
    }
}

impl<S, M, A> ReturnM for StateT<S, M>
where
    M: ReturnM<Pointed = (A, S)>,
    S: Term,
    A: Term,
{
}

impl<S, MA, A, MB, B> ChainM<B> for StateT<S, MA>
where
    S: Term,
    MA: Monad<(B, S), Pointed = (A, S), Chained = MB>,
    MB: Monad<(A, S), Pointed = (B, S), Chained = MA>,
    A: Term,
    B: Term,
{
    type Chained = StateT<S, MB>;

    fn chain_m(self, k: impl FunctionT<Self::Pointed, StateT<S, MB>>) -> StateT<S, MB>
    where
        StateT<S, MB>: Clone,
    {
        let m = self;
        let k = k.to_function();
        StateT::new_t(|s| m.run_t(s).chain_m(|(a, s_)| k(a).run_t(s_)))
    }
}

impl<S, MA, A, MB, MC> TraverseT<MB, (), MC> for StateT<S, MA>
where
    S: Term,
    MA: Pointed<Pointed = (A, S)>,
    A: Term,
    MB: Term,
    MC: Term,
{
    type Mapped = MB;
    type Value = A;
    type Traversed = MC;

    fn traverse_t(self, f: impl FunctionT<Self::Pointed, MB>) -> MC {
        todo!()
    }
}

impl<S, MB, B, MC> SequenceA<(), MC> for StateT<S, MB>
where
    S: TraverseT<MB, (), MC>,
    MB: Pointed<Pointed = (B, S)>
        + WithPointed<(B, S)>
        + WithPointed<Function<StateT<S, MB>, StateT<S, <MB as WithPointed<(B, S)>>::WithPointed>>>,
    MC: Term,
    B: Term,
{
    type Inner = MB;
    type Value = B;
    type Sequenced = MC;

    fn sequence_a(self) -> MC {
        todo!()
    }
}

impl<MA, S, A> MonadTrans<MA::Lowered> for StateT<S, MA>
where
    MA: MonadLower<A, S> + ReturnM<Pointed = (A, S)>,
    MA::Lowered: Monad<(A, S), Pointed = A, Chained = MA>,
    S: Term,
    A: Term,
{
    fn lift(m: MA::Lowered) -> Self {
        StateT::new_t(|s| m.chain_m(|a| ReturnM::return_m((a, s))))
    }
}

impl<MA, S, A> MonadIO<A> for StateT<S, MA>
where
    MA: ReturnM<Pointed = (A, S)> + MonadLower<A, S>,
    MA::Lowered: Monad<(A, S), Pointed = A, Chained = MA> + MonadIO<A>,
    S: Term,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        StateT::lift(MonadIO::lift_io(m))
    }
}

#[cfg(test)]
mod test {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TurnstileState {
        Locked,
        Unlocked,
    }

    use crate::{
        base::control::monad::{FilterM, FoldM, ReplicateM},
        prelude::*,
    };
    use TurnstileState::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TurnstileOutput {
        Thank,
        Open,
        Tut,
    }

    use TurnstileOutput::*;

    fn coin(_: TurnstileState) -> (TurnstileOutput, TurnstileState) {
        (TurnstileOutput::Thank, TurnstileState::Unlocked)
    }

    fn push(state: TurnstileState) -> (TurnstileOutput, TurnstileState) {
        match state {
            TurnstileState::Locked => (TurnstileOutput::Tut, TurnstileState::Locked),
            TurnstileState::Unlocked => (TurnstileOutput::Open, TurnstileState::Locked),
        }
    }

    fn monday(s0: TurnstileState) -> ([TurnstileOutput; 5], TurnstileState) {
        let (a1, s1) = coin(s0);
        let (a2, s2) = push(s1);
        let (a3, s3) = push(s2);
        let (a4, s4) = coin(s3);
        let (a5, s5) = push(s4);
        ([a1, a2, a3, a4, a5], s5)
    }

    fn coin_s() -> State<TurnstileState, TurnstileOutput> {
        State::new(coin)
    }

    fn push_s() -> State<TurnstileState, TurnstileOutput> {
        State::new(push)
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum TurnstileInput {
        Coin,
        Push,
    }

    use TurnstileInput::*;

    use super::State;

    fn turn(input: TurnstileInput, state: TurnstileState) -> (TurnstileOutput, TurnstileState) {
        match (input, state) {
            (Coin, _) => (Thank, Unlocked),
            (Push, Unlocked) => (Open, Locked),
            (Push, Locked) => (Tut, Locked),
        }
    }

    fn turn_s(input: TurnstileInput) -> State<TurnstileState, TurnstileOutput> {
        State::new(move |state| turn(input, state))
    }

    #[test]
    fn test_monday() {
        let out = monday(Locked);
        println!("{out:#?}");
    }

    #[test]
    fn test_coin_s() {
        let out = coin_s().run_t(Locked);
        println!("{out:#?}");
    }

    #[test]
    fn test_monday_s() {
        let out = vec![
            turn_s(Coin),
            turn_s(Push),
            turn_s(Push),
            turn_s(Coin),
            turn_s(Push),
        ]
        .sequence_a()
        .run_t(Locked);

        println!("{out:#?}");
    }

    #[test]
    fn test_turnstile() {
        let out = State::<TurnstileState, _>::put(Locked)
            .then_m(push_s().chain_m(move |check1| {
                State::<TurnstileState, _>::put(Unlocked).then_m(push_s().chain_m(move |check2| {
                    State::<TurnstileState, _>::put(Locked)
                        .then_m(ReturnM::return_m(check1 == Tut && check2 == Open))
                }))
            }))
            .run(Unlocked);

        println!("{out:#?}");
    }

    #[test]
    fn test_replicate() {
        let out = push_s().replicate_m(6).eval(Unlocked);
        println!("{out:#?}");
    }

    #[test]
    fn test_map_m() {
        let out = vec![Coin, Push, Push, Coin, Push]
            .map_m(turn_s)
            .eval(Locked);

        println!("{out:#?}");
    }

    fn gets_through_s(input: TurnstileInput) -> State<TurnstileState, bool> {
        turn_s(input).chain_m(|output| ReturnM::return_m(output == Open))
    }

    #[test]
    fn test_filter_m() {
        let out = vec![Coin, Push, Coin, Push, Push, Coin, Push]
            .filter_m(gets_through_s)
            .eval(Locked);

        println!("{out:#?}")
    }

    fn inc_if_opens(n: usize, i: TurnstileInput) -> State<TurnstileState, usize> {
        gets_through_s(i).chain_m(move |g| {
            if g {
                ReturnM::return_m(n + 1)
            } else {
                ReturnM::return_m(n)
            }
        })
    }

    fn count_opens(input: Vec<TurnstileInput>) -> State<TurnstileState, usize> {
        input.foldl_m(inc_if_opens, 0)
    }

    #[test]
    fn test_fold_m() {
        let out = count_opens(vec![Coin, Push, Coin, Push, Push, Coin, Push]).eval(Locked);
        println!("{out:#?}");
    }
}
