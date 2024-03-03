//! Generalized numeric operations.

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($ty:ty, $zero:expr) => {
        impl Zero for $ty {
            fn zero() -> Self {
                $zero
            }
        }
    };
}

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(usize, 0);

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
impl_zero!(isize, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($ty:ty, $one:expr) => {
        impl One for $ty {
            fn one() -> Self {
                $one
            }
        }
    };
}

impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);
impl_one!(usize, 1);

impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);
impl_one!(isize, 1);

impl_one!(f32, 1.0);
impl_one!(f64, 1.0);

pub trait Even {
    fn even(self) -> bool;
}

impl<T> Even for T
where
    T: Zero + One + std::ops::Add<Output = T> + std::ops::Rem<Output = T> + PartialEq,
{
    fn even(self) -> bool {
        let two = T::one() + T::one();
        self % two == T::zero()
    }
}

pub trait Odd {
    fn odd(self) -> bool;
}

impl<T> Odd for T
where
    T: Zero + One + std::ops::Add<Output = T> + std::ops::Rem<Output = T> + PartialEq,
{
    fn odd(self) -> bool {
        let two = T::one() + T::one();
        self % two == T::one()
    }
}
