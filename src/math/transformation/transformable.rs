use std::ops::Mul;

use super::{Matrix, Transformation};

/// The `Transformable` trait describes how to apply a `Transformation` to any
/// given object, implementing this allows us to `.apply()` a `Transformation`
/// to an object via this trait. This is really just some syntactic sugar so we
/// always apply Transform's to objects rather than transform objects with a
/// given Transform.
pub trait Transformable {
    #[must_use]
    fn apply(&self, transformation: &Transformation) -> Self;
}

/// Blanket implementation of Transformable for objects that can be multiplied
/// by a matrix e.g. Points and Vectors.
impl<T> Transformable for T
where
    Matrix<4>: Mul<T, Output = T>,
    T: Copy,
{
    fn apply(&self, transformation: &Transformation) -> Self {
        transformation.0 * *self
    }
}
