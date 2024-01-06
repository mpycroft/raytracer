use std::ops::{Mul, MulAssign};

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};
use derive_new::new;

use crate::math::float::impl_approx_eq;

/// A Colour represents an RGB colour in the image, values generally range from
/// 0.0..1.0 but can go outside this range before final processing.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, new)]
#[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    #[must_use]
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[must_use]
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    #[must_use]
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    #[must_use]
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    #[must_use]
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    #[must_use]
    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }

    #[must_use]
    pub fn purple() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }

    #[must_use]
    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }

    #[must_use]
    pub fn to_u8(&self) -> [u8; 3] {
        // There is no nice way to do a conversion from f64 to a u8 so we are
        // forced to use `as` and the clamp and multiplication guarantee that we
        // are within the value of a u8.
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let convert = |c: f64| (c.clamp(0.0, 1.0) * 255.0).round() as u8;

        [convert(self.red), convert(self.green), convert(self.blue)]
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

impl MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

impl_approx_eq!(Colour { red, green, blue });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

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
    fn generating_u8_values_from_a_colour() {
        assert_eq!(Colour::black().to_u8(), [0, 0, 0]);
        assert_eq!(Colour::white().to_u8(), [255, 255, 255]);

        assert_eq!(Colour::new(-0.3, 0.5, 1.0).to_u8(), [0, 128, 255]);
        assert_eq!(Colour::new(0.2, 0.51, 0.9).to_u8(), [51, 130, 230]);
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
    fn dividing_a_colour_by_a_scaler() {
        assert_approx_eq!(
            Colour::new(1.2, 0.6, 0.3) / 2.0,
            Colour::new(0.6, 0.3, 0.15)
        );

        let mut c = Colour::white();
        c /= 0.5;

        assert_approx_eq!(c, Colour::new(2.0, 2.0, 2.0));
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
