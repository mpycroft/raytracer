use crate::{
    intersect::Intersectable,
    math::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        Point, Ray,
    },
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();
        let a = 2.0 * a;

        let t1 = (-b - discriminant) / a;
        let t2 = (-b + discriminant) / a;

        Some(vec![t1, t2])
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsDiffEq for Sphere {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        true
    }
}

impl RelativeEq for Sphere {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        true
    }
}

impl UlpsEq for Sphere {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vector;

    #[test]
    fn new() {
        let _ = Sphere::new();
    }

    #[test]
    fn intersect() {
        let s = Sphere::new();
        let v = Vector::new(0.0, 0.0, 1.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], 4.0);
        assert_float_relative_eq!(i[1], 6.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 1.0, -5.0), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], 5.0);
        assert_float_relative_eq!(i[1], 5.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 2.0, -5.0), v));

        assert!(i.is_none());

        let i = s.intersect(&Ray::new(Point::origin(), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], -1.0);
        assert_float_relative_eq!(i[1], 1.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 0.0, 5.0), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], -6.0);
        assert_float_relative_eq!(i[1], -4.0);
    }
}
