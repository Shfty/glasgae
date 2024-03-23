//! The Monad class defines the basic operations over a monad,
//! a concept from a branch of mathematics known as category theory.
//!
//! From the perspective of a Rust programmer, however,
//! it is best to think of a monad as an abstract datatype of actions.
//!
//! Instances of Monad should satisfy the following:
//!
//! **Left identity**
//! ```text
//! ReturnM::return_m(a).chain_m(k) == k(a)
//! ```
//!
//! **Right identity**
//!
//! ```text
//! m.chain_m(ReturnM::return_m) == m
//! ```
//!
//! **Associativity**
//!
//! ```text
//! m.chain_m(|x| k(x).chain_m(h)) == m.chain_m(k).chain_m(h)
//! ```
//!
//! Furthermore, the Monad and Applicative operations should relate as follows:
//!
//! ```text
//! PureA::pure_a == ReturnM::return_m
//! m1.app_a(m2) == m1.chain_m(|x1| m2.chain_m(|x2| ReturnM::return_m(x1(x2))))
//! ```
//!
//! The above laws imply:
//! ```text
//! xs.fmap(f) == xs.chain_m(f.compose(return))
//! ThenM::then_m == (*>)
//! ```
//!
//! and that `PureA::pure_a` and `AppA::app_a` satisfy the applicative functor laws.
//!
//! The instances of Monad for lists and Maybe defined in the Prelude satisfy these laws.

pub mod io;
pub mod morph;

use crate::{
    base::data::{collection::list::vec::push, function::bifunction::BifunT},
    prelude::*,
};

/// Inject a value into the monadic type.
pub trait ReturnM: PureA {
    fn return_m(t: Self::Pointed) -> Self
    where
        Self: Sized,
    {
        Self::pure_a(t)
    }
}

/// Sequentially compose two actions,
/// passing any value produced by the first as an argument to the second.
///
/// `as.chain_m(bs)` can be understood as the imperative expression
/// ```text
/// let a = as();
/// bs(a);
/// ```
pub trait ChainM<T: Term>: WithPointed<T, WithPointed = Self::Chained> {
    type Chained: ChainM<Self::Pointed, Pointed = T, Chained = Self>;
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Self::Chained>) -> Self::Chained;
}

/// Convenience alias to [`ChainM::Chained`]
pub type ChainedT<T, U> = <T as ChainM<U>>::Chained;

pub trait Monad<U>: ReturnM + ChainM<U>
where
    U: Term,
{
}

impl<T, U> Monad<U> for T
where
    T: ReturnM + ChainM<U>,
    U: Term,
{
}

// Derive Monad over the inner type
#[macro_export]
macro_rules! derive_monad {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::ReturnM for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn return_m(t: Self::Pointed) -> Self {
                $ty(t)
            }
        }

        impl<$($_arg,)* $arg $(,$arg_)*, B> $crate::prelude::ChainM<B> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term + $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            B: $crate::prelude::Term,
        {
            type Chained = $ty<$($_arg,)* B $(,$arg_)*>;

            fn chain_m(self, f: impl $crate::prelude::FunctionT<$arg, $ty<$($_arg,)* B $(,$arg_)*>>) -> $ty<$($_arg,)* B $(,$arg_)*> {
                f(self.0)
            }
        }

    };
}

// Derive Monad by recursing into the inner type
#[macro_export]
macro_rules! derive_monad_via {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::ReturnM for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn return_m(t: Self::Pointed) -> Self {
                $ty(ReturnM::return_m(t))
            }
        }
        impl<$($_arg,)* $arg $(,$arg_)*, B> $crate::prelude::ChainM<B> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            B: Term,
        {
            type Chained = $ty<$($_arg,)* B $(,$arg_)*>;

            fn chain_m(self, f: impl $crate::prelude::FunctionT<$arg, $ty<$($_arg,)* B $(,$arg_)*>>) -> $ty<$($_arg,)* B $(,$arg_)*> {
                let f = f.to_function();
                $ty(self.0.chain_m(|t| f(t).0))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monad_iterable {
    ($ty:ident<$($_arg:ident $(: $_trait:path)*,)* ($arg:ident $(: $trait:path)*) $(, $arg_:ident $(: $trait_:path),*)*>) => {
        impl<$($_arg,)* $arg $(,$arg_)*> $crate::prelude::ReturnM for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
        {
            fn return_m(t: Self::Pointed) -> Self {
                FromIterator::from_iter([t])
            }
        }

        impl<$($_arg,)* $arg $(,$arg_)*, U> $crate::prelude::ChainM<U> for $ty<$($_arg,)* $arg $(,$arg_)*>
        where
            $(
                $_arg: $crate::prelude::Term $(+ $_trait)*,
            )*
            $arg: $crate::prelude::Term $(+ $trait)*,
            $(
                $arg_: $crate::prelude::Term $(+ $trait_)*,
            )*
            U: $crate::prelude::Term $(+ $trait)*,
        {
            type Chained = $ty<$($_arg,)* U $(,$arg_)*>;

            fn chain_m(self, f: impl $crate::prelude::FunctionT<$arg, $ty<$($_arg,)* U $(,$arg_)*>>) -> $ty<$($_arg,)* U $(,$arg_)*> {
                self.into_iter().flat_map(|t| f.to_function()(t)).collect()
            }
        }
    };
}

/// Sequentially compose two actions, discarding any value produced by the first,
/// like sequencing operators (such as the semicolon) in imperative languages.
///
/// `as.then_m(bs)` can be understood as the imperative expression
/// ```text
/// as();
/// bs();
/// ```
pub trait ThenM<T: Term>: Monad<T> {
    fn then_m(self, t: Self::Chained) -> Self::Chained {
        self.chain_m(|_| t)
    }
}

impl<T, U> ThenM<U> for T
where
    T: Monad<U>,
    U: Term,
{
}

/// This generalizes the list-based filter function.
pub trait FilterM<M1: Term, A: Term, M3>: Term {
    fn filter_m(self, f: impl FunctionT<A, M1>) -> M3;
}

impl<A, MA, MF, MB> FilterM<MA, A, MB> for Vec<A>
where
    MA: Functor<Function<Vec<A>, Vec<A>>, Pointed = bool, Mapped = MF>
        + WithPointed<Vec<A>, WithPointed = MB>,
    MF: Applicative<Vec<A>, Vec<A>, WithA = MB, WithB = MB>,
    MB: Pointed<Pointed = Vec<A>> + PureA,
    A: Term,
{
    fn filter_m(self, f: impl FunctionT<A, MA>) -> MB {
        let f = f.to_function();
        self.foldr(
            |next, acc| {
                {
                    let next = next.clone();
                    move |flg, acc| {
                        if flg {
                            push(next, acc)
                        } else {
                            acc
                        }
                    }
                }
                .lift_a2()(f(next), acc)
            },
            PureA::pure_a(vec![]),
        )
    }
}

pub trait FoldM<M1, A, B>: ReturnM {
    /// The foldlM function is analogous to foldl, except that its result is encapsulated in a monad.
    ///
    /// Note that foldlM works from left-to-right over the list arguments.
    /// This could be an issue where (>>) and the `folded function' are not commutative.
    ///
    /// foldM f a1 [x1, x2, ..., xm]
    ///
    /// ==
    ///
    /// do
    ///   a2 <- f a1 x1
    ///   a3 <- f a2 x2
    ///   ...
    ///   f am xm
    fn foldl_m(self, f: impl BifunT<A, B, M1>, a: A) -> M1;

    /// The foldrM function is analogous to foldr, except that its result is encapsulated in a monad.
    fn foldr_m(self, f: impl BifunT<B, A, M1>, a: A) -> M1;
}

impl<MB, A, B> FoldM<MB, A, B> for Vec<B>
where
    MB: Monad<A, Pointed = A, Chained = MB>,
    A: Term,
    B: Term,
{
    fn foldl_m(self, f: impl BifunT<A, B, MB>, a: A) -> MB {
        let mut xs = self;
        let f = f.to_bifun();

        if xs.is_empty() {
            ReturnM::return_m(a)
        } else {
            let x = xs.remove(0);
            f.clone()(a, x).chain_m({
                let xs = xs.clone();
                move |fax| xs.foldl_m(f, fax)
            })
        }
    }

    fn foldr_m(self, f: impl BifunT<B, A, MB>, a: A) -> MB {
        let mut xs = self;
        let f = f.to_bifun();

        if xs.is_empty() {
            ReturnM::return_m(a)
        } else {
            let x = xs.pop().unwrap();
            f.clone()(x, a).chain_m({
                let xs = xs.clone();
                move |fax| xs.foldr_m(f, fax)
            })
        }
    }
}

/// `replicateM n act` performs the action act n times, and then returns the list of results:
///
/// ## Examples
///
/// ```
/// # use glasgae::{base::control::monad::ReplicateM, transformers::{state::State}};
/// assert_eq!(State::new(|s| (s, s + 1)).replicate_m(3).run(1), (vec![1,2,3],4));
/// ```
pub trait ReplicateM<MB, T>: Pointed {
    fn replicate_m(self, count: usize) -> MB;
}

impl<MA, MB, T> ReplicateM<MB, T> for MA
where
    MA: Functor<Function<Vec<T>, Vec<T>>, Pointed = T> + WithPointed<Vec<T>, WithPointed = MB>,
    MA::Mapped: Applicative<Vec<T>, Vec<T>, WithA = MB, WithB = MB>,
    MB: PureA<Pointed = Vec<T>>,
    T: Term,
{
    fn replicate_m(self, count: usize) -> MB {
        let f = self;
        if count == 0 {
            PureA::pure_a(vec![])
        } else {
            push.lift_a2()(f.clone(), f.replicate_m(count - 1))
        }
    }
}

pub trait LiftM<MA, MB, A, B>: Term + FunctionT<A, B>
where
    MA: Monad<B, Pointed = A, Chained = MB>,
    MB: ReturnM<Pointed = B>,
    A: Term,
    B: Term,
{
    fn lift_m(self) -> Function<MA, MB> {
        (|m1: MA| m1.chain_m(|x1| ReturnM::return_m(self(x1)))).boxed()
    }
}

impl<F, MA, MB, A, B> LiftM<MA, MB, A, B> for F
where
    F: Term + FunctionT<A, B>,
    MA: Monad<B, Pointed = A, Chained = MB>,
    MB: ReturnM<Pointed = B>,
    A: Term,
    B: Term,
{
}
