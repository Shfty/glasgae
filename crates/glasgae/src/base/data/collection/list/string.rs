use crate::prelude::*;

impl Pointed for String {
    type Pointed = char;
}

impl WithPointed<char> for String {
    type WithPointed = String;
}

impl Functor<char> for String {
    type Mapped = String;

    fn fmap(self, f: impl FunctionT<Self::Pointed, char>) -> Self::WithPointed {
        self.chars().map(|t| f.to_function()(t)).collect()
    }
}

impl Foldable<char> for String {
    fn foldr(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<Self::Pointed, char, char>,
        z: char,
    ) -> char {
        self.chars().rfold(z, |acc, next| f.to_bifun()(next, acc))
    }

    fn foldl(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<char, Self::Pointed, char>,
        z: char,
    ) -> char {
        self.chars().fold(z, |acc, next| f.to_bifun()(acc, next))
    }
}

impl Foldable1<char> for String {
    fn foldr1(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<char, char, char>,
    ) -> char {
        self.chars()
            .reduce(|acc, next| f.to_bifun()(next, acc))
            .unwrap_or_default()
    }

    fn foldl1(
        self,
        f: impl crate::base::data::function::bifunction::BifunT<char, char, char>,
    ) -> char {
        self.chars()
            .reduce(|acc, next| f.to_bifun()(acc, next))
            .unwrap_or_default()
    }
}

impl Semigroup for String {
    fn assoc_s(self, a: Self) -> Self {
        self + &a
    }
}

impl Monoid for String {
    fn mempty() -> Self {
        String::default()
    }

    fn mconcat(list: Vec<Self>) -> Self {
        list.concat()
    }
}
