mod append;
mod filter;

pub use append::*;
pub use filter::*;

pub mod array;
pub mod linked_list;
pub mod string;
pub mod vec;
pub mod vec_deque;

pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl<T, U> ToVec<U> for T
where
    T: IntoIterator<Item = U>,
{
    fn to_vec(self) -> Vec<U> {
        self.into_iter().collect()
    }
}
