use crate::{
    intersect::{Intersectable, IntersectionPoints},
    math::{Point, Ray, Vector},
};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Sphere;

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPoints> {
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

        Some(vec![t1, t2].into())
    }

    fn normal_at(&self, point: &Point) -> Vector {
        *point - Point::origin()
    }
}

add_approx_traits!(Sphere { true });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn intersect() {
        let s = Sphere;
        let v = Vector::z_axis();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), v);

        let i = s.intersect(&r).unwrap();

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

    #[test]
    fn normal_at() {
        let s = Sphere;

        assert_relative_eq!(
            s.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_relative_eq!(
            s.normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_relative_eq!(
            s.normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        let n = s.normal_at(&Point::new(0.577_35, 0.577_35, 0.577_35));
        assert_relative_eq!(n, Vector::new(0.577_35, 0.577_35, 0.577_35));
        assert_relative_eq!(n, n.normalise());
    }

    #[test]
    fn approx() {
        let s1 = Sphere;
        let s2 = Sphere;

        assert_abs_diff_eq!(s1, s2);

        assert_relative_eq!(s1, s2);

        assert_ulps_eq!(s1, s2);
    }
}
