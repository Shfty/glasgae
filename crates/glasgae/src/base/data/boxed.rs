/// Postfix-callable [`Box`] constructor.
pub trait Boxed: Sized {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T> Boxed for T where T: Sized {}

