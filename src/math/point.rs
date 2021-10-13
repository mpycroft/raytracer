use super::float::{FLOAT_EPSILON, FLOAT_ULPS};
use super::Vector;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A Point is a representation of a geometric position within the 3 dimensional
/// scene we are working on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl AbsDiffEq for Point {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl RelativeEq for Point {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.x.relative_eq(&other.x, epsilon, max_relative)
            && self.y.relative_eq(&other.y, epsilon, max_relative)
            && self.z.relative_eq(&other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Point {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        self.x.ulps_eq(&other.x, epsilon, max_ulps)
            && self.y.ulps_eq(&other.y, epsilon, max_ulps)
            && self.z.ulps_eq(&other.z, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert_float_relative_eq!(p.x, 4.3);
        assert_float_relative_eq!(p.y, -4.2);
        assert_float_relative_eq!(p.z, 3.1);
    }

    #[test]
    fn add() {
        assert_relative_eq!(
            Point::new(0.0, 0.5, 0.0) + Vector::new(1.3, 0.0, 0.0),
            Point::new(1.3, 0.5, 0.0)
        );

        assert_relative_eq!(
            Vector::new(0.5, 0.5, 2.3) + Point::new(2.1, 3.4, 0.7),
            Point::new(2.6, 3.9, 3.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut p = Point::new(-2.1, 0.3, 1.6);
        p += Vector::new(1.1, 4.6, 2.2);

        assert_relative_eq!(p, Point::new(-1.0, 4.9, 3.8));
    }

    #[test]
    fn sub() {
        assert_relative_eq!(
            Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0),
            Vector::new(-2.0, -4.0, -6.0)
        );

        assert_relative_eq!(
            Point::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0),
            Point::new(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn sub_assign() {
        let mut p = Point::new(1.3, 5.2, 0.6);
        p -= Vector::new(0.0, -1.3, 2.5);

        assert_relative_eq!(p, Point::new(1.3, 6.5, -1.9));
    }

    #[test]
    fn approx() {
        let p1 = Point::new(2.3, 0.000_02, 51.61);
        let p2 = Point::new(2.3, 0.000_02, 51.61);
        let p3 = Point::new(2.301, 0.000_03, 51.61);

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
