use crate::prelude::*;

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
    type Mapped = [U; N];

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

impl<F, A, B, const N: usize> AppA<A, B> for [F; N]
where
    F: Term + FunctionT<A, B>,
    A: Term,
    B: Term,
{
    type WithA = [A; N];
    type WithB = [B; N];

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

impl<T, U, const NT: usize> ChainM<U> for [T; NT]
where
    T: Term,
    U: Term,
{
    type Chained = [U; NT];

    fn chain_m(self, f: impl FunctionT<T, [U; NT]>) -> [U; NT] {
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
