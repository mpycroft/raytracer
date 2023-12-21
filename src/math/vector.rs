use std::ops::Mul;

use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

use super::float::{approx_eq, impl_approx_eq};

/// A Vector is a representation of a geometric vector, pointing in a given
/// direction and with a magnitude.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Constructor, Neg)]
#[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    #[must_use]
    pub fn x_axis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    #[must_use]
    pub fn y_axis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    #[must_use]
    pub fn z_axis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    #[must_use]
    pub fn magnitude(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    #[must_use]
    pub fn normalise(&self) -> Self {
        let magnitude = self.magnitude();

        if approx_eq!(magnitude, 0.0) {
            return Self::new(0.0, 0.0, 0.0);
        }

        *self / magnitude
    }

    #[must_use]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[must_use]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[must_use]
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl_approx_eq!(Vector { x, y, z });

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_vector() {
        let p = Vector::new(2.8, 4.0, -0.7);

        assert_approx_eq!(p.x, 2.8);
        assert_approx_eq!(p.y, 4.0);
        assert_approx_eq!(p.z, -0.7);

        assert_approx_eq!(Vector::x_axis(), Vector::new(1.0, 0.0, 0.0));
        assert_approx_eq!(Vector::y_axis(), Vector::new(0.0, 1.0, 0.0));
        assert_approx_eq!(Vector::z_axis(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_magnitude_of_a_vector() {
        assert_approx_eq!(Vector::x_axis().magnitude(), 1.0);
        assert_approx_eq!(Vector::y_axis().magnitude(), 1.0);
        assert_approx_eq!(Vector::z_axis().magnitude(), 1.0);

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
            Vector::x_axis()
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

        assert_approx_eq!(v1.dot(&v2), d);
        assert_approx_eq!(v2.dot(&v1), d);
    }

    #[test]
    fn computing_the_cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_approx_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_approx_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector() {
        assert_approx_eq!(
            Vector::new(1.0, -1.0, 0.0).reflect(&Vector::y_axis()),
            Vector::new(1.0, 1.0, 0.0)
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            -Vector::y_axis().reflect(&Vector::new(
                sqrt_2_div_2,
                sqrt_2_div_2,
                0.0
            )),
            Vector::x_axis()
        );
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
