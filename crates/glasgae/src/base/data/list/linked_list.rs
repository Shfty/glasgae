use std::collections::LinkedList;

use crate::impl_list;

impl_list!(LinkedList<T>, push_back);

pub fn push_back<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_back(t);
    list
}

pub fn push_front<T>(t: T, mut list: LinkedList<T>) -> LinkedList<T> {
    list.push_front(t);
    list
}

