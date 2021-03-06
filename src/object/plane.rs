use std::marker::PhantomData;

use crate::{
    intersect::IntersectionPoints,
    math::{Point, Ray, Vector},
    util::{approx::FLOAT_EPSILON, float::Float},
    Intersectable,
};

/// A Plane is an infinitely large plane situated along the x and z axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Plane<T: Float> {
    _phantom: PhantomData<T>,
}

impl<T: Float> Plane<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Float> Intersectable<T> for Plane<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>> {
        if ray.direction.y.abs() < T::from(FLOAT_EPSILON).unwrap() {
            return None;
        }

        Some(vec![-ray.origin.y / ray.direction.y].into())
    }

    fn normal_at(&self, _point: &Point<T>) -> Vector<T> {
        Vector::y_axis()
    }
}

add_approx_traits!(Plane<T> { true });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        assert!(Plane::new()
            .intersect(&Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        assert!(Plane::new()
            .intersect(&Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let i = Plane::new()
            .intersect(&Ray::new(Point::new(0.0, 1.0, 0.0), -Vector::y_axis()))
            .unwrap();

        assert_eq!(i.len(), 1);
        assert_float_relative_eq!(i[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let i = Plane::new()
            .intersect(&Ray::new(Point::new(0.0, -1.0, 0.0), Vector::y_axis()))
            .unwrap();

        assert_eq!(i.len(), 1);
        assert_float_relative_eq!(i[0], 1.0);
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();

        assert_relative_eq!(p.normal_at(&Point::origin()), Vector::y_axis());
        assert_relative_eq!(
            p.normal_at(&Point::new(10.0, 0.0, -10.0)),
            Vector::y_axis()
        );
        assert_relative_eq!(
            p.normal_at(&Point::new(-5.0, 0.0, 150.0)),
            Vector::y_axis()
        );
    }

    #[test]
    fn planes_are_approximately_equal() {
        let p1 = Plane::<f64>::new();
        let p2 = Plane::new();

        assert_abs_diff_eq!(p1, p2);

        assert_relative_eq!(p1, p2);

        assert_ulps_eq!(p1, p2);
    }
}
