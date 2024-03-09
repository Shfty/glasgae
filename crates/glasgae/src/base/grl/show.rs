use std::{fmt::Arguments, path::Path};

/// Functional analog to Rust's formatting ecosystem.
pub trait Show {
    fn show(self) -> String;
}

/// Derive a [`Show`] implementation from [`std::fmt::Debug`].
#[macro_export]
macro_rules! derive_show_debug {
    ($ty:ty) => {
        impl Show for $ty {
            fn show(self) -> String {
                format!("{self:?}")
            }
        }
    };
}

/// Derive a [`Show`] implementation from [`std::fmt::Debug`] with multiline semantic.
#[macro_export]
macro_rules! derive_show_debug_multiline {
    ($ty:ty) => {
        impl Show for $ty {
            fn show(self) -> String {
                format!("{self:#?}")
            }
        }
    };
}

/// Derive a [`Show`] implementation from [`std::fmt::Display`].
#[macro_export]
macro_rules! derive_show_display {
    ($ty:ty) => {
        impl Show for $ty {
            fn show(self) -> String {
                format!("{self}")
            }
        }
    };
}

// Implementations on Rust primitives
// ----------------------------------------------------------------------------

derive_show_display!(bool);

derive_show_display!(u8);
derive_show_display!(u16);
derive_show_display!(u32);
derive_show_display!(u64);
derive_show_display!(u128);
derive_show_display!(usize);

derive_show_display!(i8);
derive_show_display!(i16);
derive_show_display!(i32);
derive_show_display!(i64);
derive_show_display!(i128);
derive_show_display!(isize);

derive_show_display!(f32);
derive_show_display!(f64);

derive_show_display!(char);
derive_show_display!(&str);
derive_show_display!(String);

derive_show_display!(std::io::ErrorKind);

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

impl<T> Show for Box<T>
where
    T: Show,
{
    fn show(self) -> String {
        (*self).show()
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

// Adapters
// ----------------------------------------------------------------------------
///
/// Newtype adapter mapping [`std::fmt::Display`] to [`Show`]
pub struct Display<T>(pub T);

impl<T> Show for Display<T>
where
    T: std::fmt::Display,
{
    fn show(self) -> String {
        format!("{}", self.0)
    }
}

/// Newtype adapter mapping [`std::fmt::Debug`] to [`Show`]
pub struct Debug<T>(pub T);

impl<T> Show for Debug<T>
where
    T: std::fmt::Debug,
{
    fn show(self) -> String {
        format!("{:?}", self.0)
    }
}

/// Newtype adapter mapping multiline [`std::fmt::Debug`] to [`Show`].
pub struct DebugMultiline<T>(pub T);

impl<T> Show for DebugMultiline<T>
where
    T: std::fmt::Debug,
{
    fn show(self) -> String {
        format!("{:#?}", self.0)
    }
}
