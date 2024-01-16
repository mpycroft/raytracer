mod cube;
mod cylinder;
mod plane;
mod sphere;
#[cfg(test)]
pub mod test;

use float_cmp::{ApproxEq, F64Margin};

pub use self::cylinder::Cylinder;
use crate::{
    intersection::TList,
    math::{Point, Ray, Vector},
};

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Cube,
    Cylinder(Cylinder),
    Plane,
    Sphere,
    #[cfg(test)]
    Test,
}

impl Shape {
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<TList> {
        match self {
            Self::Cube => cube::intersect(ray),
            Self::Cylinder(cylinder) => cylinder.intersect(ray),
            Self::Plane => plane::intersect(ray),
            Self::Sphere => sphere::intersect(ray),
            #[cfg(test)]
            Self::Test => test::intersect(ray),
        }
    }

    #[must_use]
    pub fn normal_at(&self, point: &Point) -> Vector {
        match self {
            Self::Cube => cube::normal_at(point),
            Self::Cylinder(cylinder) => cylinder.normal_at(point),
            Self::Plane => plane::normal_at(point),
            Self::Sphere => sphere::normal_at(point),
            #[cfg(test)]
            Self::Test => test::normal_at(point),
        }
    }
}

impl ApproxEq for Shape {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Cube, Self::Cube)
            | (Self::Sphere, Self::Sphere)
            | (Self::Plane, Self::Plane) => true,
            (Self::Cylinder(lhs), Self::Cylinder(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            #[cfg(test)]
            (Self::Test, Self::Test) => true,
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
        let s1 = Shape::Test;
        let s2 = Shape::Test;
        let s3 = Shape::Sphere;

        assert_approx_eq!(s1, s2);

        assert_approx_ne!(s1, s3);
    }
}
