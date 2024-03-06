//! Adding a new kind of pure computation to an applicative functor.

use crate::{
    base::data::{foldr_default, function::bifunction::BifunT, monoid::Endo, FoldMap},
    prelude::{
        AppA, Foldr, Function, FunctionT, Functor, Monoid, Pointed, PureA, SequenceA, TraverseT,
        WithPointed,
    },
};

/// Applicative functor formed by adding pure computations to a given applicative functor.
pub enum Lift<FA>
where
    FA: Pointed,
{
    Pure(FA::Pointed),
    Other(FA),
}

use Lift::*;

impl<FA> Lift<FA>
where
    FA: Pointed,
{
    /// Projection to the other functor.
    pub fn unlift(self) -> FA
    where
        FA: PureA,
    {
        match self {
            Lift::Pure(x) => PureA::pure_a(x),
            Lift::Other(e) => e,
        }
    }

    /// Apply a transformation to the other computation.
    pub fn map<FG>(self, f: impl FunctionT<FA, FG>) -> Lift<FG>
    where
        FG: Pointed<Pointed = FA::Pointed>,
    {
        match self {
            Lift::Pure(x) => Pure(x),
            Lift::Other(e) => Other(f(e)),
        }
    }

    /// Eliminator for Lift.
    /// ```text
    /// elimLift f g . pure = f
    /// ```
    /// ```text
    /// elimLift f g . Other = g
    /// ```
    pub fn elim<R>(self, f: impl FunctionT<FA::Pointed, R>, g: impl FunctionT<FA, R>) -> R {
        match self {
            Pure(x) => f(x),
            Other(e) => g(e),
        }
    }
}

impl<FA> Pointed for Lift<FA>
where
    FA: Pointed,
{
    type Pointed = FA::Pointed;
}

impl<FA, A, B> Functor<B> for Lift<FA>
where
    B: Clone,
    FA: Functor<B, Pointed = A>,
{
    fn fmap(self, f: impl FunctionT<A, B> + Clone) -> Self::WithPointed {
        match self {
            Pure(x) => Pure(f(x)),
            Other(y) => Other(y.fmap(f)),
        }
    }
}

impl<FA, B> WithPointed<B> for Lift<FA>
where
    FA: WithPointed<B>,
{
    type WithPointed = Lift<FA::WithPointed>;
}

impl<FA> PureA for Lift<FA>
where
    FA: Pointed,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Pure(t)
    }
}

impl<FF, FA, FB, F, A, B> AppA<Lift<FA>, Lift<FB>> for Lift<FF>
where
    Lift<FA>: Functor<B, Pointed = A, WithPointed = Lift<FB>>,
    FA: PureA<Pointed = A>,
    FB: Pointed<Pointed = B>,
    FF: Pointed<Pointed = F> + AppA<FA, FB>,
    F: Clone + FunctionT<A, B>,
    B: Clone,
{
    fn app_a(self, a: Lift<FA>) -> Lift<FB> {
        match self {
            Pure(f) => a.fmap(f),
            Other(f) => Other(f.app_a(a.unlift())),
        }
    }
}

impl<FA, A, B> FoldMap<A, B> for Lift<FA>
where
    B: Monoid,
    FA: Pointed<Pointed = A> + FoldMap<A, B>,
{
    fn fold_map(self, f: impl FunctionT<A, B> + Clone) -> B {
        match self {
            Pure(x) => f(x),
            Other(y) => y.fold_map(f),
        }
    }
}

impl<FA, A, B> Foldr<A, B> for Lift<FA>
where
    Self: FoldMap<A, Endo<Function<B, B>>>,
    FA: Pointed,
    Endo<B>: Monoid,
    A: 'static + Clone,
{
    fn foldr(self, f: impl BifunT<A, B, B> + Clone, z: B) -> B {
        foldr_default::<Self, A, B>(self, f, z)
    }
}

impl<FA, A1, A_, A2> TraverseT<A1, A_, A2> for Lift<FA>
where
    FA: Pointed,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A2 {
        todo!()
    }
}

impl<FA, A_, A2> SequenceA<A_, A2> for Lift<FA>
where
    FA: Pointed,
{
    fn sequence_a(self) -> A2 {
        todo!()
    }
}
