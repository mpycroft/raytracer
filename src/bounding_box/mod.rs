mod bounded;

use std::{
    f64::{EPSILON, INFINITY},
    ops::{Add, AddAssign},
};

use derive_new::new;

pub(super) use self::bounded::Bounded;
use crate::math::{
    float::impl_approx_eq, Point, Ray, Transformable, Transformation,
};

/// A `BoundingBox` is an axis aligned box that can be used to cut down the
/// number of intersection tests we need to perform.
#[derive(Clone, Copy, Debug, new)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point,
}

impl BoundingBox {
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

        let (min, max) = if direction.abs() >= EPSILON {
            (min_numerator / direction, max_numerator / direction)
        } else {
            (min_numerator * INFINITY, max_numerator * INFINITY)
        };

        if min > max {
            return (max, min);
        }

        (min, max)
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Self) -> Self::Output {
        BoundingBox::new(
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
        self.minimum = Point::new(
            self.minimum.x.min(rhs.minimum.x),
            self.minimum.y.min(rhs.minimum.y),
            self.minimum.z.min(rhs.minimum.z),
        );
        self.maximum = Point::new(
            self.maximum.x.max(rhs.maximum.x),
            self.maximum.y.max(rhs.maximum.y),
            self.maximum.z.max(rhs.maximum.z),
        );
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

        macro_rules! find {
            ($func:path, $axis:ident) => {
                $func(
                    p1.$axis,
                    $func(
                        p2.$axis,
                        $func(
                            p3.$axis,
                            $func(
                                p4.$axis,
                                $func(
                                    p5.$axis,
                                    $func(p6.$axis, $func(p7.$axis, p8.$axis)),
                                ),
                            ),
                        ),
                    ),
                )
            };
        }

        let minimum = Point::new(
            find!(f64::min, x),
            find!(f64::min, y),
            find!(f64::min, z),
        );
        let maximum = Point::new(
            find!(f64::max, x),
            find!(f64::max, y),
            find!(f64::max, z),
        );

        BoundingBox::new(minimum, maximum)
    }
}

impl_approx_eq!(BoundingBox { minimum, maximum });

#[cfg(test)]
mod tests {
    use std::f64::{
        consts::{FRAC_PI_4, SQRT_2},
        INFINITY, NEG_INFINITY,
    };

    use super::*;
    use crate::math::{float::assert_approx_eq, Angle, Vector};

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
            Point::new(-2.0, -2.0, -2.0),
            Point::new(2.0, 2.0, 2.0),
        );

        assert_approx_eq!(
            b + BoundingBox::new(
                Point::new(-1.0, -3.0, 0.0),
                Point::new(0.0, -1.0, 5.0)
            ),
            BoundingBox::new(
                Point::new(-2.0, -3.0, -2.0),
                Point::new(2.0, 2.0, 5.0)
            )
        );

        b += BoundingBox::new(
            Point::new(3.0, 0.0, -5.0),
            Point::new(4.0, 1.0, 1.0),
        );

        assert_approx_eq!(
            b,
            BoundingBox::new(
                Point::new(-2.0, -2.0, -5.0),
                Point::new(4.0, 2.0, 2.0)
            )
        );
    }

    #[test]
    fn transforming_a_bounding_box() {
        let b = BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        );

        assert_approx_eq!(
            b.apply(&Transformation::new().translate(1.0, -1.0, 0.0)),
            BoundingBox::new(
                Point::new(0.0, -2.0, -1.0),
                Point::new(2.0, 0.0, 1.0)
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
