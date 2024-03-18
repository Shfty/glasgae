use std::collections::LinkedList;

use crate::derive_iterable_collection;

derive_iterable_collection!(LinkedList<T>, push_back);

pub fn push_back<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_back(t);
    list
}

pub fn push_front<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_front(t);
    list
}

