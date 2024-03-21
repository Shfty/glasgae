//! The identity monad transformer.
//!
//! This is useful for functions parameterized by a monad transformer.

use crate::{base::control::monad::io::MonadIO, prelude::*, derive_kinded_unary, derive_with_kinded_unary};

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

derive_kinded_unary!(IdentityT<MA>);
derive_with_kinded_unary!(IdentityT<MA>);

impl<MA> Pointed for IdentityT<MA>
where
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<MA, T> WithPointed<T> for IdentityT<MA>
where
    MA: WithPointed<T>,
{
    type WithPointed = IdentityT<MA::WithPointed>;
}

impl<MA, T> Fmap<T> for IdentityT<MA>
where
    MA: Fmap<T>,
    T: Term,
{
    fn fmap(self, f: impl crate::prelude::FunctionT<Self::Pointed, T>) -> Self::WithPointed {
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

impl<MA, A1, A2> AppA<IdentityT<A1>, IdentityT<A2>> for IdentityT<MA>
where
    MA: AppA<A1, A2>,
    A1: Term,
    A2: Term,
{
    fn app_a(self, a: IdentityT<A1>) -> IdentityT<A2> {
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

impl<MA, MB> ChainM<IdentityT<MB>> for IdentityT<MA>
where
    MA: ChainM<MB>,
    MB: Term,
{
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
