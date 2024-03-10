use crate::{base::data::term::Term, transformers::cont::Cont};

use super::{travel::Travel, MakeZipper, Zipper};

pub trait ZipTravel<D>:
    MakeZipper<D> + Travel<D, Cont<Zipper<Self, D>, (Option<Self>, D)>, Cont<Zipper<Self, D>, Self>>
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
