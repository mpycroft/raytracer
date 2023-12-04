use float_cmp::{ApproxEq, F64Margin};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::float::approx_eq;

/// A Vector is a representation of a geometric vector, pointing in a given
/// direction and with a magnitude.
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalise(&self) -> Self {
        let magnitude = self.magnitude();

        if approx_eq!(magnitude, 0.0) {
            return Self::new(0.0, 0.0, 0.0);
        }

        *self / magnitude
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

impl ApproxEq for Vector {
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
    use crate::math::float::{assert_approx_eq, assert_approx_ne};

    use super::*;

    #[test]
    fn creating_a_vector() {
        let p = Vector::new(2.8, 4.0, -0.7);

        assert_approx_eq!(p.x, 2.8);
        assert_approx_eq!(p.y, 4.0);
        assert_approx_eq!(p.z, -0.7);
    }

    #[test]
    fn computing_the_magnitude_of_a_vector() {
        assert_approx_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);

        assert_approx_eq!(
            Vector::new(1.0, 2.0, 3.0).magnitude(),
            f64::sqrt(14.0)
        );
        assert_approx_eq!(
            Vector::new(-1.0, -2.0, -3.0).magnitude(),
            f64::sqrt(14.0)
        );
    }

    #[test]
    fn normalising_a_vector() {
        assert_approx_eq!(
            Vector::new(4.0, 0.0, 0.0).normalise(),
            Vector::new(1.0, 0.0, 0.0)
        );

        let v = Vector::new(1.0, 2.0, 3.0).normalise();
        let sqrt_14 = f64::sqrt(14.0);
        assert_approx_eq!(
            v,
            Vector::new(1.0 / sqrt_14, 2.0 / sqrt_14, 3.0 / sqrt_14)
        );
        assert_approx_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 0.0, 0.0).normalise();
        assert_approx_eq!(v, Vector::new(0.0, 0.0, 0.0));
        assert_approx_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn computing_the_dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let d = 20.0;

        assert_approx_eq!(v1.dot(v2), d);
        assert_approx_eq!(v2.dot(v1), d);
    }

    #[test]
    fn computing_the_cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_approx_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_approx_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn adding_two_vectors() {
        assert_approx_eq!(
            Vector::new(2.3, 5.1, -3.0) + Vector::new(1.0, 1.0, 1.0),
            Vector::new(3.3, 6.1, -2.0)
        );

        let mut v = Vector::new(-0.6, 0.5, 1.2);
        v += Vector::new(-0.0, 0.5, -0.2);

        assert_approx_eq!(v, Vector::new(-0.6, 1.0, 1.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        assert_approx_eq!(
            Vector::new(1.0, 2.0, 3.0) - Vector::new(3.0, 2.0, 1.0),
            Vector::new(-2.0, 0.0, 2.0)
        );

        let mut v = Vector::new(-1.0, -2.0, -3.0);
        v -= Vector::new(-2.0, 2.5, -0.1);

        assert_approx_eq!(v, Vector::new(1.0, -4.5, -2.9));

        assert_approx_eq!(
            Vector::new(0.0, 0.0, 0.0) - Vector::new(1.0, -2.0, 3.0),
            Vector::new(-1.0, 2.0, -3.0)
        );
    }

    #[test]
    fn multiplying_a_vector_by_a_scaler() {
        assert_approx_eq!(
            Vector::new(1.0, -2.0, 3.0) * 3.5,
            Vector::new(3.5, -7.0, 10.5)
        );

        assert_approx_eq!(
            Vector::new(1.0, -2.0, 3.0) * 0.5,
            Vector::new(0.5, -1.0, 1.5)
        );

        assert_approx_eq!(
            -1.5 * Vector::new(-2.6, 0.0, 1.2),
            Vector::new(3.9, 0.0, -1.8)
        );

        let mut v = Vector::new(1.0, 2.5, 3.1);
        v *= 2.5;

        assert_approx_eq!(v, Vector::new(2.5, 6.25, 7.75));
    }

    #[test]
    fn dividing_a_vector_by_a_scaler() {
        assert_approx_eq!(
            Vector::new(1.0, -2.0, 3.0) / 2.0,
            Vector::new(0.5, -1.0, 1.5)
        );

        let mut v = Vector::new(-0.0, 2.9, 0.6);
        v /= 0.2;

        assert_approx_eq!(v, Vector::new(0.0, 14.5, 3.0));
    }

    #[test]
    fn negating_a_vector() {
        assert_approx_eq!(
            -Vector::new(1.0, -2.0, 3.0),
            Vector::new(-1.0, 2.0, -3.0)
        );
    }

    #[test]
    fn comparing_vectors() {
        let v1 = Vector::new(0.0, -1.0, 2.5);
        let v2 = Vector::new(-0.0, -1.0, 2.5);
        let v3 = Vector::new(0.000_06, -1.0, 2.5);

        assert_approx_eq!(v1, v2);

        assert_approx_ne!(v1, v3);
    }
}
