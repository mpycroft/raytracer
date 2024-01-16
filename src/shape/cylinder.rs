use derive_new::new;

use crate::{
    intersection::TList,
    math::{
        float::{approx_eq, impl_approx_eq},
        Point, Ray, Vector,
    },
};

// A `Cylinder` is an infinite cylinder of radius 1 centred on the y axis.
#[derive(Clone, Copy, Debug, new)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
}

impl Cylinder {
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<TList> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if approx_eq!(a, 0.0) {
            return None;
        };

        let b = 2.0
            * (ray.origin.x * ray.direction.x + ray.origin.z * ray.direction.z);

        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        };

        let discriminant = discriminant.sqrt();
        let a = 2.0 * a;

        let t0 = (-b - discriminant) / a;
        let t1 = (-b + discriminant) / a;

        Some(TList::from(vec![t0, t1]))
    }

    #[must_use]
    pub fn normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x, 0.0, point.z)
    }
}

impl_approx_eq!(Cylinder { minimum, maximum });

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let c = Cylinder::new(-INFINITY, INFINITY);

        assert!(c
            .intersect(&Ray::new(Point::new(1.0, 0.0, 0.0), Vector::y_axis()))
            .is_none());
        assert!(c
            .intersect(&Ray::new(Point::origin(), Vector::y_axis()))
            .is_none());
        assert!(c
            .intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0).normalise()
            ))
            .is_none());
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let c = Cylinder::new(-INFINITY, INFINITY);

        let test = |r, t0, t1| {
            let i = c.intersect(&r).unwrap();

            assert_approx_eq!(i[0], t0, epsilon = 0.000_01);
            assert_approx_eq!(i[1], t1, epsilon = 0.000_01);
        };

        test(Ray::new(Point::new(1.0, 0.0, -5.0), Vector::z_axis()), 5.0, 5.0);
        test(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()), 4.0, 6.0);
        test(
            Ray::new(
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0).normalise(),
            ),
            6.807_98,
            7.088_72,
        );
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::new(-INFINITY, INFINITY);

        assert_approx_eq!(
            c.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::x_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 5.0, -1.0)),
            -Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, -2.0, 1.0)),
            Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-1.0, 1.0, 0.0)),
            -Vector::x_axis()
        );
    }
}
