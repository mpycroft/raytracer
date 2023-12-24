mod plane;
mod sphere;
#[cfg(test)]
mod test;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};

use self::plane::Plane;
use self::sphere::Sphere;
#[cfg(test)]
pub(super) use self::test::Test;
use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{Point, Ray, Vector},
};

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch]
pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
    #[cfg(test)]
    Test(Test),
}

impl Shape {
    #[must_use]
    pub fn new_plane() -> Self {
        Self::Plane(Plane)
    }

    #[must_use]
    pub fn new_sphere() -> Self {
        Self::Sphere(Sphere)
    }

    #[cfg(test)]
    #[must_use]
    pub fn new_test() -> Self {
        Self::Test(Test)
    }
}

impl ApproxEq for Shape {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, _margin: M) -> bool {
        match (self, other) {
            (Self::Sphere(_), Self::Sphere(_))
            | (Self::Plane(_), Self::Plane(_)) => true,
            #[cfg(test)]
            (Self::Test(_), Self::Test(_)) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn comparing_shapes() {
        let s1 = Shape::new_test();
        let s2 = Shape::new_test();
        let s3 = Shape::new_sphere();

        assert_approx_eq!(s1, s2);

        assert_approx_ne!(s1, s3);
    }
}
