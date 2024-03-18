use std::collections::VecDeque;

use crate::impl_list;

impl_list!(VecDeque<T>, push_back);

pub fn push_back<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_back(t);
    deque
}

pub fn push_front<T>(t: T, mut deque: VecDeque<T>) -> VecDeque<T> {
    deque.push_front(t);
    deque
}
