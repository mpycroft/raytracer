mod plane;
mod sphere;
#[cfg(test)]
mod test;

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

impl Intersectable for Shape {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        match self {
            Self::Plane(plane) => plane.intersect(ray),
            Self::Sphere(sphere) => sphere.intersect(ray),
            #[cfg(test)]
            Self::Test(test) => test.intersect(ray),
        }
    }

    fn normal_at(&self, point: &Point) -> Vector {
        match self {
            Self::Plane(plane) => plane.normal_at(point),
            Self::Sphere(sphere) => sphere.normal_at(point),
            #[cfg(test)]
            Self::Test(test) => test.normal_at(point),
        }
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
