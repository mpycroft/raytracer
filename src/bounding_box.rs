use derive_new::new;

use crate::math::{
    float::impl_approx_eq, Point, Transformable, Transformation,
};

/// A `BoundingBox` is an axis aligned box that can be used to cut down the
/// number of intersection tests we need to perform.
#[derive(Clone, Copy, Debug, new)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point,
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
    use crate::math::{float::assert_approx_eq, Angle};

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
