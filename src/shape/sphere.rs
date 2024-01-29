use derive_new::new;

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{Point, Ray, Vector},
    Object,
};

/// A `Sphere` is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, new)]
pub struct Sphere;

impl Intersectable for Sphere {
    #[must_use]
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Option<List<'a>> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();
        let a = 2.0 * a;

        let t1 = (-b - discriminant) / a;
        let t2 = (-b + discriminant) / a;

        Some(List::from(vec![
            Intersection::new(object, t1),
            Intersection::new(object, t2),
        ]))
    }

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector {
        *point - Point::origin()
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, shape::Shape};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let o = Object::sphere_builder().build();

        let Shape::Sphere(s) = &o.shape else { unreachable!() };

        let l = s
            .intersect(
                &Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()),
                &o,
            )
            .unwrap();

        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, 4.0);
        assert_approx_eq!(l[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let o = Object::sphere_builder().build();

        let Shape::Sphere(s) = &o.shape else { unreachable!() };

        let l = s
            .intersect(
                &Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis()),
                &o,
            )
            .unwrap();

        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, 5.0);
        assert_approx_eq!(l[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let o = Object::sphere_builder().build();

        let Shape::Sphere(s) = &o.shape else { unreachable!() };

        assert!(s
            .intersect(
                &Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis()),
                &o
            )
            .is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let o = Object::sphere_builder().build();

        let Shape::Sphere(s) = &o.shape else { unreachable!() };

        let l = s
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()), &o)
            .unwrap();

        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, -1.0);
        assert_approx_eq!(l[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let o = Object::sphere_builder().build();

        let Shape::Sphere(s) = &o.shape else { unreachable!() };

        let l = s
            .intersect(
                &Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis()),
                &o,
            )
            .unwrap();

        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, -6.0);
        assert_approx_eq!(l[1].t, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_an_axis() {
        let s = Sphere::new();

        assert_approx_eq!(
            s.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::x_axis()
        );

        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::y_axis()
        );

        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::z_axis()
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::new();

        let sqrt_3_div_3 = f64::sqrt(3.0) / 3.0;
        let n =
            s.normal_at(&Point::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));

        assert_approx_eq!(
            n,
            Vector::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3)
        );

        assert_approx_eq!(n, n.normalise());
    }

    #[test]
    fn the_bounding_box_of_a_sphere() {
        let s = Sphere::new();

        assert_approx_eq!(
            s.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -1.0, -1.0),
                Point::new(1.0, 1.0, 1.0)
            )
        );
    }
}
