use crate::math::Ray;

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
}
