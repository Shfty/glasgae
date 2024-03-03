/// flip f takes its (first) two arguments in the reverse order of f.
///
/// flip f x y = f y x
/// flip . flip = id
/// Examples
/// >>> flip (++) "hello" "world"
/// "worldhello"
/// >>> let (.>) = flip (.) in (+1) .> show $ 5
/// "6"
pub trait Flip<A, B, C>: Sized {
    fn flip(self) -> impl Fn(B, A) -> C
    where
        Self: Fn(A, B) -> C,
    {
        move |a, b| self(b, a)
    }

    fn flip_mut(mut self) -> impl FnMut(B, A) -> C
    where
        Self: FnMut(A, B) -> C,
    {
        move |a, b| self(b, a)
    }

    fn flip_once(self) -> impl FnOnce(B, A) -> C
    where
        Self: FnOnce(A, B) -> C,
    {
        move |a, b| self(b, a)
    }

    fn flip_clone(self) -> impl FnOnce(B, A) -> C + Clone
    where
        Self: FnOnce(A, B) -> C + Clone,
    {
        move |a, b| self(b, a)
    }
}

impl<F, A, B, C> Flip<A, B, C> for F where F: FnOnce(A, B) -> C {}
