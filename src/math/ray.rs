use super::{
    float::{FLOAT_EPSILON, FLOAT_ULPS},
    Point, Vector,
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// A Ray represents a geometric vector with a specific origin point and
/// pointing in some direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }
}

impl AbsDiffEq for Ray {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.direction.abs_diff_eq(&other.direction, epsilon)
    }
}

impl RelativeEq for Ray {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.origin.relative_eq(&other.origin, epsilon, max_relative)
            && self.direction.relative_eq(
                &other.direction,
                epsilon,
                max_relative,
            )
    }
}

impl UlpsEq for Ray {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        self.origin.ulps_eq(&other.origin, epsilon, max_ulps)
            && self.direction.ulps_eq(&other.direction, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(4.0, 5.0, 6.0);

        let r = Ray::new(p, v);

        assert_relative_eq!(r.origin, p);
        assert_relative_eq!(r.direction, v);
    }

    #[test]
    fn approx() {
        let r1 =
            Ray::new(Point::new(0.0, 1.5, -2.3), Vector::new(9.5, 0.1, 0.5));
        let r2 =
            Ray::new(Point::new(0.0, 1.5, -2.3), Vector::new(9.5, 0.1, 0.5));
        let r3 = Ray::new(
            Point::new(0.000_01, 1.5, -2.3),
            Vector::new(9.502, 0.1, 0.5),
        );

        assert_abs_diff_eq!(r1, r2);
        assert_abs_diff_ne!(r1, r3);

        assert_relative_eq!(r1, r2);
        assert_relative_ne!(r1, r3);

        assert_ulps_eq!(r1, r2);
        assert_ulps_ne!(r1, r3);
    }
}
