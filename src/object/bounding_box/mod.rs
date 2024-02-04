mod bounded;

use std::{
    f64::{INFINITY, NEG_INFINITY},
    ops::{Add, AddAssign},
};

use derive_new::new;

pub use self::bounded::Bounded;
use crate::math::{
    float::{approx_eq, impl_approx_eq},
    Point, Ray, Transformable, Transformation,
};

/// A `BoundingBox` is an axis aligned box that can be used to cut down the
/// number of intersection tests we need to perform.
#[derive(Clone, Copy, Debug, new)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point,
}

impl BoundingBox {
    pub fn add_point(&mut self, point: Point) {
        self.minimum.x = self.minimum.x.min(point.x);
        self.minimum.y = self.minimum.y.min(point.y);
        self.minimum.z = self.minimum.z.min(point.z);

        self.maximum.x = self.maximum.x.max(point.x);
        self.maximum.y = self.maximum.y.max(point.y);
        self.maximum.z = self.maximum.z.max(point.z);
    }

    #[must_use]
    pub fn contains(&self, point: &Point) -> bool {
        (self.minimum.x..=self.maximum.x).contains(&point.x)
            && (self.minimum.y..=self.maximum.y).contains(&point.y)
            && (self.minimum.z..=self.maximum.z).contains(&point.z)
    }

    #[must_use]
    pub fn contains_box(&self, bounding_box: &BoundingBox) -> bool {
        self.contains(&bounding_box.minimum)
            && self.contains(&bounding_box.maximum)
    }

    #[must_use]
    pub fn is_intersected_by(&self, ray: &Ray) -> bool {
        let (x_min, x_max) = Self::check_axis(
            ray.origin.x,
            ray.direction.x,
            self.minimum.x,
            self.maximum.x,
        );
        let (y_min, y_max) = Self::check_axis(
            ray.origin.y,
            ray.direction.y,
            self.minimum.y,
            self.maximum.y,
        );
        let (z_min, z_max) = Self::check_axis(
            ray.origin.z,
            ray.direction.z,
            self.minimum.z,
            self.maximum.z,
        );

        let min = x_min.max(y_min).max(z_min);
        let max = x_max.min(y_max).min(z_max);

        if min > max || max < 0.0 {
            return false;
        }

        true
    }

    #[must_use]
    pub fn check_axis(
        origin: f64,
        direction: f64,
        min: f64,
        max: f64,
    ) -> (f64, f64) {
        let min_numerator = min - origin;
        let max_numerator = max - origin;

        let (min, max) = if approx_eq!(direction, 0.0) {
            (min_numerator * INFINITY, max_numerator * INFINITY)
        } else {
            (min_numerator / direction, max_numerator / direction)
        };

        if min > max {
            return (max, min);
        }

        (min, max)
    }
}

impl Add for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            Point::new(
                self.minimum.x.min(rhs.minimum.x),
                self.minimum.y.min(rhs.minimum.y),
                self.minimum.z.min(rhs.minimum.z),
            ),
            Point::new(
                self.maximum.x.max(rhs.maximum.x),
                self.maximum.y.max(rhs.maximum.y),
                self.maximum.z.max(rhs.maximum.z),
            ),
        )
    }
}

impl AddAssign for BoundingBox {
    fn add_assign(&mut self, rhs: Self) {
        self.add_point(rhs.minimum);
        self.add_point(rhs.maximum);
    }
}

impl From<Vec<Point>> for BoundingBox {
    fn from(value: Vec<Point>) -> Self {
        let mut bounding_box = BoundingBox::default();

        for point in value {
            bounding_box.add_point(point);
        }

        bounding_box
    }
}

impl Transformable for BoundingBox {
    fn apply(&self, transformation: &Transformation) -> Self {
        let p1 = self.minimum.apply(transformation);
        let p2 = Point::new(self.minimum.x, self.minimum.y, self.maximum.z)
            .apply(transformation);
        let p3 = Point::new(self.minimum.x, self.maximum.y, self.minimum.z)
            .apply(transformation);
        let p4 = Point::new(self.minimum.x, self.maximum.y, self.maximum.z)
            .apply(transformation);
        let p5 = Point::new(self.maximum.x, self.minimum.y, self.minimum.z)
            .apply(transformation);
        let p6 = Point::new(self.maximum.x, self.minimum.y, self.maximum.z)
            .apply(transformation);
        let p7 = Point::new(self.maximum.x, self.maximum.y, self.minimum.z)
            .apply(transformation);
        let p8 = self.maximum.apply(transformation);

        Self::from(vec![p1, p2, p3, p4, p5, p6, p7, p8])
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::new(
            Point::new(INFINITY, INFINITY, INFINITY),
            Point::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
        )
    }
}

impl_approx_eq!(BoundingBox { minimum, maximum });

#[cfg(test)]
mod tests {
    use std::f64::{
        consts::{FRAC_1_SQRT_2, FRAC_PI_4, SQRT_2},
        INFINITY, NEG_INFINITY,
    };

    use super::*;
    use crate::math::{float::*, Angle, Vector};

    #[test]
    fn creating_a_bounding_box() {
        let b = BoundingBox::new(
            Point::new(-10.0, NEG_INFINITY, 5.0),
            Point::new(5.1, INFINITY, 10.6),
        );

        assert_approx_eq!(b.minimum, Point::new(-10.0, NEG_INFINITY, 5.0));
        assert_approx_eq!(b.maximum, Point::new(5.1, INFINITY, 10.6));
    }

    #[test]
    fn adding_points_to_a_bounding_box() {
        let mut b = BoundingBox::default();

        b.add_point(Point::new(-5.0, 2.0, 0.0));
        b.add_point(Point::new(7.0, 0.0, -3.0));

        assert_approx_eq!(
            b,
            BoundingBox::new(
                Point::new(-5.0, 0.0, -3.0),
                Point::new(7.0, 2.0, 0.0)
            )
        );
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_point() {
        let b = BoundingBox::new(
            Point::new(5.0, -2.0, 0.0),
            Point::new(11.0, 4.0, 7.0),
        );

        assert!(b.contains(&Point::new(5.0, -2.0, 0.0)));
        assert!(b.contains(&Point::new(11.0, 4.0, 7.0)));
        assert!(b.contains(&Point::new(8.0, 1.0, 3.0)));
        assert!(!b.contains(&Point::new(3.0, 0.0, 3.0)));
        assert!(!b.contains(&Point::new(8.0, -4.0, 3.0)));
        assert!(!b.contains(&Point::new(8.0, 1.0, -1.0)));
        assert!(!b.contains(&Point::new(13.0, 1.0, 3.0)));
        assert!(!b.contains(&Point::new(8.0, 5.0, 3.0)));
        assert!(!b.contains(&Point::new(8.0, 1.0, 8.0)));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_box() {
        let b = BoundingBox::new(
            Point::new(5.0, -2.0, 0.0),
            Point::new(11.0, 4.0, 7.0),
        );

        assert!(b.contains_box(&BoundingBox::new(
            Point::new(5.0, -2.0, 0.0),
            Point::new(11.0, 4.0, 7.0)
        )));
        assert!(b.contains_box(&BoundingBox::new(
            Point::new(6.0, -1.0, 1.0),
            Point::new(10.0, 3.0, 6.0)
        )));
        assert!(!b.contains_box(&BoundingBox::new(
            Point::new(4.0, -3.0, -1.0),
            Point::new(10.0, 3.0, 6.0)
        )));
        assert!(!b.contains_box(&BoundingBox::new(
            Point::new(6.0, -1.0, 1.0),
            Point::new(12.0, 5.0, 8.0)
        )));
    }

    #[test]
    fn intersecting_a_bounding_box() {
        let b = BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        );

        assert!(b.is_intersected_by(&Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::z_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(0.0, 5.0, 0.0),
            -Vector::y_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(-5.0, 0.0, 0.5),
            Vector::x_axis()
        )));
    }

    #[test]
    fn intersecting_a_non_cuboid_bounding_box() {
        let b = BoundingBox::new(
            Point::new(5.0, -2.0, 0.0),
            Point::new(11.0, 4.0, 7.0),
        );

        assert!(b.is_intersected_by(&Ray::new(
            Point::new(14.0, 2.0, 3.0),
            -Vector::x_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(-2.0, -1.5, 0.0),
            Vector::x_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(6.0, 7.8, 7.0),
            -Vector::y_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(9.0, -4.0, 2.0),
            Vector::y_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(10.0, 0.0, 12.0),
            -Vector::z_axis()
        )));
        assert!(b.is_intersected_by(&Ray::new(
            Point::new(5.0, -1.0, -1.0),
            Vector::z_axis()
        )));

        assert!(!b.is_intersected_by(&Ray::new(
            Point::new(4.95, 1.0, 3.0),
            Vector::y_axis()
        )));
        assert!(!b.is_intersected_by(&Ray::new(
            Point::new(6.0, -3.0, 2.0),
            Vector::z_axis()
        )));
        assert!(!b.is_intersected_by(&Ray::new(
            Point::new(7.0, 0.0, 14.0),
            -Vector::x_axis()
        )));

        assert!(b.is_intersected_by(&Ray::new(
            Point::new(5.0, 1.0, 3.0),
            Vector::new(1.0, 1.0, 0.0).normalise()
        )));
    }

    #[test]
    fn adding_two_bounding_boxes() {
        let mut b = BoundingBox::new(
            Point::new(-5.0, -2.0, 0.0),
            Point::new(7.0, 4.0, 4.0),
        );

        assert_approx_eq!(
            b + BoundingBox::new(
                Point::new(8.0, -7.0, -2.0),
                Point::new(14.0, 2.0, 8.0)
            ),
            BoundingBox::new(
                Point::new(-5.0, -7.0, -2.0),
                Point::new(14.0, 4.0, 8.0)
            )
        );

        b += BoundingBox::new(
            Point::new(3.0, 0.0, -5.0),
            Point::new(4.0, 5.0, 1.0),
        );

        assert_approx_eq!(
            b,
            BoundingBox::new(
                Point::new(-5.0, -2.0, -5.0),
                Point::new(7.0, 5.0, 4.0)
            )
        );
    }

    #[test]
    fn creating_a_bounding_box_from_points() {
        let p1 = Point::new(1.0, 2.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 5.0);

        assert_approx_eq!(
            BoundingBox::from(vec![p1, p2]),
            BoundingBox::new(p1, p2)
        );

        let p1 = Point::origin();
        let p2 = Point::new(-1.0, 2.0, 2.0);
        let p3 = Point::new(5.0, 5.0, 5.0);
        let p4 = Point::new(0.0, -2.0, 0.0);
        let p5 = Point::new(4.5, -1.0, -0.5);

        assert_approx_eq!(
            BoundingBox::from(vec![p1, p2, p3, p4, p5]),
            BoundingBox::new(
                Point::new(-1.0, -2.0, -0.5),
                Point::new(5.0, 5.0, 5.0)
            )
        );
    }

    #[test]
    fn transforming_a_bounding_box() {
        let b = BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        );

        let one_plus_1_div_sqrt_2 = 1.0 + FRAC_1_SQRT_2;
        assert_approx_eq!(
            b.apply(
                &Transformation::new()
                    .rotate_y(Angle(FRAC_PI_4))
                    .rotate_x(Angle(FRAC_PI_4))
            ),
            BoundingBox::new(
                Point::new(
                    -SQRT_2,
                    -one_plus_1_div_sqrt_2,
                    -one_plus_1_div_sqrt_2
                ),
                Point::new(
                    SQRT_2,
                    one_plus_1_div_sqrt_2,
                    one_plus_1_div_sqrt_2
                )
            )
        );

        let t = Transformation::new()
            .rotate_y(Angle(FRAC_PI_4))
            .rotate_x(Angle(FRAC_PI_4));

        let one_plus_sqrt2_div_2 = 1.0 + f64::sqrt(2.0) / 2.0;
        assert_approx_eq!(
            b.apply(&t),
            BoundingBox::new(
                Point::new(
                    -SQRT_2,
                    -one_plus_sqrt2_div_2,
                    -one_plus_sqrt2_div_2
                ),
                Point::new(SQRT_2, one_plus_sqrt2_div_2, one_plus_sqrt2_div_2)
            ),
            epsilon = 0.000_01
        );
    }
}
