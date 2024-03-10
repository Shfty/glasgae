use crate::prelude::Term;

pub trait Pair<L, R>: Term {
    fn pair(l: L, r: R) -> Self;
    fn fst(self) -> L;
    fn snd(self) -> R;
}

impl<L, R> Pair<L, R> for (L, R)
where
    L: Term,
    R: Term,
{
    fn pair(l: L, r: R) -> Self {
        (l, r)
    }

    fn fst(self) -> L {
        self.0
    }

    fn snd(self) -> R {
        self.1
    }
}
