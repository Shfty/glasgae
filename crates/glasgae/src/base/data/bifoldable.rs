use crate::prelude::{Compose, Function, FunctionT, Monoid, Term};

use super::{
    bipointed::Bipointed,
    function::{bifunction::BifunT, Curried},
    monoid::Endo,
};

pub trait Bifoldable<T>: Bipointed {
    fn bifoldr(
        self,
        fa: impl BifunT<Self::Bipointed, T, T>,
        fb: impl BifunT<Self::Pointed, T, T>,
        z: T,
    ) -> T;

    fn bifoldl(
        self,
        fa: impl BifunT<T, Self::Bipointed, T>,
        fb: impl BifunT<T, Self::Pointed, T>,
        z: T,
    ) -> T;
}

/// Derive foldr from FoldMap
pub fn bifoldr_default<This, T, U>(this: This, f: impl BifunT<T, U, U>, z: U) -> U
where
    This: BifoldMap<Endo<Function<U, U>>, Bipointed = T>,
    Endo<U>: Monoid,
    T: Term,
    U: Term,
{
    this.bifold_map(f.to_bifun().curried().compose_clone(Endo::new))
        .app()(z)
}

/// Derive foldl from FoldMap
pub fn bifoldl_default<This, T, U>(this: This, f: impl BifunT<U, T, U>, z: U) -> U {
    /*
     foldl f z t = appEndo (getDual (foldMap (Dual . Endo . flip f) t)) z
    */

    todo!()
}

pub trait BifoldMap<U>: Bipointed
where
    U: Monoid,
{
    fn bifold_map(self, f: impl FunctionT<Self::Bipointed, U> + Clone) -> U;
}

/// Derive fold_map from Foldr
pub fn bifold_map_default<This, T>(
    this: This,
    fa: impl FunctionT<This::Bipointed, T>,
    fb: impl FunctionT<This::Pointed, T>,
) -> T
where
    This: Bifoldable<T>,
    T: Monoid,
{
    let fa = fa.to_function();
    let fb = fb.to_function();
    this.bifoldr(
        |next, acc| fa(next).assoc_s(acc),
        |next, acc| fb(next).assoc_s(acc),
        Monoid::mempty(),
    )
}
