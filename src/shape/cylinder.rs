use std::f64::EPSILON;

use derive_new::new;

use crate::{
    intersection::TList,
    math::{
        float::{approx_eq, approx_ne, impl_approx_eq},
        Point, Ray, Vector,
    },
};

// A `Cylinder` is an infinite cylinder of radius 1 centred on the y axis.
#[derive(Clone, Copy, Debug, new)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cylinder {
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<TList> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        let mut list = TList::new();

        if approx_eq!(a, 0.0) {
            return self.intersect_caps(ray, &mut list);
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

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            list.push(t0);
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            list.push(t1);
        }

        self.intersect_caps(ray, &mut list)
    }

    #[must_use]
    pub fn normal_at(&self, point: &Point) -> Vector {
        let distance = point.x.powi(2) + point.z.powi(2);

        if distance < 1.0 && point.y >= self.maximum - EPSILON {
            return Vector::y_axis();
        } else if distance < 1.0 && point.y <= self.minimum + EPSILON {
            return -Vector::y_axis();
        }

        Vector::new(point.x, 0.0, point.z)
    }

    #[must_use]
    fn intersect_caps(&self, ray: &Ray, list: &mut TList) -> Option<TList> {
        let check_cap = |t: f64| {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            x.powi(2) + z.powi(2) <= 1.0
        };

        if self.closed && approx_ne!(ray.direction.y, 0.0) {
            let t = (self.minimum - ray.origin.y) / ray.direction.y;

            if check_cap(t) {
                list.push(t);
            }

            let t = (self.maximum - ray.origin.y) / ray.direction.y;

            if check_cap(t) {
                list.push(t);
            }
        }

        if list.is_empty() {
            return None;
        };

        Some(list.to_owned())
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
        let c = Cylinder::new(-INFINITY, INFINITY, false);

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
        let c = Cylinder::new(-INFINITY, INFINITY, false);

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
    fn intersecting_a_constrained_cylinder() {
        let c = Cylinder::new(1.0, 2.0, false);

        assert!(c
            .intersect(&Ray::new(
                Point::new(0.0, 1.5, 0.0),
                Vector::new(0.1, 1.0, 0.0).normalise()
            ))
            .is_none());
        assert!(c
            .intersect(&Ray::new(Point::new(0.0, 3.0, -5.0), Vector::z_axis()))
            .is_none());
        assert!(c
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .is_none());
        assert!(c
            .intersect(&Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis()))
            .is_none());
        assert!(c
            .intersect(&Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis()))
            .is_none());

        let i = c
            .intersect(&Ray::new(Point::new(0.0, 1.5, -2.0), Vector::z_axis()))
            .unwrap();

        assert_approx_eq!(i[0], 1.0);
        assert_approx_eq!(i[1], 3.0);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let c = Cylinder::new(1.0, 2.0, true);

        let test = |r, t0, t1| {
            let i = c.intersect(&r).unwrap();

            assert_approx_eq!(i[0], t0, epsilon = 0.000_01);
            assert_approx_eq!(i[1], t1, epsilon = 0.000_01);
        };

        test(Ray::new(Point::new(0.0, 3.0, 0.0), -Vector::y_axis()), 2.0, 1.0);
        test(
            Ray::new(
                Point::new(0.0, 3.0, -2.0),
                -Vector::new(0.0, -1.0, 2.0).normalise(),
            ),
            -3.354_1,
            -2.236_07,
        );
        test(
            Ray::new(
                Point::new(0.0, 4.0, -2.0),
                -Vector::new(0.0, -1.0, 1.0).normalise(),
            ),
            -4.242_64,
            -2.828_43,
        );
        test(
            Ray::new(
                Point::new(0.0, 0.0, -2.0),
                -Vector::new(0.0, 1.0, 2.0).normalise(),
            ),
            -3.354_1,
            -2.236_07,
        );
        test(
            Ray::new(
                Point::new(0.0, -1.0, -2.0),
                -Vector::new(0.0, 1.0, 1.0).normalise(),
            ),
            -2.828_43,
            -4.242_64,
        );
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::new(-INFINITY, INFINITY, false);

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

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let c = Cylinder::new(1.0, 2.0, true);

        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 1.0, 0.0)),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.5, 1.0, 0.0)),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 1.0, 0.5)),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 2.0, 0.0)),
            Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.5, 2.0, 0.0)),
            Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 2.0, 0.5)),
            Vector::y_axis()
        );
    }
}
