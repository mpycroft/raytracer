use crate::{
    intersect::Intersectable,
    math::{Point, Ray},
};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug)]
pub struct Sphere;

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{float::*, Vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0], 4.0);
        assert_approx_eq!(i[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0], 5.0);
        assert_approx_eq!(i[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::origin(), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0], -1.0);
        assert_approx_eq!(i[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0], -6.0);
        assert_approx_eq!(i[1], -4.0);
    }
}
