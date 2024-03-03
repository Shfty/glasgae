//! Analogous to `Control.Monad.Trans` from Haskell `transformers`.

pub mod class;
pub mod cont;
pub mod reader;
pub mod state;
pub mod writer;

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Display};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TurnstileState {
        Locked,
        Unlocked,
    }

    impl Display for TurnstileState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
        }
    }

    use TurnstileState::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TurnstileOutput {
        Thank,
        Open,
        Tut,
    }

    impl Display for TurnstileOutput {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
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

    use crate::{
        base::{
            control::monad::{FilterM, FoldM, ReplicateM},
            data::{functor::identity::Identity, pointed::Lower},
        },
        prelude::{ChainM, Curry, MapM, PointedT, ReturnM, SequenceA, ThenM}, mtl::state::{MonadState, MonadPut},
    };

    use super::{reader::ReaderT, state::StateT, writer::WriterT};

    type MyComplexMonad<R, W, S, T> = ReaderT<R, WriterT<W, StateT<S, Identity<((T, W), S)>>>>;

    pub trait RunMyComplexMonad<R> {
        type Input;
        type Output;

        fn run(self, input: Self::Input) -> Self::Output;
    }

    impl<R, W, S, T> RunMyComplexMonad<R> for MyComplexMonad<R, W, S, T> {
        type Input = (R, S);
        type Output = ((T, W), S);

        fn run(self, (r, s): Self::Input) -> Self::Output {
            self.run_t(r).run_t().run_t(s).run()
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

    #[test]
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

    #[test]
    fn test_coin_s_reader() {
        let out = coin_s().run((1, Locked));
        println!("{out:#?}");
    }

    #[test]
    fn test_monday_s_reader() {
        let out = vec![
            turn_s(Coin),
            turn_s(Push),
            turn_s(Push),
            turn_s(Coin),
            turn_s(Push),
        ]
        .sequence_a()
        .run((1, Locked));

        println!("{out:#?}");
    }

    #[test]
    fn test_turnstile_reader() {
        let out = MyComplexMonad::put(Locked)
            .then_m(push_s().chain_m(move |check1| {
                MyComplexMonad::put(Unlocked).then_m(push_s().chain_m(move |check2| {
                    MyComplexMonad::put(Locked)
                        .then_m(ReturnM::return_m(check1 == Tut && check2 == Open))
                }))
            }))
            .run((1, Unlocked));

        println!("{out:#?}");
    }

    #[test]
    fn test_replicate_reader() {
        let out = push_s().replicate_m(6).run((1, Unlocked));
        println!("{out:#?}");
    }

    #[test]
    fn test_map_m_reader() {
        let out = vec![Coin, Push, Push, Coin, Push]
            .map_m(turn_s)
            .run((1, Locked));

        println!("{out:#?}");
    }

    fn gets_through_s(
        input: TurnstileInput,
    ) -> MyComplexMonad<usize, Vec<String>, TurnstileState, bool> {
        turn_s(input).chain_m(|output| ReturnM::return_m(output == Open))
    }

    #[test]
    fn test_filter_m_reader() {
        let out = vec![Coin, Push, Coin, Push, Push, Coin, Push]
            .filter_m(|t| gets_through_s(t).map_t(log_w))
            .run((1, Locked));

        println!("{out:#?}")
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

    #[test]
    fn test_fold_m_reader() {
        let out = count_opens(vec![Coin, Push, Coin, Push, Push, Coin, Push]).run((1, Locked));
        println!("{out:#?}");
    }
}
