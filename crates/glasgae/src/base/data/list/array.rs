use crate::{prelude::*, base::data::function::bifunction::BifunT};

impl<T, const N: usize> Pointed for [T; N] {
    type Pointed = T;
}

impl<T, U, const N: usize> WithPointed<U> for [T; N] {
    type WithPointed = [U; N];
}

impl<T, const N: usize, U> Functor<U> for [T; N]
where
    U: Clone,
{
    fn fmap(self, f: impl FunctionT<T, U> + Clone) -> [U; N] {
        self.into_iter()
            .map(|t| f.clone()(t))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl<T> PureA for [T; 1] {
    fn pure_a(t: Self::Pointed) -> Self {
        [t]
    }
}

impl<F, A, B, const N: usize> AppA<[A; N], [B; N]> for [F; N]
where
    F: FnOnce(A) -> B,
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

impl<T> ReturnM for [T; 1] {}

impl<T, U, const NT: usize, const NU: usize> ChainM<[U; NU]> for [T; NT] {
    fn chain_m(self, f: impl FunctionT<T, [U; NU]> + Clone) -> [U; NU] {
        self.into_iter()
            .flat_map(|t| f.clone_fun()(t))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }
}

impl<T, const N: usize, U> Foldable<T, U> for [T; N] {
    fn foldr(self, f: impl BifunT<T, U, U> + Clone, init: U) -> U {
        self.into_iter()
            .rfold(init, |acc, next| f.clone()(next, acc))
    }
}
