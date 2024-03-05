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

use crate::{base::data::{list::vec::push, function::bifunction::BifunT}, prelude::*};

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
pub trait ChainM<T>: Pointed {
    fn chain_m(self, f: impl FunctionT<Self::Pointed, T> + Clone) -> T;
}

/// Sequentially compose two actions, discarding any value produced by the first,
/// like sequencing operators (such as the semicolon) in imperative languages.
///
/// `as.then_m(bs)` can be understood as the imperative expression
/// ```text
/// as();
/// bs();
/// ```
pub trait ThenM<T>: ChainM<T> {
    fn then_m(self, t: T) -> T
    where
        Self: Sized,
        T: 'static + Clone,
    {
        self.chain_m(|_| t)
    }
}

impl<T, U> ThenM<U> for T where T: ChainM<U> {}

/// This generalizes the list-based filter function.
pub trait FilterM<M1, A, M3> {
    fn filter_m(self, f: impl FunctionT<A, M1> + Clone) -> M3;
}

impl<A, M1, M3> FilterM<M1, A, M3> for Vec<A>
where
    M1: Functor<Function<Vec<A>, Vec<A>>, Pointed = bool>,
    M1::WithPointed: Clone + AppA<M3, M3>,
    M3: Pointed<Pointed = Vec<A>> + PureA,
    A: 'static + Clone,
{
    fn filter_m(self, f: impl FunctionT<A, M1> + Clone) -> M3 {
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

/// The foldM function is analogous to foldl, except that its result is encapsulated in a monad.
///
/// Note that foldM works from left-to-right over the list arguments. This could be an issue where (>>) and the `folded function' are not commutative.
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
/// If right-to-left evaluation is required, the input list should be reversed.
///
/// Note: foldM is the same as foldlM
pub trait FoldM<M1, A, B> {
    fn fold_m(self, f: impl BifunT<A, B, M1> + Clone, a: A) -> M1;
}

impl<MB, A, B> FoldM<MB, A, B> for Vec<B>
where
    MB: ReturnM<Pointed = A> + ChainM<MB> + Clone,
    B: 'static + Clone,
{
    fn fold_m(self, f: impl BifunT<A, B, MB> + Clone, a: A) -> MB {
        let mut xs = self;
        if xs.is_empty() {
            ReturnM::return_m(a)
        } else {
            let x = xs.remove(0);
            f.clone()(a, x).chain_m({
                let xs = xs.clone();
                move |fax| xs.fold_m(f, fax)
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
pub trait ReplicateM<MB, T> {
    fn replicate_m(self, count: usize) -> MB;
}

impl<MA, MB, T> ReplicateM<MB, T> for MA
where
    MA: Clone + Functor<Function<Vec<T>, Vec<T>>, Pointed = T>,
    MA::WithPointed: AppA<MB, MB>,
    MB: PureA<Pointed = Vec<T>>,
    T: 'static + Clone,
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
