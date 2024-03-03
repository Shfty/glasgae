/// By-value equivalent of [`From<String>`].
pub trait Read {
    fn read(t: String) -> Self;
}

impl<T> Read for T
where
    T: From<String>,
{
    fn read(t: String) -> Self {
        t.into()
    }
}

