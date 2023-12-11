use derive_more::Constructor;

use super::{float::impl_approx_eq, point::Point, vector::Vector};

/// A Ray represents a geometric vector with a specific origin point and
/// pointing in some direction.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl_approx_eq!(Ray { origin, direction });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_ray() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(4.0, 5.0, 6.0);

        let r = Ray::new(p, v);

        assert_approx_eq!(r.origin, p);
        assert_approx_eq!(r.direction, v);
    }

    #[test]
    fn comparing_rays() {
        let r1 =
            Ray::new(Point::new(1.06, 0.0, -20.5), Vector::new(1.0, 0.0, -2.7));
        let r2 =
            Ray::new(Point::new(1.06, 0.0, -20.5), Vector::new(1.0, 0.0, -2.7));
        let r3 = Ray::new(
            Point::new(1.06, 0.000_09, -20.5),
            Vector::new(1.0, 0.0, -2.7),
        );

        assert_approx_eq!(r1, r2);

        assert_approx_ne!(r1, r3);
    }
}
