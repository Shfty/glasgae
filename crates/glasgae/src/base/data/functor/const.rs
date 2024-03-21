use crate::{prelude::*, derive_with_kinded_unary, derive_kinded_unary};

use super::Fmap;

/// The Const functor.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const<MA>(pub MA);

impl<MA> Const<MA> {
    pub fn get(self) -> MA {
        self.0
    }
}

derive_kinded_unary!(Const<T>);
derive_with_kinded_unary!(Const<T>);

impl<MA> Pointed for Const<MA>
where
    MA: Pointed,
{
    type Pointed = MA::Pointed;
}

impl<MA, T> WithPointed<T> for Const<MA>
where
    MA: Pointed<Pointed = T>,
{
    type WithPointed = Self;
}

impl<MA, A> Fmap<A> for Const<MA>
where
    MA: Pointed<Pointed = A> + WithPointed<A>,
    A: Term,
{
    fn fmap(self, _: impl FunctionT<Self::Pointed, A>) -> Self::WithPointed {
        self
    }
}

impl<MA> Semigroup for Const<MA>
where
    MA: Semigroup,
{
    fn assoc_s(self, a: Self) -> Self {
        Const(self.0.assoc_s(a.0))
    }
}

impl<MA> Monoid for Const<MA>
where
    MA: 'static + Monoid,
{
    fn mempty() -> Self {
        Const(MA::mempty())
    }
}
