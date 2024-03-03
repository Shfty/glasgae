use crate::transformers::cont::Cont;

use super::{MakeZipper, travel::Travel, Zipper};

pub trait ZipTravel<D>:
    'static
    + MakeZipper<D>
    + Travel<D, Cont<Zipper<Self, D>, (Option<Self>, D)>, Cont<Zipper<Self, D>, Self>>
where
    D: 'static,
{
    fn zip_travel(self) -> Cont<Zipper<Self, D>> {
        self.make_zipper(Travel::travel)
    }
}

impl<T, D> ZipTravel<D> for T
where
    T: 'static
        + MakeZipper<D>
        + Travel<D, Cont<Zipper<Self, D>, (Option<Self>, D)>, Cont<Zipper<Self, D>, Self>>,
    D: 'static,
{
}
