//! The [`Either`] type represents values with two possibilities:
//! a value of type [`Either<A, B>`] is either [`Left(A)`](Either::Left) or
//! [`Right(B)`](Either::Right).

use crate::prelude::*;

use super::{fold_map_default, function::bifunction::BifunT, FoldMap};

pub mod result;

/// The `Either` type is sometimes used to represent a value which is either correct or an error;
/// by convention, the `Left` constructor is used to hold an error value
/// and the `Right` constructor is used to hold a correct value
/// (mnemonic: "right" also means "correct").
///
/// In practical terms, this is equivalent to Rust's native [`Result`] type
/// (implementations for which are provided in the [`result`] module,)
/// but with a more general semantic.
///
/// ## Examples
///
/// The type `Either<String, u32>` is the type of values which can be either a `String` or an `u32`.
/// The `Left` constructor can be used only on `String`s,
/// and the `Right` constructor can be used only on `u32`s:
///
/// ```
/// # use glasgae::prelude::{Either, Either::*};
/// let s: Either<&str, u32> = Left("foo");
/// assert_eq!(s, Left("foo"))
/// ```
///
/// ```
/// # use glasgae::prelude::{Either, Either::*};
/// let n: Either<&str, u32> = Right(3);
/// assert_eq!(n, Right(3));
/// ```
///
/// The fmap from our Functor instance will ignore `Left` values,
/// but will apply the supplied function to values contained in a `Right`:
///
/// ```
/// # use glasgae::prelude::{Functor, Either, Either::*};
/// let s: Either<&str, u32> = Left("foo");
/// let n: Either<&str, u32> = Right(3);
/// assert_eq!(s.fmap(|t| t * 2), Left("foo"));
/// assert_eq!(n.fmap(|t| t * 2), Right(6));
/// ```
///
/// The `Monad` instance for `Either` allows us to chain together multiple actions which may fail,
/// and fail overall if any of the individual steps failed.
///
/// First we'll write a function that can either parse a `u32` from an `char`, or fail.
///
/// ```
/// # use glasgae::prelude::{FunctionT, Either, Either::*};
/// fn parse_either(c: char) -> Either<String, u32> {
///       if c.is_digit(10) {
///           Right(c.to_digit(10).unwrap())
///       }
///       else {
///           Left("parse error".to_string())
///       }
/// };
/// ```
///
/// The following should work, since both `1` and `2` can be parsed as u32s.
///
/// ```
/// # use glasgae::prelude::{Either, Either::*, ChainM, ReturnM};
/// # fn parse_either(c: char) -> Either<String, u32> {
/// #       if c.is_digit(10) {
/// #           Right(c.to_digit(10).unwrap())
/// #       }
/// #       else {
/// #           Left("parse error".to_string())
/// #       }
/// # };
///
/// let parse_multiple: Either<String, u32> = parse_either('1')
///         .chain_m(|x| {
///             parse_either('2').chain_m(move |y| {
///                 ReturnM::return_m(x + y)
///             })
///         });
///
/// assert_eq!(parse_multiple, Right(3));
/// ```
/// But the following should fail overall, since the first operation where we attempt to parse 'm' as an u32 will fail:
///
/// ```
/// # use glasgae::prelude::{Either, Either::*, ChainM, ReturnM};
/// # fn parse_either(c: char) -> Either<String, u32> {
/// #       if c.is_digit(10) {
/// #           Right(c.to_digit(10).unwrap())
/// #       }
/// #       else {
/// #           Left("parse error".to_string())
/// #       }
/// # };
///
/// let parse_multiple: Either<String, u32> = parse_either('m')
///     .chain_m(|x| {
///         parse_either('2')
///             .chain_m(move |y| {
///                 ReturnM::return_m(x + y)
///             })
///         });
///
/// assert_eq!(parse_multiple, Left("parse error".to_string()));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<A, B = A> {
    Left(A),
    Right(B),
}

use Either::*;

impl<T, E> From<Result<T, E>> for Either<E, T> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(t) => Right(t),
            Err(e) => Left(e),
        }
    }
}

impl<A, B> Either<A, B> {
    pub fn is_left(self) -> bool {
        matches!(self, Left(_))
    }

    pub fn is_right(self) -> bool {
        matches!(self, Right(_))
    }

    pub fn from_left(left: A) -> Self {
        Left(left)
    }

    pub fn from_right(right: B) -> Self {
        Right(right)
    }
}

impl<E, A> Pointed for Either<E, A> {
    type Pointed = A;
}

impl<E, A, B> WithPointed<B> for Either<E, A> {
    type WithPointed = Either<E, B>;
}

impl<E, A, B> Functor<B> for Either<E, A>
where
    B: Clone,
{
    fn fmap(self, f: impl FunctionT<A, B> + Clone) -> Either<E, B> {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }
}

impl<E, A> PureA for Either<E, A> {
    fn pure_a(t: Self::Pointed) -> Self {
        Right(t)
    }
}

impl<E, F, A, B> AppA<Either<E, A>, Either<E, B>> for Either<E, F>
where
    F: FunctionT<A, B> + Clone,
    B: Clone,
{
    fn app_a(self, r: Either<E, A>) -> Either<E, B> {
        match self {
            Left(e) => Left(e),
            Right(f) => r.fmap(f),
        }
    }
}

impl<E, A> ReturnM for Either<E, A> {}

impl<E, A, B> ChainM<Either<E, B>> for Either<E, A> {
    fn chain_m(self, f: impl FunctionT<Self::Pointed, Either<E, B>> + Clone) -> Either<E, B> {
        match self {
            Left(l) => Left(l),
            Right(r) => f(r),
        }
    }
}

impl<E, A, B> FoldMap<A, B> for Either<E, A>
where
    B: Monoid,
{
    fn fold_map(self, f: impl FunctionT<A, B> + Clone) -> B {
        fold_map_default(self, f)
    }
}

impl<E, A, B> Foldr<A, B> for Either<E, A> {
    fn foldr(self, f: impl BifunT<A, B, B> + Clone, z: B) -> B {
        match self {
            Left(_) => z,
            Right(y) => f(y, z),
        }
    }
}

impl<E, A, A_, A1> TraverseT<A1, A_, A1::WithPointed> for Either<E, A>
where
    A1: Functor<Either<E, A_>, Pointed = A_>,
    A1::WithPointed: PureA<Pointed = Either<E, A_>>,
    E: 'static + Clone,
    A_: 'static + Clone,
{
    fn traverse_t(self, f: impl FunctionT<Self::Pointed, A1> + Clone) -> A1::WithPointed {
        match self {
            Left(x) => PureA::pure_a(Left(x)),
            Right(y) => f(y).fmap(Right.boxed()),
        }
    }
}

impl<E, A1, A_> SequenceA<A_, A1::WithPointed> for Either<E, A1>
where
    A1: Functor<Either<E, A_>, Pointed = A_>,
    A1::WithPointed: PureA<Pointed = Either<E, A_>>,
    E: 'static + Clone,
    A_: 'static + Clone,
{
    fn sequence_a(self) -> A1::WithPointed {
        match self {
            Left(x) => PureA::pure_a(Left(x)),
            Right(y) => y.fmap(Right.boxed()),
        }
    }
}

impl<E, A> Semigroup for Either<E, A> {
    fn assoc_s(self, b: Self) -> Self {
        match self {
            Left(_) => b,
            Right(_) => self,
        }
    }
}

pub fn either<A, B, C>(
    fl: impl FunctionT<A, C>,
    fr: impl FunctionT<B, C>,
    either: Either<A, B>,
) -> C {
    match either {
        Left(l) => fl(l),
        Right(r) => fr(r),
    }
}

pub fn lefts<A, B>(es: Vec<Either<A, B>>) -> Vec<A> {
    es.into_iter()
        .flat_map(|t| match t {
            Left(l) => Some(l),
            Right(_) => None,
        })
        .collect()
}

pub fn rights<A, B>(es: Vec<Either<A, B>>) -> Vec<B> {
    es.into_iter()
        .flat_map(|t| match t {
            Left(_) => None,
            Right(r) => Some(r),
        })
        .collect()
}

pub fn partition_eithers<A, B>(es: Vec<Either<A, B>>) -> (Vec<A>, Vec<B>) {
    es.into_iter()
        .fold((vec![], vec![]), |(mut ls, mut rs), next| {
            match next {
                Left(l) => ls.push(l),
                Right(r) => rs.push(r),
            }
            (ls, rs)
        })
}
