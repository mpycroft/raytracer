mod cone;
mod cube;
mod cylinder;
mod group;
mod intersectable;
mod plane;
mod sphere;
#[cfg(test)]
pub mod test;
mod triangle;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use paste::paste;

pub use self::intersectable::Intersectable;
#[cfg(test)]
use self::test::Test;
use self::{
    cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane,
    sphere::Sphere, triangle::Triangle,
};
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{Point, Ray, Vector},
    Object,
};

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Shapes {
    Cone(Cone),
    Cube(Cube),
    Cylinder(Cylinder),
    Group(Group),
    Plane(Plane),
    Sphere(Sphere),
    #[cfg(test)]
    Test(Test),
    Triangle(Triangle),
}

macro_rules! add_new_fn {
    ($shape:ident($($args:ident : $ty:ty $(,)?)*)) => {
        paste! {
            #[must_use]
            pub fn [<new_ $shape:lower>]($($args:$ty,)*) -> Shapes {
                Self::$shape($shape::new($($args,)*))
            }
        }
    };
}

impl Shapes {
    add_new_fn!(Cone(minimum: f64, maximum: f64, closed: bool));
    add_new_fn!(Cube());
    add_new_fn!(Cylinder(minimum: f64, maximum: f64, closed: bool));
    add_new_fn!(Group(objects: Vec<Object>));
    add_new_fn!(Plane());
    add_new_fn!(Sphere());
    #[cfg(test)]
    add_new_fn!(Test());
    add_new_fn!(Triangle(point1: Point, point2: Point, point3: Point));

    #[must_use]
    pub fn new_smooth_triangle(
        point1: Point,
        point2: Point,
        point3: Point,
        normal1: Vector,
        normal2: Vector,
        normal3: Vector,
    ) -> Self {
        Self::Triangle(Triangle::new_with_normals(
            point1, point2, point3, normal1, normal2, normal3,
        ))
    }
}

impl ApproxEq for &Shapes {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Shapes::Cone(lhs), Shapes::Cone(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Shapes::Cube(_), Shapes::Cube(_)) => true,
            (Shapes::Cylinder(lhs), Shapes::Cylinder(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Shapes::Group(lhs), Shapes::Group(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Shapes::Sphere(_), Shapes::Sphere(_)) => true,
            (Shapes::Plane(_), Shapes::Plane(_)) => true,
            #[cfg(test)]
            (Shapes::Test(_), Shapes::Test(_)) => true,
            (Shapes::Triangle(lhs), Shapes::Triangle(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
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
        let s1 = Shapes::new_test();
        let s2 = Shapes::new_test();
        let s3 = Shapes::new_sphere();
        let s4 = Shapes::new_cylinder(1.0, 2.0, false);
        let s5 = Shapes::new_cylinder(1.0, 2.0, true);
        let s6 = Shapes::new_cone(-1.5, 1.5, true);
        let s7 = Shapes::new_cone(-1.5, 1.500_1, true);
        let s8 = Shapes::new_group(vec![Object::sphere_builder().build()]);
        let s9 = Shapes::new_group(vec![Object::plane_builder().build()]);
        let s10 = Shapes::new_triangle(
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );
        let s11 = Shapes::new_triangle(
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
        );
        let s12 = Shapes::new_smooth_triangle(
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, -1.0, 0.0),
            Vector::x_axis(),
            Vector::y_axis(),
            Vector::z_axis(),
        );
        let s13 = Shapes::new_smooth_triangle(
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, -1.0, 0.0),
            Vector::x_axis(),
            Vector::y_axis(),
            -Vector::z_axis(),
        );

        assert_approx_eq!(s1, &s2);

        assert_approx_ne!(s1, &s3);

        assert_approx_ne!(s4, &s5);
        assert_approx_ne!(s6, &s7);
        assert_approx_ne!(s8, &s9);
        assert_approx_ne!(s10, &s11);
        assert_approx_ne!(s12, &s13);
    }
}
