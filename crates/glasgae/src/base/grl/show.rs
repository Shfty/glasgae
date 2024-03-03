/// By-value equivalent of [`ToString`].
pub trait Show {
    fn show(self) -> String;
}

impl<T> Show for T
where
    T: ToString,
{
    fn show(self) -> String {
        self.to_string()
    }
}

