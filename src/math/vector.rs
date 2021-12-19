use std::ops::Mul;

use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

/// A Vector is a representation of a geometric vector, pointing in a given
/// direction and with a magnitude.
#[rustfmt::skip] // Don't merge these derives or we get a huge vertical list
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
#[derive(Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn normalise(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector::new(0.0, 0.0, 0.0);
        }

        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(*normal)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

add_approx_traits!(Vector { x, y, z });

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use approx::*;

    use super::*;

    #[test]
    fn new() {
        let v = Vector::new(4.3, -4.2, 3.1);

        assert_float_relative_eq!(v.x, 4.3);
        assert_float_relative_eq!(v.y, -4.2);
        assert_float_relative_eq!(v.z, 3.1);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_relative_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_relative_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn dot() {
        assert_float_relative_eq!(
            Vector::new(1.0, 2.0, 3.0).dot(Vector::new(2.0, 3.0, 4.0)),
            20.0
        );
    }

    #[test]
    fn magnitude() {
        assert_float_relative_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_float_relative_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_float_relative_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);

        assert_float_relative_eq!(
            Vector::new(1.0, 2.0, 3.0).magnitude(),
            3.741_657
        );
        assert_float_relative_eq!(
            Vector::new(-1.0, -2.0, -3.0).magnitude(),
            3.741_657
        );
    }

    #[test]
    fn normalise() {
        assert_relative_eq!(
            Vector::new(4.0, 0.0, 0.0).normalise(),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_relative_eq!(
            Vector::new(1.0, 2.0, 3.0).normalise(),
            Vector::new(0.267_261, 0.534_522, 0.801_784)
        );

        assert_float_relative_eq!(
            Vector::new(1.0, 2.0, 3.0).normalise().magnitude(),
            1.0
        );

        assert_float_relative_eq!(
            Vector::new(0.0, 0.0, 0.0).normalise().magnitude(),
            0.0
        );
    }

    #[test]
    fn reflect() {
        assert_relative_eq!(
            Vector::new(1.0, -1.0, 0.0).reflect(&Vector::new(0.0, 1.0, 0.0)),
            Vector::new(1.0, 1.0, 0.0)
        );

        assert_relative_eq!(
            Vector::new(0.0, -1.0, 0.0).reflect(&Vector::new(
                FRAC_1_SQRT_2,
                FRAC_1_SQRT_2,
                0.0
            )),
            Vector::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn add() {
        assert_relative_eq!(
            Vector::new(1.3, 2.6, 0.9) + Vector::new(0.0, -1.3, 3.1),
            Vector::new(1.3, 1.3, 4.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut v = Vector::new(2.5, 0.3, 1.5);
        v += Vector::new(1.3, 1.6, 0.0);

        assert_relative_eq!(v, Vector::new(3.8, 1.9, 1.5));
    }

    #[test]
    fn div() {
        assert_relative_eq!(
            Vector::new(1.0, -2.0, 3.0) / 2.0,
            Vector::new(0.5, -1.0, 1.5)
        );
    }

    #[test]
    fn div_assign() {
        let mut v = Vector::new(2.3, 0.0, 1.5);
        v /= 0.8;

        assert_relative_eq!(v, Vector::new(2.875, 0.0, 1.875));
    }

    #[test]
    fn mul() {
        assert_relative_eq!(
            Vector::new(1.0, -2.0, 3.0) * 3.5,
            Vector::new(3.5, -7.0, 10.5)
        );

        assert_relative_eq!(
            0.5 * Vector::new(1.0, -2.0, 3.0),
            Vector::new(0.5, -1.0, 1.5)
        );
    }

    #[test]
    fn mul_assign() {
        let mut v = Vector::new(0.0, -2.3, 4.1);
        v *= 1.3;

        assert_relative_eq!(v, Vector::new(0.0, -2.99, 5.33));
    }

    #[test]
    fn neg() {
        assert_relative_eq!(
            -Vector::new(1.0, -2.0, 3.0),
            Vector::new(-1.0, 2.0, -3.0)
        );
    }

    #[test]
    fn sub() {
        assert_relative_eq!(
            Vector::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0),
            Vector::new(-2.0, -4.0, -6.0)
        );

        assert_relative_eq!(
            Vector::new(0.0, 0.0, 0.0) - Vector::new(1.0, -2.0, 3.0),
            Vector::new(-1.0, 2.0, -3.0)
        );
    }

    #[test]
    fn sub_assign() {
        let mut v = Vector::new(0.0, 1.5, 0.9);
        v -= Vector::new(1.3, 0.9, 0.1);

        assert_relative_eq!(v, Vector::new(-1.3, 0.6, 0.8));
    }

    #[test]
    fn approx() {
        let v1 = Vector::new(0.004, 126.610_1, 9.61);
        let v2 = Vector::new(0.004, 126.610_1, 9.61);
        let v3 = Vector::new(0.004_1, 126.610_1, 9.22);

        assert_abs_diff_eq!(v1, v2);
        assert_abs_diff_ne!(v1, v3);

        assert_relative_eq!(v1, v2);
        assert_relative_ne!(v1, v3);

        assert_ulps_eq!(v1, v2);
        assert_ulps_ne!(v1, v3);
    }
}
