use crate::{transformers::cont::Cont, base::data::function::Term};

use super::{MakeZipper, travel::Travel, Zipper};

pub trait ZipTravel<D>:
    'static
    + MakeZipper<D>
    + Travel<D, Cont<Zipper<Self, D>, (Option<Self>, D)>, Cont<Zipper<Self, D>, Self>>
where
    D: Term,
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
    D: Term,
{
}
