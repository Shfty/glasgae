use crate::{
    base::data::{kinded::Kinded, with_kinded::WithKinded},
    prelude::*,
};

impl<T, const N: usize> Kinded for [T; N]
where
    T: Term,
{
    type Kinded = T;
}

impl<T, U, const N: usize> WithKinded<U> for [T; N]
where
    T: Term,
    U: Term,
{
    type WithKinded = [U; N];
}

impl<T, const N: usize> Pointed for [T; N]
where
    T: Term,
{
    type Pointed = T;
}

impl<T, U, const N: usize> WithPointed<U> for [T; N]
where
    T: Term,
    U: Term,
{
    type WithPointed = [U; N];
}

impl<T, const N: usize, U> Fmap<U> for [T; N]
where
    T: Term,
    U: Term,
{
    fn fmap(self, f: impl FunctionT<T, U>) -> [U; N] {
        self.into_iter()
            .map(|t| f.to_function()(t))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl<T> PureA for [T; 1]
where
    T: Term,
{
    fn pure_a(t: Self::Pointed) -> Self {
        [t]
    }
}

impl<F, A, B, const N: usize> AppA<[A; N], [B; N]> for [F; N]
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    fn app_a(self, a: [A; N]) -> [B; N] {
        self.into_iter()
            .zip(a)
            .map(|(f, a)| f(a))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl<T> ReturnM for [T; 1] where T: Term {}

impl<T, U, const NT: usize, const NU: usize> ChainM<[U; NU]> for [T; NT]
where
    T: Term,
    U: Term,
{
    fn chain_m(self, f: impl FunctionT<T, [U; NU]>) -> [U; NU] {
        self.into_iter()
            .flat_map(|t| f.to_function()(t))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl<T, const N: usize, U> Foldable<U> for [T; N]
where
    T: Term,
    U: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, init: U) -> U {
        let data: Vec<_> = self.into_iter().collect();
        data.foldr(f, init)
    }

    fn foldl(self, f: impl BifunT<U, T, U>, init: U) -> U {
        let data: Vec<_> = self.into_iter().collect();
        data.foldl(f, init)
    }
}

impl<T, const N: usize> Foldable1<T> for [T; N]
where
    T: Term,
{
    fn foldr1(self, f: impl BifunT<T, T, T>) -> T {
        foldr1_default(self, f)
    }

    fn foldl1(self, f: impl BifunT<T, T, T>) -> T {
        foldl1_default(self, f)
    }
}
