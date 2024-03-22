pub use vector_map::set::*;

use crate::{
    derive_applicative_iterable, derive_foldable_iterable, derive_functor_iterable,
    derive_monad_iterable, derive_monoid_iterable, derive_semigroup_iterable,
    derive_traversable_iterable, derive_pointed, derive_with_pointed,
};

derive_pointed!(VecSet<(X: Eq)>);
derive_with_pointed!(VecSet<(X: Eq)>);
derive_functor_iterable!(VecSet<(X: Eq)>);
derive_applicative_iterable!(VecSet<(X: Eq)>);
derive_monad_iterable!(VecSet<(X: Eq)>);
derive_semigroup_iterable!(VecSet<(X: Eq)>);
derive_monoid_iterable!(VecSet<(X: Eq)>);
derive_foldable_iterable!(VecSet<(X: Eq)>);
derive_traversable_iterable!(VecSet<(X: Eq)>, insert);

pub fn insert<T>(t: T, mut m: VecSet<T>) -> VecSet<T>
where
    T: PartialEq,
{
    m.insert(t);
    m
}
