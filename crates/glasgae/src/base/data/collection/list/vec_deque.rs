use std::collections::VecDeque;

use crate::derive_iterable_collection;

derive_iterable_collection!(VecDeque<T>, push_back);

pub fn push_back<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_back(t);
    deque
}

pub fn push_front<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_front(t);
    deque
}
