use std::{path::Path, fmt::Arguments};

/// By-value equivalent of [`ToString`].
pub trait Show {
    fn show(self) -> String;
}

macro_rules! impl_show {
    ($ty:ty) => {
        impl Show for $ty {
            fn show(self) -> String {
                format!("{self}")
            }
        }
    };
}

impl_show!(bool);

impl_show!(u8);
impl_show!(u16);
impl_show!(u32);
impl_show!(u64);
impl_show!(u128);
impl_show!(usize);

impl_show!(i8);
impl_show!(i16);
impl_show!(i32);
impl_show!(i64);
impl_show!(i128);
impl_show!(isize);

impl_show!(f32);
impl_show!(f64);

impl_show!(char);
impl_show!(&str);
impl_show!(String);

impl<T> Show for Vec<T>
where
    T: Show,
{
    fn show(self) -> String {
        format!(
            "[{}]",
            self.into_iter()
                .map(Show::show)
                .reduce(|acc, next| { format!("{acc}, {next}") })
                .unwrap_or_default()
        )
    }
}

impl<T, U> Show for (T, U)
where
    T: Show,
    U: Show,
{
    fn show(self) -> String {
        format!("({}, {})", self.0.show(), self.1.show())
    }
}

impl Show for &Path {
    fn show(self) -> String {
        format!("{}", self.display())
    }
}

impl<'a> Show for Arguments<'a> {
    fn show(self) -> String {
        self.to_string()
    }
}
