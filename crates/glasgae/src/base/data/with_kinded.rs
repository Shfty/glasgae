use super::kinded::Kinded;

/// Modification of a single free type parameter
pub trait WithKinded<T>: Kinded {
    type WithKinded: Kinded<Kinded = T>;
}

