use glasgae::{
    base::{
        control::monad::{FilterM, FoldM, ReplicateM},
        data::{functor::identity::Identity, pointed::Lower},
    },
    mtl::state::{MonadPut, MonadState},
    prelude::{
        print, ChainM, Curry, Flip, Functor, MapM, PointedT, ReturnM, SequenceA, Show, ThenM, IO,
    },
    transformers::{reader::ReaderT, state::StateT, writer::WriterT},
};
use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TurnstileState {
    Locked,
    Unlocked,
}

impl Show for TurnstileState {
    fn show(self) -> String {
        format!("{:#?}", self)
    }
}

use TurnstileState::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TurnstileOutput {
    Thank,
    Open,
    Tut,
}

impl Show for TurnstileOutput {
    fn show(self) -> String {
        format!("{self:#?}")
    }
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

/*
fn monday(s0: TurnstileState) -> ([TurnstileOutput; 5], TurnstileState) {
    let (a1, s1) = coin(s0);
    let (a2, s2) = push(s1);
    let (a3, s3) = push(s2);
    let (a4, s4) = coin(s3);
    let (a5, s5) = push(s4);
    ([a1, a2, a3, a4, a5], s5)
}
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TurnstileInput {
    Coin,
    Push,
}

use TurnstileInput::*;

impl Show for TurnstileInput {
    fn show(self) -> String {
        format!("{self:#?}")
    }
}

type MyComplexMonad<R, W, S, T> = ReaderT<R, WriterT<W, StateT<S, Identity<((T, W), S)>>>>;

pub trait RunMyComplexMonad<R> {
    type Input;
    type Output;
    type OutputT;

    fn run(self, input: Self::Input) -> Self::Output;
    fn run_t(self, input: Self::Input) -> Self::OutputT;
}

impl<R, W, S, T> RunMyComplexMonad<R> for MyComplexMonad<R, W, S, T> {
    type Input = (R, S);
    type Output = ((T, W), S);
    type OutputT = Identity<Self::Output>;

    fn run(self, input: Self::Input) -> Self::Output {
        RunMyComplexMonad::run_t(self, input).run()
    }

    fn run_t(self, (r, s): Self::Input) -> Self::OutputT {
        self.run_t(r).run_t().run_t(s)
    }
}

fn log_w<MA, MB>(t: MA) -> WriterT<Vec<String>, MB>
where
    MA: ChainM<WriterT<Vec<String>, MB>>,
    MA::Pointed: Clone + Debug,
    MB: ReturnM<Pointed = (MA::Pointed, Vec<String>)>,
{
    t.chain_m(|t| WriterT::new((t.clone(), vec![format!("{t:?}")])))
}

fn test_type_name() {
    type S = StateT<TurnstileState, Identity<((TurnstileOutput, Vec<String>), TurnstileState)>>;
    type W = WriterT<Vec<String>, S>;
    type R = ReaderT<usize, W>;
    println!("R: {}", std::any::type_name::<R>());
    println!("W: {}", std::any::type_name::<W>());
    println!("W: {}", std::any::type_name::<S>());

    type Point = PointedT<W>;
    println!("W Pointed: {}", std::any::type_name::<Point>());

    type S_ = <S as Lower<Vec<String>, PointedT<W>>>::Lowered;
    println!("S Lowered: {}", std::any::type_name::<S_>());

    type Point_ = PointedT<S>;
    println!("S Value: {}", std::any::type_name::<Point_>());
}

fn coin_s() -> MyComplexMonad<usize, Vec<String>, TurnstileState, TurnstileOutput> {
    MyComplexMonad::state(coin).map_t(log_w)
}

fn push_s() -> MyComplexMonad<usize, Vec<String>, TurnstileState, TurnstileOutput> {
    MyComplexMonad::state(push).map_t(log_w)
}

fn turn(input: TurnstileInput, state: TurnstileState) -> (TurnstileOutput, TurnstileState) {
    match (input, state) {
        (Coin, _) => (Thank, Unlocked),
        (Push, Unlocked) => (Open, Locked),
        (Push, Locked) => (Tut, Locked),
    }
}

fn turn_s(
    input: TurnstileInput,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, TurnstileOutput> {
    MyComplexMonad::state(turn.curry_clone(input))
}

fn test_coin_s_reader() -> IO<()> {
    print("CoinS:")
        .then_m(IO::return_m(coin_s().run((1, Locked))))
        .chain_m(print)
}

fn test_monday_s_reader() -> IO<()> {
    print("MondayS:")
        .then_m(IO::return_m(
            vec![
                turn_s(Coin),
                turn_s(Push),
                turn_s(Push),
                turn_s(Coin),
                turn_s(Push),
            ]
            .sequence_a()
            .run((1, Locked)),
        ))
        .chain_m(print)
}

fn test_turnstile_reader() -> IO<()> {
    print("Turnstile:")
        .then_m(IO::return_m(MyComplexMonad::put(Locked).then_m(
            push_s().chain_m(move |check1| {
                MyComplexMonad::put(Unlocked).then_m(push_s().chain_m(move |check2| {
                    MyComplexMonad::put(Locked)
                        .then_m(ReturnM::return_m(check1 == Tut && check2 == Open))
                }))
            }),
        )))
        .fmap(
            RunMyComplexMonad::run
                .flip_clone()
                .curry_clone((1, Unlocked)),
        )
        .chain_m(print)
}

fn test_replicate_reader() -> IO<()> {
    print("ReplicateM:")
        .then_m(IO::return_m(push_s().replicate_m(6).run((1, Unlocked))))
        .chain_m(print)
}

fn test_map_m_reader() -> IO<()> {
    print("MapM:")
        .then_m(IO::return_m(
            vec![Coin, Push, Push, Coin, Push]
                .map_m(turn_s)
                .run((1, Locked)),
        ))
        .chain_m(print)
}

fn gets_through_s(
    input: TurnstileInput,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, bool> {
    turn_s(input).chain_m(|output| ReturnM::return_m(output == Open))
}

fn test_filter_m_reader() -> IO<()> {
    print("FilterM:")
        .then_m(IO::return_m(
            vec![Coin, Push, Coin, Push, Push, Coin, Push]
                .filter_m(|t| gets_through_s(t).map_t(log_w))
                .run((1, Locked)),
        ))
        .chain_m(print)
}

fn inc_if_opens(
    n: usize,
    i: TurnstileInput,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, usize> {
    gets_through_s(i).map_t(log_w).chain_m(move |g| {
        if g {
            ReturnM::return_m(n + 1)
        } else {
            ReturnM::return_m(n)
        }
    })
}

fn count_opens(
    input: Vec<TurnstileInput>,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, usize> {
    input.fold_m(inc_if_opens, 0)
}

fn test_fold_m_reader() -> IO<()> {
    print("FoldM:")
        .then_m(IO::return_m(
            count_opens(vec![Coin, Push, Coin, Push, Push, Coin, Push]).run((1, Locked)),
        ))
        .chain_m(print)
}

fn main() -> IO<()> {
    test_coin_s_reader()
        .then_m(print(""))
        .then_m(test_monday_s_reader())
        .then_m(print(""))
        .then_m(test_turnstile_reader())
        .then_m(print(""))
        .then_m(test_replicate_reader())
        .then_m(print(""))
        .then_m(test_map_m_reader())
        .then_m(print(""))
        .then_m(test_filter_m_reader())
        .then_m(print(""))
        .then_m(test_fold_m_reader())
}
