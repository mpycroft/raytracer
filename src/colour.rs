use crate::math::float::{FLOAT_EPSILON, FLOAT_ULPS};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// A Colour represents an RGB colour in the image, values generally range from
/// 0.0..1.0 but can go outside this range before final processing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl AbsDiffEq for Colour {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r.abs_diff_eq(&other.r, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

impl RelativeEq for Colour {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.r.relative_eq(&other.r, epsilon, max_relative)
            && self.g.relative_eq(&other.g, epsilon, max_relative)
            && self.b.relative_eq(&other.b, epsilon, max_relative)
    }
}

impl UlpsEq for Colour {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        self.r.ulps_eq(&other.r, epsilon, max_ulps)
            && self.g.ulps_eq(&other.g, epsilon, max_ulps)
            && self.b.ulps_eq(&other.b, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_float_relative_eq!(c.r, -0.5);
        assert_float_relative_eq!(c.g, 0.4);
        assert_float_relative_eq!(c.b, 1.7);
    }

    #[test]
    fn add() {
        assert_relative_eq!(
            Colour::new(0.9, 0.6, 0.75) + Colour::new(0.7, 0.1, 0.25),
            Colour::new(1.6, 0.7, 1.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut c = Colour::new(0.0, -1.3, 5.9);
        c += Colour::new(0.6, 0.0, 2.1);

        assert_relative_eq!(c, Colour::new(0.6, -1.3, 8.0));
    }

    #[test]
    fn mul() {
        assert_relative_eq!(
            Colour::new(0.2, 0.3, 0.4) * 2.0,
            Colour::new(0.4, 0.6, 0.8)
        );

        assert_relative_eq!(
            0.9 * Colour::new(-1.5, 0.0, 2.3),
            Colour::new(-1.35, 0.0, 2.07)
        );

        assert_relative_eq!(
            Colour::new(1.0, 0.2, 0.4) * Colour::new(0.9, 1.0, 0.1),
            Colour::new(0.9, 0.2, 0.04)
        );
    }

    #[test]
    fn mul_assign() {
        let mut c = Colour::new(1.0, 1.5, 0.11);
        c *= -2.35;

        assert_relative_eq!(c, Colour::new(-2.35, -3.525, -0.258_5));

        c *= Colour::new(1.0, 0.25, 0.0);

        assert_relative_eq!(c, Colour::new(-2.35, -0.881_25, 0.0));
    }

    #[test]
    fn sub() {
        assert_relative_eq!(
            Colour::new(0.9, 0.6, 0.75) - Colour::new(0.7, 0.1, 0.25),
            Colour::new(0.2, 0.5, 0.5)
        );
    }

    #[test]
    fn sub_assign() {
        let mut c = Colour::new(0.8, 0.1, 5.2);
        c -= Colour::new(0.2, 1.0, -0.2);

        assert_relative_eq!(c, Colour::new(0.6, -0.9, 5.4));
    }

    #[test]
    fn approx() {
        let c1 = Colour::new(-10.531, 0.851, 1.5681);
        let c2 = Colour::new(-10.531, 0.851, 1.5681);
        let c3 = Colour::new(-10.532, 0.851_05, 1.5681);

        assert_abs_diff_eq!(c1, c2);
        assert_abs_diff_ne!(c1, c3);

        assert_relative_eq!(c1, c2);
        assert_relative_ne!(c1, c3);

        assert_ulps_eq!(c1, c2);
        assert_ulps_ne!(c1, c3);
    }
}
