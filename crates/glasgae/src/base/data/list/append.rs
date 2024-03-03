pub trait Append {
    fn append(self, t: Self) -> Self;
}

impl<T> Append for Vec<T> {
    fn append(self, t: Self) -> Self {
        self.into_iter().chain(t).collect()
    }
}

impl Append for String {
    fn append(self, t: Self) -> Self {
        self + &t
    }
}

