use crate::prelude::{Compose, Function, FunctionT, Monoid, Term};

use super::{
    bipointed::Bipointed,
    function::{bifunction::BifunT, Curried},
    monoid::Endo,
};

pub trait Bifoldable<U>: Bipointed {
    fn bifoldr(self, f: impl BifunT<Self::Bipointed, U, U>, z: U) -> U;
    fn bifoldl(self, f: impl BifunT<U, Self::Bipointed, U>, z: U) -> U;
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
pub fn bifold_map_default<This, T, U>(this: This, f: impl FunctionT<T, U>) -> U
where
    This: Bifoldable<U, Bipointed = T>,
    T: Term,
    U: Monoid,
{
    let f = f.to_function();
    this.bifoldr(|next, acc| f(next).assoc_s(acc), Monoid::mempty())
}
