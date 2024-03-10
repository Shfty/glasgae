//! A class for categories.
//!
//! Instances should satisfy the following laws:
//!
//! **Right identity**
//! ```text
//! f . id = f
//! ```
//!
//! **Left identity**
//! ```text
//! id . f = f
//! ```
//!
//! **Associativity**
//! ```text
//! f . (g . h) = (f . g) . h
//! ```

use std::convert::identity;

use crate::{
    base::data::function::Term,
    prelude::{Boxed, Function, FunctionT},
};

/// The identity morphism.
pub trait Id<A, B> {
    type Identity;
    fn id() -> Self::Identity;
}

impl<F, A, B> Id<A, B> for F
where
    F: FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type Identity = Function<A, A>;

    fn id() -> Self::Identity {
        identity.boxed()
    }
}

/// Utility alias to [`Id::Identity`].
pub type IdentityT<T, A, B> = <T as Id<A, B>>::Identity;

/// Morphism composition.
pub trait Compose<FB, A, B, C> {
    type Composed;
    fn compose(self, m: FB) -> Self::Composed;
}

impl<FA, FB, A, B, C> Compose<FB, A, B, C> for FA
where
    FA: Term + FunctionT<A, B>,
    FB: Term + FunctionT<B, C>,
    A: Term,
    B: Term,
    C: Term,
{
    type Composed = Function<A, C>;

    fn compose(self, m: FB) -> Self::Composed {
        (|a| m(self(a))).boxed()
    }
}

/// Utility alias to [`Compose::Composed`].
pub type ComposedT<T, F, A, B, C> = <T as Compose<F, A, B, C>>::Composed;
