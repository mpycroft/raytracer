use derive_more::Constructor;
use float_cmp::{ApproxEq, F64Margin};

use super::sphere::Sphere;
use crate::math::Ray;

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
}

/// An Intersection stores both the t value of the intersection in addition to a
/// reference to the object that was intersected.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub t: f64,
}

impl<'a> ApproxEq for Intersection<'a> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        self.t.approx_eq(other.t, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_an_intersection() {
        let i = Intersection::new(&Sphere, 1.5);

        assert_eq!(i.object, &Sphere);
        assert_approx_eq!(i.t, 1.5);
    }

    #[test]
    fn comparing_intersections() {
        let i1 = Intersection::new(&Sphere, 3.2);
        let i2 = Intersection::new(&Sphere, 3.2);
        let i3 = Intersection::new(&Sphere, 3.200_01);

        assert_approx_eq!(i1, i2);

        assert_approx_ne!(i1, i3);
    }
}
