//! A functor with application, providing operations to embed pure expressions ([`pure`](PureA::pure_a)),
//! and sequence computations and combine their results ([`app_a`](AppA::app_a) and
//! [`lift_a2`](LiftA2::lift_a2)).
//!
//! A minimal complete definition must include implementations of [`pure`](PureA::pure_a)
//! and of either [`app_a`](AppA::app_a) or [`lift_a2`](LiftA2::lift_a2).
//!
//! If it defines both, then they must behave the same as their default definitions:
//! ```text
//! app_a == identity.lift_a2()
//! ```
//! ```text
//! f.lift_a2(x, y) == f.fmap(x).app_a(y)
//! ```
//!
//! Further, any definition must satisfy the following:
//!
//! **Identity**
//! ```text
//! PureA::pure_a(identity).app_a(v) == v
//! ```
//!
//! **Composition**
//! ```text
//! PureA::pure_a(Compose::compose).app_a(u).app_a(v).app_a(w) == u.app_a(v.app_a(w))
//! ```
//!
//! **Homomorphism**
//! ```text
//! PureA::pure_a(f).app(PureA::pure_a(x)) == PureA::pure_a(f(x))
//! ```
//!
//! **Interchange**
//! ```text
//! u.app_a(PureA::pure_a(y)) == PureA::pure_a(|f| f(y)).app_a(u)
//! ```
//!
//! The other methods have the following default definitions, which may be overridden with equivalent specialized implementations:
//! ```text
//! u *> v = (id <$ u) <*> v
//! ```
//! ```text
//! u <* v = liftA2 const u v
//! ```
//!
//! As a consequence of these laws, the Functor instance for f will satisfy
//! ```text
//! fmap f x = pure f <*> x
//! ```
//!
//! It may be useful to note that supposing
//! ```text
//! forall x y. p (q x y) = f x . g y
//! ```
//!
//! it follows from the above that
//!
//! ```text
//! liftA2 p (liftA2 q u v) = liftA2 f u . liftA2 g v
//! ```
//!
//! If f is also a Monad, it should satisfy
//!
//! ```text
//! pure = return
//! ```
//! ```text
//! m1 <*> m2 = m1 >>= (\x1 -> m2 >>= (\x2 -> return (x1 x2)))
//! ```
//! ```text
//! (*>) = (>>)
//! ```
//! (which implies that pure and <*> satisfy the applicative functor laws).

use crate::{base::data::function::bifunction::BifunT, prelude::*};

/// Lift a value.
pub trait PureA: Pointed {
    fn pure_a(t: Self::Pointed) -> Self;
}

/// Sequential application.
///
/// A few functors support an implementation of [`app_a`](AppA::app_a) that is more efficient than the default one.
///
/// # Example
///
/// Used in combination with [`fmap`](crate::prelude::Functor::fmap), [`app_a`](AppA::app_a) can be used to build a record.
///
/// ```ignore
/// struct Foo;
/// struct Bar;
/// struct Baz;
///
/// struct MyState {
///     arg1: Foo,
///     arg2: Bar,
///     arg3: Baz
/// }
///
/// impl MyState {
///     pub fn new(arg1: Foo, arg2: Bar, arg3: Baz) -> Self {
///         MyState {
///             arg1,
///             arg2,
///             arg3
///         }
///     }
/// }
///
/// fn produce_foo() -> <impl Applicative> {
///     ...
/// }
///
/// fn produce_bar() -> <impl Applicative> {
///     ...
/// }
///
/// fn produce_baz() -> <impl Applicative> {
///     ...
/// }
///
/// let mk_state = produce_foo()
///     .fmap(MyState::new.curried())
///     .app(produce_bar())
///     .app(produce_baz());
/// ```
pub trait AppA<A, B>: WithPointed<A, WithPointed = Self::WithA> + WithPointed<B, WithPointed = Self::WithB> {
    type WithA: WithPointed<Self::Pointed, Pointed = A, WithPointed = Self>;
    type WithB: WithPointed<Self::Pointed, Pointed = B, WithPointed = Self>;

    fn app_a(self, a: Self::WithA) -> Self::WithB;
}

// Derive Applicative over the inner type
#[macro_export]
macro_rules! derive_applicative {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> PureA for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn pure_a(t: Self::Pointed) -> Self {
                $ty(t)
            }
        }

        impl<$($_arg,)* $arg $(,$arg_)*, A, B> AppA<A, B> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term + $crate::prelude::FunctionT<A, B> + $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            A: $crate::prelude::Term,
            B: $crate::prelude::Term,
        {
            type WithA = $ty<A>;
            type WithB = $ty<B>;

            fn app_a(self, a: $ty<A>) -> $ty<B> {
                a.fmap(self.0)
            }
        }

    };
}

// Derive Applicative by recursing into the inner type
#[macro_export]
macro_rules! derive_applicative_via {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> PureA for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn pure_a(t: Self::Pointed) -> Self {
                $ty($crate::prelude::PureA::pure_a(t))
            }
        }

        impl<$($_arg,)* $arg $(,$arg_)*, A, B> AppA<A, B> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term + $crate::prelude::FunctionT<A, B> + $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            A: $crate::prelude::Term,
            B: $crate::prelude::Term,
        {
            type WithA = $ty<A>;
            type WithB = $ty<B>;

            fn app_a(self, a: $ty<A>) -> $ty<B> {
                $ty($crate::prelude::AppA::app_a(self.0, a.0))
            }
        }
    };
}

pub trait Applicative<A, B>: PureA + AppA<A, B> {}

impl<T, A, B> Applicative<A, B> for T where T: PureA + AppA<A, B> {}

#[macro_export]
macro_rules! derive_applicative_iterable {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::PureA for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn pure_a(t: Self::Pointed) -> Self {
                FromIterator::from_iter([t])
            }
        }

        impl<$($_arg,)* $arg $(,$arg_)*, A, B> $crate::prelude::AppA<A, B> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term + $crate::prelude::FunctionT<A, B> $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            A: $crate::prelude::Term $(+ $trait)*,
            B: $crate::prelude::Term $(+ $trait)*,
        {
            type WithA = $ty<$($_arg,)* A $(,$arg_)*>;
            type WithB = $ty<$($_arg,)* B $(,$arg_)*>;

            fn app_a(self, xs: $ty<A>) -> $ty<B> {
                let fs = self;
                $crate::prelude::ChainM::chain_m(
                    fs,
                    |f| $crate::prelude::ChainM::chain_m(
                        xs,
                        |x| $crate::prelude::ReturnM::return_m(f(x))
                    )
                )
            }
        }
    };
}

/// Lift a binary function to actions.
///
/// Some functors support an implementation of [`lift_a2`](LiftA2::lift_a2)
/// that is more efficient than the default one.
///
/// In particular, if fmap is an expensive operation,
/// it is likely better to use [`lift_a2`](LiftA2::lift_a2)
/// than to [`fmap`](Functor::fmap) over the structure and then use [`app_a`](AppA::app_a).
///
/// # Example
/// ```
/// # use glasgae::{prelude::LiftA2, base::data::tuple::pair::Pair};
/// assert_eq!(
///     Pair::pair.lift_a2()(Some(3), Some(5)),
///     Some((3, 5))
/// )
/// ```
pub trait LiftA2<MA, B, C>: Term + BifunT<MA::Pointed, B, C>
where
    MA: Pointed + WithPointed<B> + WithPointed<C>,
{
    fn lift_a2(self) -> impl BifunT<MA, WithPointedT<MA, B>, WithPointedT<MA, C>>;
}

impl<F, MF, MA, A, MB, B, MC, C> LiftA2<MA, B, C> for F
where
    F: Term + BifunT<A, B, C>,
    MA: Pointed<Pointed = A>
        + Functor<Function<B, C>, Mapped = MF>
        + WithPointed<B, WithPointed = MB>
        + WithPointed<C, WithPointed = MC>,
    MF: Applicative<B, C, WithA = MB, WithB = MC>,
    MB: Pointed<Pointed = B>,
    MC: Pointed<Pointed = C>,
    A: Term,
    B: Term,
    C: Term,
{
    fn lift_a2(self) -> impl BifunT<MA, MB, MC> {
        |ma, mb| ma.fmap(|t| (|v| self(t, v)).boxed()).app_a(mb)
    }
}
