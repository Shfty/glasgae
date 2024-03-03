use crate::prelude::Boxed;

/// Binary function
pub trait BifunT<A, B, C>: FnOnce(A, B) -> C + 'static {
    fn clone_bifun(&self) -> Bifun<A, B, C>;
}

impl<F, A, B, C> BifunT<A, B, C> for F
where
    F: FnOnce(A, B) -> C + Clone + 'static,
{
    fn clone_bifun(&self) -> Bifun<A, B, C> {
        self.clone().boxed()
    }
}

pub type Bifun<A, B, C> = Box<dyn BifunT<A, B, C>>;

impl<A, B, C> Clone for Bifun<A, B, C>
where
    A: 'static,
    B: 'static,
    C: 'static,
{
    fn clone(&self) -> Self {
        (**self).clone_bifun()
    }
}
