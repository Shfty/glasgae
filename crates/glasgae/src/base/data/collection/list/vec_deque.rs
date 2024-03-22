use std::collections::VecDeque;

use crate::{
    derive_applicative_iterable, derive_foldable_iterable, derive_functor_iterable,
    derive_monad_iterable, derive_monoid_iterable, derive_pointed,
    derive_semigroup_iterable, derive_traversable_iterable, derive_with_pointed,
};

derive_pointed!(VecDeque<(X)>);
derive_with_pointed!(VecDeque<(X)>);
derive_functor_iterable!(VecDeque<(X)>);
derive_applicative_iterable!(VecDeque<(X)>);
derive_monad_iterable!(VecDeque<(X)>);
derive_semigroup_iterable!(VecDeque<(X)>);
derive_monoid_iterable!(VecDeque<(X)>);
derive_foldable_iterable!(VecDeque<(X)>);
derive_traversable_iterable!(VecDeque<(X)>, push_back);

pub fn push_back<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_back(t);
    deque
}

pub fn push_front<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_front(t);
    deque
}
