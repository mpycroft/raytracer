use std::ops::{Add, AddAssign, Sub, SubAssign};

use derive_new::new;

use super::{float::impl_approx_eq, Vector};
use crate::util::impl_deserialize_tuple;

/// A Point is a representation of a geometric position within the 3 dimensional
/// scene we are working on.
#[derive(Clone, Copy, Debug, new)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    #[must_use]
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
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

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

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

impl_approx_eq!(Point { x, y, z });

impl_deserialize_tuple!(Point);

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_point() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert_approx_eq!(p.x, 4.3);
        assert_approx_eq!(p.y, -4.2);
        assert_approx_eq!(p.z, 3.1);

        assert_approx_eq!(Point::origin(), Point::new(0.0, 0.0, 0.0));
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
    fn subtracting_two_points() {
        assert_approx_eq!(
            Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0),
            Vector::new(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        assert_approx_eq!(
            Point::new(3.7, 7.1, 2.001) - Vector::new(1.0, 1.8, 0.0),
            Point::new(2.7, 5.3, 2.001)
        );

        let mut p = Point::new(1.5, 0.3, -0.5);
        p -= Vector::new(0.3, 0.4, 0.5);

        assert_approx_eq!(p, Point::new(1.2, -0.1, -1.0));
    }

    #[test]
    fn comparing_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        let p3 = Point::new(1.0, 2.006, 3.0);

        assert_approx_eq!(p1, p2);

        assert_approx_ne!(p1, p3);
    }

    #[test]
    fn deserialize_point() {
        let p: Point = from_str("[1.0, 0.5, 2]").unwrap();

        assert_approx_eq!(p, Point::new(1.0, 0.5, 2.0));
    }
}
