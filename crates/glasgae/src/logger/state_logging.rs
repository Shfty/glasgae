use crate::{
    base::data::{function::bifunction::BifunT, tuple::pair::Pair},
    prelude::*,
    transformers::{class::MonadTrans, state::StateT},
};

use super::{LoggingT, MonadLogger};

pub type StateLogging<LVL, MSG, S, T> = StateLoggingT<LVL, MSG, S, IO<(T, S)>>;

pub type StateLoggingT<LVL, MSG, S, MA> = StateT<S, LoggingT<LVL, (MSG, S), MA>>;

impl<LVL, MSG> MonadLogger<LVL, MSG> for StateLoggingT<LVL, MSG, usize, IO<((), usize)>>
where
    LVL: Clone,
    MSG: Clone,
{
    fn log(level: LVL, message: MSG) -> Self {
        StateLoggingT::<LVL, MSG, usize, IO<(usize, usize)>>::get()
            .chain_m(move |s| MonadTrans::lift(LoggingT::log(level, (message, s))))
    }
}

pub trait RunStateLogging<LVL, MSG, MA> {
    fn run(self, f: impl BifunT<LVL, MSG, IO<()>> + Clone) -> MA;
}

impl<LVL, MSG, MA, MB, S, T> RunStateLogging<LVL, (MSG, S), MB> for StateLoggingT<LVL, MSG, S, MA>
where
    MA: Pointed<Pointed = (T, S)> + ChainM<MB>,
    MB: 'static + ReturnM<Pointed = T>,
    S: Clone + Default,
    T: 'static + Clone,
{
    fn run(self, f: impl BifunT<LVL, (MSG, S), IO<()>> + Clone) -> MB {
        self.run_t(Default::default())
            .run_t(f)
            .chain_m(Pair::fst.compose_clone(ReturnM::return_m))
    }
}

pub fn indent<LVL, MSG, MA>() -> StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Clone,
    MSG: Clone,
    MA: Clone + ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: Clone + ReturnM<Pointed = usize> + ChainM<MA>,
{
    StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
        LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s + 1)
    })
}

pub fn unindent<LVL, MSG, MA>() -> StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Clone,
    MSG: Clone,
    MA: Clone + ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: Clone + ReturnM<Pointed = usize> + ChainM<MA>,
{
    StateLoggingT::<LVL, MSG, usize, MA>::modify_m(|s| {
        LoggingT::<LVL, (MSG, usize), MA::WithPointed>::return_m(s - 1)
    })
}

pub fn log_scope<LVL, MSG, MA>(
    m: StateLoggingT<LVL, MSG, usize, MA>,
) -> StateLoggingT<LVL, MSG, usize, MA>
where
    LVL: Clone,
    MSG: Clone,
    MA: Clone + ReturnM<Pointed = ((), usize)> + WithPointed<usize>,
    MA::WithPointed: Clone + ReturnM<Pointed = usize> + ChainM<MA>,
    StateLoggingT<LVL, MSG, usize, MA>: ThenM<StateLoggingT<LVL, MSG, usize, MA>>,
{
    indent().then_m(m).then_m(unindent())
}
