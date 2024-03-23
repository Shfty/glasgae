//! The identity monad transformer.
//!
//! This is useful for functions parameterized by a monad transformer.

use crate::{
    base::control::monad::io::MonadIO, derive_pointed_via, derive_with_pointed_via, prelude::*,
};

use super::class::MonadTrans;

/// The trivial monad transformer, which maps a monad to an equivalent monad.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentityT<MA>(MA);

impl<MA> IdentityT<MA>
where
    MA: Term,
{
    pub fn new(ma: MA) -> Self {
        IdentityT(ma)
    }

    pub fn run(self) -> MA {
        self.0
    }

    /// Lift a unary operation to the new monad.
    pub fn map<MB>(self, f: impl FunctionT<MA, MB>) -> IdentityT<MB>
    where
        MA: Pointed,
        MB: Term,
    {
        IdentityT(f(self.run()))
    }
}

derive_pointed_via!(IdentityT<(MA)>);
derive_with_pointed_via!(IdentityT<(MA)>);

impl<MA, T> Functor<T> for IdentityT<MA>
where
    MA: Functor<T>,
    T: Term,
{
    type Mapped = IdentityT<MA::Mapped>;

    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, T>) -> Self::Mapped {
        IdentityT(self.0.fmap(f))
    }
}

impl<MA> PureA for IdentityT<MA>
where
    MA: PureA,
{
    fn pure_a(t: Self::Pointed) -> Self {
        IdentityT(PureA::pure_a(t))
    }
}

impl<MF, F, MA, A, MB, B> AppA<A, B> for IdentityT<MF>
where
    MF: Pointed<Pointed = F>
        + Applicative<A, B, WithA = MA, WithB = MB>
        + WithPointed<A, WithPointed = MA>
        + WithPointed<B, WithPointed = MB>,
    MA: WithPointed<F, Pointed = A, WithPointed = MF>,
    MB: WithPointed<F, Pointed = B, WithPointed = MF>,
    F: Term,
    A: Term,
    B: Term,
{
    type WithA = IdentityT<MA>;
    type WithB = IdentityT<MB>;

    fn app_a(self, a: IdentityT<MA>) -> IdentityT<MB> {
        IdentityT(self.run().app_a(a.run()))
    }
}

impl<MA> ReturnM for IdentityT<MA>
where
    MA: ReturnM,
{
    fn return_m(t: Self::Pointed) -> Self
    where
        Self: Sized,
    {
        IdentityT(ReturnM::return_m(t))
    }
}

impl<MA, A, MB, B> ChainM<B> for IdentityT<MA>
where
    MA: Monad<B, Pointed = A, Chained = MB>,
    MB: Monad<A, Pointed = B, Chained = MA>,
    A: Term,
    B: Term,
{
    type Chained = IdentityT<MB>;

    fn chain_m(
        self,
        k: impl crate::prelude::FunctionT<Self::Pointed, IdentityT<MB>>,
    ) -> IdentityT<MB> {
        let m = self;
        let k = k.to_function();
        IdentityT(m.run().chain_m(|t| k(t).run()))
    }
}

impl<MA> MonadTrans<MA> for IdentityT<MA>
where
    MA: Term,
{
    fn lift(m: MA) -> Self {
        IdentityT::new(m)
    }
}

impl<MA, A> MonadIO<A> for IdentityT<MA>
where
    Self: MonadTrans<IO<A>>,
    MA: Pointed<Pointed = A>,
    A: Term,
{
    fn lift_io(m: IO<A>) -> Self {
        Self::lift(MonadIO::lift_io(m))
    }
}
