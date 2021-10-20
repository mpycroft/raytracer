use crate::{
    intersect::{Intersectable, Intersection, IntersectionList},
    math::{Matrix, Point, Ray, Vector},
};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix<4>,
}

impl Sphere {
    pub fn new() -> Self {
        Self::with_transform(Matrix::identity())
    }

    pub fn with_transform(transform: Matrix<4>) -> Self {
        Self { transform }
    }

    pub fn normal_at(&self, point: &Point) -> Vector {
        let inv_matrix = self.transform.invert().unwrap();
        let object_point = inv_matrix * *point;
        let object_normal = object_point - Point::origin();

        (inv_matrix.transpose() * object_normal).normalise()
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let ray = ray.transform(&self.transform.invert().unwrap());

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

        Some(IntersectionList::from(vec![
            Intersection::new(self, t1),
            Intersection::new(self, t2),
        ]))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

add_approx_traits!(Sphere { transform });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vector;
    use approx::*;
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn new() {
        assert_relative_eq!(Sphere::new().transform, Matrix::identity());
    }

    #[test]
    fn with_transform() {
        let m = Matrix::translate(2.0, 3.0, 4.0);

        assert_relative_eq!(Sphere::with_transform(m).transform, m);
    }

    #[test]
    fn intersect() {
        let s = Sphere::new();
        let v = Vector::new(0.0, 0.0, 1.0);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), v);

        let i = s.intersect(&r).unwrap();

        assert_eq!(i.len(), 2);
        assert_relative_eq!(*i[0].object, s);
        assert_float_relative_eq!(i[0].t, 4.0);
        assert_relative_eq!(*i[1].object, s);
        assert_float_relative_eq!(i[1].t, 6.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 1.0, -5.0), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0].t, 5.0);
        assert_float_relative_eq!(i[1].t, 5.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 2.0, -5.0), v));

        assert!(i.is_none());

        let i = s.intersect(&Ray::new(Point::origin(), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0].t, -1.0);
        assert_float_relative_eq!(i[1].t, 1.0);

        let i = s.intersect(&Ray::new(Point::new(0.0, 0.0, 5.0), v)).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0].t, -6.0);
        assert_float_relative_eq!(i[1].t, -4.0);

        let s = Sphere::with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let i = s.intersect(&r).unwrap();

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0].t, 3.0);
        assert_float_relative_eq!(i[1].t, 7.0);

        let s = Sphere::with_transform(Matrix::translate(5.0, 0.0, 0.0));
        assert!(s.intersect(&r).is_none());
    }

    #[test]
    fn normal_at() {
        let s = Sphere::new();

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

        assert_relative_eq!(
            Sphere::with_transform(Matrix::translate(0.0, 1.0, 0.0)).normal_at(
                &Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
            ),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        assert_relative_eq!(
            Sphere::with_transform(
                Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI / 5.0)
            )
            .normal_at(&Point::new(0.0, 0.577_35, -0.577_35)),
            Vector::new(0.0, 0.970_142, -0.242_536)
        );
    }

    #[test]
    fn approx() {
        let m = Matrix::rotate_y(1.5);

        let s1 = Sphere::with_transform(m);
        let s2 = Sphere::with_transform(m);
        let s3 = Sphere::with_transform(Matrix::translate(0.0, 1.5, 2.3));

        assert_abs_diff_eq!(s1, s2);
        assert_abs_diff_ne!(s1, s3);

        assert_relative_eq!(s1, s2);
        assert_relative_ne!(s1, s3);

        assert_ulps_eq!(s1, s2);
        assert_ulps_ne!(s1, s3);
    }
}
