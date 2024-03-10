use std::panic::{UnwindSafe, RefUnwindSafe};

/// [`Term`] without its [`Sized`] and [`Clone`] constraints, for object-safety.
pub trait TermBase: 'static + Send + Sync + UnwindSafe + RefUnwindSafe {}
impl<T> TermBase for T where T: 'static + Send + Sync + UnwindSafe + RefUnwindSafe {}

/// A type suitable for use within a functional expression.
pub trait Term: TermBase + Sized + Clone {}
impl<T> Term for T where T: TermBase + Clone {}
