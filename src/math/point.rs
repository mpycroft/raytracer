use float_cmp::{ApproxEq, F64Margin};
use std::ops::{Add, AddAssign};

use super::vector::Vector;

/// A Point is a representation of a geometric position within the 3 dimensional
/// scene we are working on
#[derive(Clone, Copy, Debug)]
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
        rhs + self
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ApproxEq for Point {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::{assert_approx_eq, assert_approx_ne};

    #[test]
    fn creating_a_point() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert_approx_eq!(p.x, 4.3);
        assert_approx_eq!(p.y, -4.2);
        assert_approx_eq!(p.z, 3.1);
    }

    #[test]
    fn adding_a_point_and_a_vector() {
        assert_approx_eq!(
            Point::new(3.0, -2.0, 5.0) + Vector::new(-2.0, 3.0, 1.0),
            Point::new(1.0, 1.0, 6.0)
        );

        assert_approx_eq!(
            Vector::new(5.25, 4.14, 4.0) + Point::new(0.1, 0.01, 1.0),
            Point::new(5.35, 4.15, 5.0)
        );

        let mut p = Point::new(1.1, 2.2, 3.3);
        p += Vector::new(-0.1, 0.1, 0.0);

        assert_approx_eq!(p, Point::new(1.0, 2.3, 3.3));
    }

    #[test]
    fn comparing_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        let p3 = Point::new(1.0, 2.006, 3.0);

        assert_approx_eq!(p1, p2);

        assert_approx_ne!(p1, p3);
    }
}
