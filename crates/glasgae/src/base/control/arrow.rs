//! Basic arrow definitions.
//!
//! Based on Generalising Monads to Arrows by John Hughes,
//! Science of Computer Programming 37, pp67-111, May 2000.
//!
//! plus a couple of definitions (returnA and loop) from
//!
//! A New Notation for Arrows, by Ross Paterson,
//! in ICFP 2001, Firenze, Italy, pp229-240.
//!
//! These papers and more information on arrows can be found at <http://www.haskell.org/arrows/>.
//!
/// Instances should satisfy the following laws:
///
/// ```text
/// arr id = id
/// ```
/// ```text
/// arr (f >>> g) = arr f >>> arr g
/// ```
/// ```text
/// first (arr f) = arr (first f)
/// ```
/// ```text
/// first (f >>> g) = first f >>> first g
/// ```
/// ```text
/// first f >>> arr fst = arr fst >>> f
/// ```
/// ```text
/// first f >>> arr (id *** g) = arr (id *** g) >>> first f
/// ```
/// ```text
/// first (first f) >>> arr assoc = arr assoc >>> first f
/// ```
///
/// where
/// ```text
/// assoc ((a,b),c) = (a,(b,c))
/// ```
///
/// The other combinators have sensible default definitions, which may be overridden for efficiency.
use crate::{
    base::data::term::Term,
    prelude::{Boxed, Function, FunctionT},
};

/// Lift a function to an arrow.
pub trait Arrow<A, B>: Term {
    fn arrow(self) -> Self;
}

impl<F, A, B> Arrow<A, B> for F
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn arrow(self) -> Self {
        self
    }
}

/// Split the input between the two argument arrows and combine their output. Note that this is in general not a functor.
///
/// The default definition may be overridden with a more efficient version if desired.
pub trait Split<F, LA, LB, RA, RB>: Term {
    type Split;
    fn split(self, other: F) -> Self::Split;
}

impl<FA, FB, LA, LB, RA, RB> Split<FB, LA, LB, RA, RB> for FA
where
    FA: Term + FunctionT<LA, LB>,
    FB: Term + FunctionT<RA, RB>,
    LA: Term,
    LB: Term,
    RA: Term,
    RB: Term,
{
    type Split = Function<(LA, RA), (LB, RB)>;

    fn split(self, g: FB) -> Self::Split {
        let f = self;
        (|(x, y)| (f(x), g(y))).boxed()
    }
}

/// Send the input to both argument arrows and combine their output.
///
/// The default definition may be overridden with a more efficient version if desired.
pub trait Fanout<F, LA, LB, RA, RB>: Term {
    type Fanout;
    fn fanout(self, other: F) -> Self::Fanout;
}

impl<FA, FB, A, LB, RB> Fanout<FB, A, LB, A, RB> for FA
where
    FA: Term + FunctionT<A, LB> + Split<FB, A, LB, A, RB>,
    FB: Term + FunctionT<A, RB>,
    FA::Split: FunctionT<(A, A), (LB, RB)>,
    A: Term,
    LB: Term,
    RB: Term,
{
    type Fanout = Function<A, (LB, RB)>;

    fn fanout(self, g: FB) -> Self::Fanout {
        let f = self;
        (|a: A| f.split(g)((a.clone(), a))).boxed()
    }
}
