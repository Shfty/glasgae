//! Adding a new kind of pure computation to an applicative functor.

use crate::{derive_pointed_via, derive_with_pointed_via, prelude::*};

/// Applicative functor formed by adding pure computations to a given applicative functor.
#[derive(Clone)]
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
    pub fn elim<R>(self, f: impl FunctionT<FA::Pointed, R>, g: impl FunctionT<FA, R>) -> R
    where
        R: Term,
    {
        match self {
            Pure(x) => f(x),
            Other(e) => g(e),
        }
    }
}

derive_pointed_via!(Lift<(FA)>);
derive_with_pointed_via!(Lift<(FA)>);

impl<FA, A, FB, B> Functor<B> for Lift<FA>
where
    FA: Functor<B, Pointed = A, Mapped = FB>,
    FB: Functor<A, Pointed = B, Mapped = FA>,
    A: Term,
    B: Term,
{
    type Mapped = Lift<FA::Mapped>;

    fn fmap(self, f: impl FunctionT<A, B>) -> Self::Mapped {
        match self {
            Pure(x) => Pure(f(x)),
            Other(y) => Other(y.fmap(f)),
        }
    }
}

impl<FA> PureA for Lift<FA>
where
    FA: Pointed,
{
    fn pure_a(t: Self::Pointed) -> Self {
        Pure(t)
    }
}

impl<FF, F, FA, A, FB, B> AppA<A, B> for Lift<FF>
where
    FF: WithPointed<A, WithPointed = FA> + WithPointed<B, WithPointed = FB>,
    FA: PureA<Pointed = A>
        + Functor<B, Pointed = A, Mapped = FB>
        + WithPointed<F, WithPointed = FF>,
    FB: Functor<A, Pointed = B, Mapped = FA> + WithPointed<F, WithPointed = FF>,
    FF: Pointed<Pointed = F> + Applicative<FA, FB, WithA = FA, WithB = FB>,
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = Lift<FA>;
    type WithB = Lift<FB>;

    fn app_a(self, a: Lift<FA>) -> Lift<FB> {
        match self {
            Pure(f) => a.fmap(f),
            Other(f) => Other(f.app_a(a.unlift())),
        }
    }
}

impl<FA, A, B> FoldMap<B> for Lift<FA>
where
    FA: Pointed<Pointed = A> + FoldMap<B>,
    A: Term,
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B>) -> B {
        let f = f.to_function();
        match self {
            Pure(x) => f(x),
            Other(y) => y.fold_map(f),
        }
    }
}

impl<FA, B> Foldable<B> for Lift<FA>
where
    FA: Pointed,
{
    fn foldr(self, f: impl BifunT<Self::Pointed, B, B>, z: B) -> B {
        todo!()
    }

    fn foldl(self, f: impl BifunT<B, Self::Pointed, B>, z: B) -> B {
        todo!()
    }
}

impl<FA, A1, A2> TraverseT<A1, (), A2> for Lift<FA>
where
    Self: Functor<A1>,
    MappedT<Self, A1>: SequenceA<(), A2, Sequenced = A2>,
    FA: Pointed,
    A1: Pointed,
    A2: Term,
{
    type Mapped = A1;
    type Value = PointedT<A1>;
    type Traversed = A2;

    fn traverse_t(self, f: impl FunctionT<Self::Pointed, Self::Mapped>) -> Self::Traversed {
        traverse_t_default(self, f)
    }
}

impl<FA, A, A2> SequenceA<(), A2> for Lift<FA>
where
    FA: Pointed<Pointed = A>
        + WithPointed<A>
        + WithPointed<Function<Lift<FA>, Lift<<FA as WithPointed<A>>::WithPointed>>>,
    A: Term,
    A2: Term,
{
    type Inner = FA;
    type Value = A;
    type Sequenced = A2;

    fn sequence_a(self) -> A2 {
        todo!()
    }
}
