use crate::{base::data::function::bifunction::BifunT, prelude::*};

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

impl<T, const N: usize, U> Functor<U> for [T; N]
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

impl<T, const N: usize, U> Foldr<T, U> for [T; N]
where
    T: Term,
    U: Term,
{
    fn foldr(self, f: impl BifunT<T, U, U>, init: U) -> U {
        let data: Vec<_> = self.into_iter().collect();
        data.foldr(f, init)
    }
}
