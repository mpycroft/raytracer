use std::ops::{Add, AddAssign, Sub, SubAssign};

use derive_new::new;

use super::Vector;
use crate::util::float::Float;

/// A `Point` is a representation of a geometric position within the 3
/// dimensional scene we are working on.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Point<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point<T> {
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Vector<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Float> Add<Point<T>> for Vector<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Float + AddAssign> AddAssign<Vector<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Float> Sub for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Float> Sub<Vector<T>> for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Float + SubAssign> SubAssign<Vector<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

add_approx_traits!(Point<T> { x, y, z });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_point() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert_float_relative_eq!(p.x, 4.3);
        assert_float_relative_eq!(p.y, -4.2);
        assert_float_relative_eq!(p.z, 3.1);
    }

    #[test]
    fn creating_a_point_at_the_origin() {
        assert_relative_eq!(Point::origin(), Point::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn adding_a_vector_to_a_point() {
        assert_relative_eq!(
            Point::new(0.0, 0.5, 0.0) + Vector::new(1.3, 0.0, 0.0),
            Point::new(1.3, 0.5, 0.0)
        );

        assert_relative_eq!(
            Vector::new(0.5, 0.5, 2.3) + Point::new(2.1, 3.4, 0.7),
            Point::new(2.6, 3.9, 3.0)
        );

        let mut p = Point::new(-2.1, 0.3, 1.6);
        p += Vector::new(1.1, 4.6, 2.2);

        assert_relative_eq!(p, Point::new(-1.0, 4.9, 3.8));
    }

    #[test]
    fn subtracting_two_points() {
        assert_relative_eq!(
            Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0),
            Vector::new(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        assert_relative_eq!(
            Point::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0),
            Point::new(-2.0, -4.0, -6.0)
        );

        let mut p = Point::new(1.3, 5.2, 0.6);
        p -= Vector::new(0.0, -1.3, 2.5);

        assert_relative_eq!(p, Point::new(1.3, 6.5, -1.9));
    }

    #[test]
    fn points_are_approximately_equal() {
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
