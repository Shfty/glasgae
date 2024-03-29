use crate::{derive_pointed_via, prelude::*};

use super::Functor;

/// The Const functor.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const<MA>(pub MA);

impl<MA> Const<MA> {
    pub fn get(self) -> MA {
        self.0
    }
}

derive_pointed_via!(Const<(MA)>);

impl<MA, T> WithPointed<T> for Const<MA>
where
    MA: Pointed<Pointed = T>,
{
    type WithPointed = Self;
}

impl<MA, A> Functor<A> for Const<MA>
where
    MA: Pointed<Pointed = A> + WithPointed<A>,
    A: Term,
{
    type Mapped = Const<MA>;

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
