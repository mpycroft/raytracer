use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use float_cmp::{ApproxEq, F64Margin};

/// A Colour represents an RGB colour in the image, values generally range from
/// 0.0..1.0 but can go outside this range before final processing.
#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }

    pub fn purple() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }

    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        self.red -= rhs.red;
        self.green -= rhs.green;
        self.blue -= rhs.blue;
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.red * rhs, self.green * rhs, self.blue * rhs)
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
        Self::Output::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

impl MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

impl ApproxEq for Colour {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.red.approx_eq(other.red, margin)
            && self.green.approx_eq(other.green, margin)
            && self.blue.approx_eq(other.blue, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::{assert_approx_eq, assert_approx_ne};

    #[test]
    fn creating_a_colour() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_approx_eq!(c.red, -0.5);
        assert_approx_eq!(c.green, 0.4);
        assert_approx_eq!(c.blue, 1.7);

        assert_approx_eq!(Colour::black(), Colour::new(0.0, 0.0, 0.0));
        assert_approx_eq!(Colour::white(), Colour::new(1.0, 1.0, 1.0));
        assert_approx_eq!(Colour::red(), Colour::new(1.0, 0.0, 0.0));
        assert_approx_eq!(Colour::green(), Colour::new(0.0, 1.0, 0.0));
        assert_approx_eq!(Colour::blue(), Colour::new(0.0, 0.0, 1.0));
        assert_approx_eq!(Colour::yellow(), Colour::new(1.0, 1.0, 0.0));
        assert_approx_eq!(Colour::purple(), Colour::new(1.0, 0.0, 1.0));
        assert_approx_eq!(Colour::cyan(), Colour::new(0.0, 1.0, 1.0));
    }

    #[test]
    fn adding_two_colours() {
        assert_approx_eq!(
            Colour::new(0.9, 0.6, 0.75) + Colour::new(0.7, 0.1, 0.25),
            Colour::new(1.6, 0.7, 1.0)
        );

        let mut c = Colour::new(-0.5, 0.9, 1.2);
        c += Colour::new(0.5, 0.01, -0.3);

        assert_approx_eq!(c, Colour::new(0.0, 0.91, 0.9));
    }

    #[test]
    fn subtracting_two_colours() {
        assert_approx_eq!(
            Colour::new(0.9, 0.6, 0.75) - Colour::new(0.7, 0.1, 0.25),
            Colour::new(0.2, 0.5, 0.5)
        );

        let mut c = Colour::new(1.0, 1.0, 1.0);
        c -= Colour::new(1.0, 0.0, 0.5);

        assert_approx_eq!(c, Colour::new(0.0, 1.0, 0.5));
    }

    #[test]
    fn multiplying_a_colour_by_a_scaler() {
        assert_approx_eq!(
            Colour::new(0.2, 0.3, 0.4) * 2.0,
            Colour::new(0.4, 0.6, 0.8)
        );

        assert_approx_eq!(
            0.9 * Colour::new(1.0, 0.2, 1.4),
            Colour::new(0.90, 0.18, 1.26)
        );

        let mut c = Colour::new(0.5, -1.2, 2.44);
        c *= -0.3;

        assert_approx_eq!(c, Colour::new(-0.15, 0.36, -0.732));
    }

    #[test]
    fn multiplying_two_colours() {
        assert_approx_eq!(
            Colour::new(1.0, 0.2, 0.4) * Colour::new(0.9, 1.0, 0.1),
            Colour::new(0.9, 0.2, 0.04)
        );

        let mut c = Colour::new(-1.0, 0.7, 1.2);
        c *= Colour::new(0.5, 1.0, -0.6);

        assert_approx_eq!(c, Colour::new(-0.5, 0.7, -0.72));
    }

    #[test]
    fn comparing_colours() {
        let c1 = Colour::new(0.1, 0.7, 0.9);
        let c2 = Colour::new(0.1, 0.7, 0.9);
        let c3 = Colour::new(0.1, 0.7, 0.902);

        assert_approx_eq!(c1, c2);

        assert_approx_ne!(c1, c3);
    }
}
