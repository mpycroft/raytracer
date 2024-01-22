mod cone;
mod cube;
mod cylinder;
mod plane;
mod sphere;
#[cfg(test)]
pub mod test;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use paste::paste;

#[cfg(test)]
pub use self::test::Test;
pub use self::{
    cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere,
};
use crate::{
    intersection::{Intersectable, TList},
    math::{Point, Ray, Vector},
};

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch]
pub enum Shape {
    Cone(Cone),
    Cube(Cube),
    Cylinder(Cylinder),
    Plane(Plane),
    Sphere(Sphere),
    #[cfg(test)]
    Test(Test),
}

macro_rules! add_new_fn {
    ($shape:ident($($args:ident : $ty:ty $(,)?)*)) => {
        paste! {
            #[must_use]
            pub fn [<new_ $shape:lower>]($($args:$ty,)*) -> Shape {
                Self::$shape($shape::new($($args,)*))
            }
        }
    };
}

impl Shape {
    add_new_fn!(Cone(minimum: f64, maximum: f64, closed: bool));
    add_new_fn!(Cube());
    add_new_fn!(Cylinder(minimum: f64, maximum: f64, closed: bool));
    add_new_fn!(Plane());
    add_new_fn!(Sphere());
    #[cfg(test)]
    add_new_fn!(Test());
}

impl ApproxEq for Shape {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Cone(lhs), Self::Cone(rhs)) => lhs.approx_eq(rhs, margin),
            (Self::Cube(_), Self::Cube(_)) => true,
            (Self::Sphere(_), Self::Sphere(_)) => true,
            (Self::Plane(_), Self::Plane(_)) => true,
            (Self::Cylinder(lhs), Self::Cylinder(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
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
        let s4 = Shape::new_cylinder(1.0, 2.0, false);
        let s5 = Shape::new_cylinder(1.0, 2.0, true);
        let s6 = Shape::new_cone(-1.5, 1.5, true);
        let s7 = Shape::new_cone(-1.5, 1.500_1, true);

        assert_approx_eq!(s1, s2);

        assert_approx_ne!(s1, s3);

        assert_approx_ne!(s4, s5);
        assert_approx_ne!(s6, s7);
    }
}
