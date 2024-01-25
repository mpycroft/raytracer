use derive_new::new;

use crate::math::{float::impl_approx_eq, Point};

/// A `BoundingBox` is an axis aligned box that can be used to cut down the
/// number of intersection tests we need to perform.
#[derive(Clone, Copy, Debug, new)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point,
}

impl_approx_eq!(BoundingBox { minimum, maximum });

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use super::*;
    use crate::math::float::assert_approx_eq;

    #[test]
    fn creating_a_bounding_box() {
        let b = BoundingBox::new(
            Point::new(-10.0, NEG_INFINITY, 5.0),
            Point::new(5.1, INFINITY, 10.6),
        );

        assert_approx_eq!(b.minimum, Point::new(-10.0, NEG_INFINITY, 5.0));
        assert_approx_eq!(b.maximum, Point::new(5.1, INFINITY, 10.6));
    }
}
