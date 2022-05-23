use std::marker::PhantomData;

use crate::{
    intersect::{Intersectable, IntersectionPoints},
    math::{Point, Ray, Vector},
    util::float::Float,
};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Sphere<T: Float> {
    _phantom: PhantomData<T>,
}

impl<T: Float> Sphere<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Float> Intersectable<T> for Sphere<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(&ray.direction);
        let b = T::two() * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - T::one();

        let discriminant = b * b - T::from(4.0f64).unwrap() * a * c;
        if discriminant < T::zero() {
            return None;
        }

        let discriminant = discriminant.sqrt();
        let a = T::two() * a;

        let t1 = (-b - discriminant) / a;
        let t2 = (-b + discriminant) / a;

        Some(vec![t1, t2].into())
    }

    fn normal_at(&self, point: &Point<T>) -> Vector<T> {
        *point - Point::origin()
    }
}

add_approx_traits!(Sphere<T> { true });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let i = Sphere::new()
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], 4.0);
        assert_float_relative_eq!(i[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let i = Sphere::new()
            .intersect(&Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis()))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], 5.0);
        assert_float_relative_eq!(i[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let i = Sphere::new()
            .intersect(&Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis()));

        assert!(i.is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let i = Sphere::<f64>::new()
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], -1.0);
        assert_float_relative_eq!(i[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let i = Sphere::new()
            .intersect(&Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis()))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], -6.0);
        assert_float_relative_eq!(i[1], -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        assert_relative_eq!(
            Sphere::new().normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        assert_relative_eq!(
            Sphere::new().normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        assert_relative_eq!(
            Sphere::new().normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let n =
            Sphere::new().normal_at(&Point::new(0.577_35, 0.577_35, 0.577_35));

        assert_relative_eq!(n, Vector::new(0.577_35, 0.577_35, 0.577_35));
        assert_relative_eq!(n, n.normalise());
    }

    #[test]
    fn the_normal_is_a_normalised_vector() {
        let n =
            Sphere::new().normal_at(&Point::new(0.577_35, 0.577_35, 0.577_35));

        assert_relative_eq!(n, n.normalise());
    }

    #[test]
    fn spheres_are_approximately_equal() {
        let s1 = Sphere::<f64>::new();
        let s2 = Sphere::new();

        assert_abs_diff_eq!(s1, s2);

        assert_relative_eq!(s1, s2);

        assert_ulps_eq!(s1, s2);
    }
}
