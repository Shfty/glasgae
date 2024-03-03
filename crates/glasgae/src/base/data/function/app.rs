// Infix function applicator
pub trait App: Sized {
    fn app<O>(self, f: impl FnOnce(Self) -> O) -> O {
        f(self)
    }
}

impl<T> App for T {}

// Binary infix function applicator
pub trait App2<A, B>: Sized {
    fn app2<O>(self, f: impl FnOnce(A, B) -> O) -> O;
}

impl<A, B> App2<A, B> for (A, B) {
    fn app2<O>(self, f: impl FnOnce(A, B) -> O) -> O {
        f(self.0, self.1)
    }
}

// Ternary infix function applicator
pub trait App3<A, B, C>: Sized {
    fn app2<O>(self, f: impl FnOnce(A, B, C) -> O) -> O;
}

impl<A, B, C> App3<A, B, C> for (A, B, C) {
    fn app2<O>(self, f: impl FnOnce(A, B, C) -> O) -> O {
        f(self.0, self.1, self.2)
    }
}
