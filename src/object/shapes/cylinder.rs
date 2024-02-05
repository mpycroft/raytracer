use std::f64::EPSILON;

use derive_new::new;
use float_cmp::{ApproxEq, F64Margin};

use super::{Bounded, BoundingBox, Intersectable};
use crate::{
    intersection::{Intersection, TList, TValues},
    math::{
        float::{approx_eq, approx_ne},
        Point, Ray, Vector,
    },
};

// A `Cylinder` is an cylinder of radius 1 centred on the y axis which extends
// from minimum to maximum. Closed indicates if the cylinder is capped on both
// ends.
#[derive(Clone, Copy, Debug, new)]
pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cylinder {
    #[must_use]
    fn intersect_caps(&self, ray: &Ray, mut list: TList) -> Option<TList> {
        let check_cap = |t: f64| {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            x.powi(2) + z.powi(2) <= 1.0
        };

        if self.closed && approx_ne!(ray.direction.y, 0.0) {
            let t = (self.minimum - ray.origin.y) / ray.direction.y;

            if check_cap(t) {
                list.push(TValues::new(t));
            }

            let t = (self.maximum - ray.origin.y) / ray.direction.y;

            if check_cap(t) {
                list.push(TValues::new(t));
            }
        }

        if list.is_empty() {
            return None;
        };

        Some(list)
    }
}

impl Intersectable for Cylinder {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<TList> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        let mut list = TList::new();

        if approx_eq!(a, 0.0) {
            return self.intersect_caps(ray, list);
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
            list.push(TValues::new(t0));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            list.push(TValues::new(t1));
        }

        self.intersect_caps(ray, list)
    }

    #[must_use]
    fn normal_at(&self, point: &Point, _intersection: &Intersection) -> Vector {
        let distance = point.x.powi(2) + point.z.powi(2);

        if distance < 1.0 && point.y >= self.maximum - EPSILON {
            return Vector::y_axis();
        } else if distance < 1.0 && point.y <= self.minimum + EPSILON {
            return -Vector::y_axis();
        }

        Vector::new(point.x, 0.0, point.z)
    }
}

impl Bounded for Cylinder {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point::new(-1.0, self.minimum, -1.0),
            Point::new(1.0, self.maximum, 1.0),
        )
    }
}

impl ApproxEq for &Cylinder {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        if self.closed == other.closed
            && self.minimum.approx_eq(other.minimum, margin)
            && self.maximum.approx_eq(other.maximum, margin)
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use super::*;
    use crate::{math::float::*, Object};

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

            assert_eq!(i.len(), 2);
            assert_approx_eq!(i[0].t, t0, epsilon = 0.000_01);
            assert_approx_eq!(i[1].t, t1, epsilon = 0.000_01);
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

        assert_eq!(i.len(), 2);
        assert_approx_eq!(i[0].t, 1.0);
        assert_approx_eq!(i[1].t, 3.0);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let c = Cylinder::new(1.0, 2.0, true);

        let test = |r, t0, t1| {
            let i = c.intersect(&r).unwrap();

            assert_eq!(i.len(), 2);
            assert_approx_eq!(i[0].t, t0, epsilon = 0.000_01);
            assert_approx_eq!(i[1].t, t1, epsilon = 0.000_01);
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

        let o = Object::test_builder().build();
        let i = Intersection::new(&o, 0.0);

        assert_approx_eq!(
            c.normal_at(&Point::new(1.0, 0.0, 0.0), &i),
            Vector::x_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 5.0, -1.0), &i),
            -Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, -2.0, 1.0), &i),
            Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-1.0, 1.0, 0.0), &i),
            -Vector::x_axis()
        );
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let c = Cylinder::new(1.0, 2.0, true);

        let o = Object::test_builder().build();
        let i = Intersection::new(&o, 0.0);

        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 1.0, 0.0), &i),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.5, 1.0, 0.0), &i),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 1.0, 0.5), &i),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 2.0, 0.0), &i),
            Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.5, 2.0, 0.0), &i),
            Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.0, 2.0, 0.5), &i),
            Vector::y_axis()
        );
    }

    #[test]
    fn the_bounding_box_of_a_cylinder() {
        let c = Cylinder::new(-5.0, 3.0, true);

        assert_approx_eq!(
            c.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -5.0, -1.0),
                Point::new(1.0, 3.0, 1.0)
            )
        );
    }

    #[test]
    fn comparing_cylinders() {
        let c1 = Cylinder::new(0.0, 1.0, true);
        let c2 = Cylinder::new(0.0, 1.0, true);
        let c3 = Cylinder::new(0.001, 1.0, true);

        assert_approx_eq!(c1, &c2);

        assert_approx_ne!(c1, &c3);
    }
}
