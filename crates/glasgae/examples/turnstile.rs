use glasgae::{
    base::{
        control::monad::{FilterM, FoldM},
        data::functor::identity::Identity,
    },
    mtl::state::*,
    prelude::*,
    transformers::{class::MonadTrans, reader::ReaderT, state::StateT, writer::WriterT},
};
use std::fmt::Debug;

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

impl<R, W, S, T> RunMyComplexMonad<R> for MyComplexMonad<R, W, S, T>
where
    R: Term,
    W: Term,
    S: Term,
    T: Term,
{
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

fn log_w<MA, A>(m: WriterT<Vec<String>, MA>) -> WriterT<Vec<String>, MA>
where
    MA: Monad<(A, Vec<String>), Pointed = (A, Vec<String>), Chained = MA>,
    A: Term + Debug,
{
    _do! {
        a <- m;
        WriterT::new((a.clone(), vec![format!("{a:?}")]))
    }
}

fn test_type_name() -> IO<()> {
    type S = StateT<TurnstileState, Identity<((TurnstileOutput, Vec<String>), TurnstileState)>>;
    type W = WriterT<Vec<String>, S>;
    type R = ReaderT<usize, W>;

    type Point = PointedT<W>;
    type S_ = <S as MonadLower<PointedT<W>, Vec<String>>>::Lowered;
    type Point_ = PointedT<S>;

    _do! {
        print(format!("R: {}", std::any::type_name::<R>()));
        print(format!("W: {}", std::any::type_name::<W>()));
        print(format!("W: {}", std::any::type_name::<S>()));

        print(format!("W Pointed: {}", std::any::type_name::<Point>()));
        print(format!("S Lowered: {}", std::any::type_name::<S_>()));
        print(format!("S Value: {}", std::any::type_name::<Point_>()))
    }
}

fn coin_s() -> MyComplexMonad<usize, Vec<String>, TurnstileState, TurnstileOutput> {
    MyComplexMonad::state(coin)
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
    _do! {
        print("CoinS:");
        print(
            coin_s()
            .run((1, Locked))
        )
    }
}

fn test_monday_s_reader() -> IO<()> {
    _do! {
        print("MondayS:");
        print(
            vec![
                turn_s(Coin),
                turn_s(Push),
                turn_s(Push),
                turn_s(Coin),
                turn_s(Push),
            ]
            .sequence_a()
            .run((1, Locked))
        )
    }
}

fn test_turnstile_reader() -> IO<()> {
    _do! {
        print("Turnstile:");
        let out = _do!{
                      MyComplexMonad::put(Locked);
                      check1 <- push_s();
                      MyComplexMonad::put(Unlocked);
                      check2 <- push_s();
                      MyComplexMonad::put(Locked);
                      ReturnM::return_m(check1 == Tut && check2 == Open)
                  }
                  .run((1, Unlocked));

        print(out)
    }
}

fn test_replicate_reader() -> IO<()> {
    _do! {
        print("ReplicateM:");
        print(
            push_s()
            .replicate_m(6)
            .run((1, Unlocked))
        )
    }
}

fn test_map_m_reader() -> IO<()> {
    _do! {
        print("MapM:");
        print(
            vec![Coin, Push, Push, Coin, Push]
           .traverse_t(turn_s)
           .run((1, Locked))
        )
    }
}

fn gets_through_s(
    input: TurnstileInput,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, bool> {
    _do! {
        output <- turn_s(input);
        ReturnM::return_m(output == Open)
    }
}

fn test_filter_m_reader() -> IO<()> {
    _do! {
        print("FilterM:");
        print(
            vec![Coin, Push, Coin, Push, Push, Coin, Push]
            .filter_m(|t| gets_through_s(t).map_t(log_w))
            .run((1, Locked))
        )
    }
}

fn inc_if_opens(
    n: usize,
    i: TurnstileInput,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, usize> {
    _do! {
        g <- gets_through_s(i).map_t(log_w);
        if g {
            ReturnM::return_m(n + 1)
        } else {
            ReturnM::return_m(n)
        }
    }
}

fn count_opens(
    input: Vec<TurnstileInput>,
) -> MyComplexMonad<usize, Vec<String>, TurnstileState, usize> {
    input.foldl_m(inc_if_opens, 0)
}

fn test_fold_m_reader() -> IO<()> {
    _do! {
        print("FoldM:");
        print(
            count_opens(vec![
                Coin,
                Push,
                Coin,
                Push,
                Push,
                Coin,
                Push
            ])
            .run((1, Locked))
        )
    }
}

fn main() -> IO<()> {
    _do! {
        test_type_name();
        test_coin_s_reader();
        print("");
        test_monday_s_reader();
        print("");
        test_turnstile_reader();
        print("");
        test_replicate_reader();
        print("");
        test_map_m_reader();
        print("");
        test_filter_m_reader();
        print("");
        test_fold_m_reader()
    }
}
