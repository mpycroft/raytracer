use derive_new::new;
use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersection::TList,
    math::{
        float::{approx_eq, approx_ne},
        Point, Ray, Vector,
    },
};

// A `Cone` is a double napped cone centred on the origin and extending in both
// directions, its extend is given by minimum and maximum. Closed indicates if
// the ends are capped.
#[derive(Clone, Copy, Debug, new)]
pub struct Cone {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cone {
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<TList> {
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2)
            + ray.direction.z.powi(2);

        let b = 2.0
            * (ray.origin.x * ray.direction.x - ray.origin.y * ray.direction.y
                + ray.origin.z * ray.direction.z);

        let c =
            ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        let mut list = TList::new();

        if approx_eq!(a, 0.0) {
            if approx_ne!(b, 0.0) {
                list.push(-c / (2.0 * b));
            }
        } else {
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
        }

        self.intersect_caps(ray, &mut list)
    }

    #[must_use]
    pub fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }

    #[must_use]
    fn intersect_caps(&self, ray: &Ray, list: &mut TList) -> Option<TList> {
        let check_cap = |t: f64, r: f64| {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            x.powi(2) + z.powi(2) <= r.powi(2)
        };

        if self.closed && approx_ne!(ray.direction.y, 0.0) {
            let t = (self.minimum - ray.origin.y) / ray.direction.y;

            if check_cap(t, self.minimum) {
                list.push(t);
            }

            let t = (self.maximum - ray.origin.y) / ray.direction.y;

            if check_cap(t, self.maximum) {
                list.push(t);
            }
        }

        if list.is_empty() {
            return None;
        };

        Some(list.to_owned())
    }
}

impl ApproxEq for Cone {
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
    use std::f64::{consts::FRAC_1_SQRT_2, INFINITY};

    use super::*;
    use crate::math::float::*;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let c = Cone::new(-INFINITY, INFINITY, false);

        let test = |r, t0, t1| {
            let i = c.intersect(&r).unwrap();

            assert_eq!(i.len(), 2);
            assert_approx_eq!(i[0], t0, epsilon = 0.000_01);
            assert_approx_eq!(i[1], t1, epsilon = 0.000_01);
        };

        test(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()), 5.0, 5.0);
        test(
            Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0).normalise(),
            ),
            8.660_25,
            8.660_25,
        );
        test(
            Ray::new(
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0).normalise(),
            ),
            4.550_06,
            49.449_94,
        );
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let c = Cone::new(-INFINITY, INFINITY, false);

        let i = c
            .intersect(&Ray::new(
                Point::new(0.0, 0.0, -1.0),
                Vector::new(0.0, 1.0, 1.0).normalise(),
            ))
            .unwrap();

        assert_eq!(i.len(), 1);
        assert_approx_eq!(i[0], 0.353_55, epsilon = 0.000_01);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let c = Cone::new(-0.5, 0.5, true);

        let i = c
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::y_axis()));

        assert!(i.is_none());

        let i = c
            .intersect(&Ray::new(
                Point::new(0.0, 0.0, -0.25),
                Vector::new(0.0, 1.0, 1.0).normalise(),
            ))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_approx_eq!(i[0], 0.088_39, epsilon = 0.000_01);
        assert_approx_eq!(i[1], FRAC_1_SQRT_2, epsilon = 0.000_01);

        let i = c
            .intersect(&Ray::new(Point::new(0.0, 0.0, -0.25), Vector::y_axis()))
            .unwrap();

        assert_eq!(i.len(), 4);
        assert_approx_eq!(i[0], 0.25);
        assert_approx_eq!(i[1], -0.25);
        assert_approx_eq!(i[2], -0.5);
        assert_approx_eq!(i[3], 0.5);
    }

    #[test]
    fn comparing_cones() {
        let c1 = Cone::new(0.0, 1.0, true);
        let c2 = Cone::new(0.0, 1.0, true);
        let c3 = Cone::new(0.0, 1.0, false);

        assert_approx_eq!(c1, c2);

        assert_approx_ne!(c1, c3);
    }
}
