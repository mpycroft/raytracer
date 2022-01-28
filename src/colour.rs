use std::ops::{Mul, MulAssign};

use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};
use num_traits::{clamp, ToPrimitive};

use crate::util::float::Float;

/// A Colour represents an RGB colour in the image, values generally range from
/// 0.0..1.0 but can go outside this range before final processing.
#[rustfmt::skip] // Don't merge these derives or we get a huge vertical list
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Constructor)]
#[derive(Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign)]
#[mul(forward)]
#[mul_assign(forward)]
pub struct Colour<T: Float> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Float> Colour<T> {
    pub fn black() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn white() -> Self {
        Self::new(T::one(), T::one(), T::one())
    }

    pub fn red() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    pub fn green() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    pub fn blue() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        let convert = |c: T| {
            ToPrimitive::to_u8(
                &(clamp(c, T::zero(), T::one()) * T::from(255.0f64).unwrap()),
            )
            .unwrap()
        };

        (convert(self.r), convert(self.g), convert(self.b))
    }
}

impl<T: Float> Mul<T> for Colour<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl<T: Float> MulAssign<T> for Colour<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

add_left_mul_scaler!(Colour<T>);

add_approx_traits!(Colour<T> { r, g, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_colour() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_float_relative_eq!(c.r, -0.5);
        assert_float_relative_eq!(c.g, 0.4);
        assert_float_relative_eq!(c.b, 1.7);
    }

    #[test]
    fn creating_a_named_colour() {
        assert_relative_eq!(Colour::black(), Colour::new(0.0, 0.0, 0.0));

        assert_relative_eq!(Colour::white(), Colour::new(1.0, 1.0, 1.0));

        assert_relative_eq!(Colour::red(), Colour::new(1.0, 0.0, 0.0));

        assert_relative_eq!(Colour::green(), Colour::new(0.0, 1.0, 0.0));

        assert_relative_eq!(Colour::blue(), Colour::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn converting_a_colour_to_an_rgb_tuple() {
        assert_eq!(Colour::new(0.0, 0.5, 1.0).to_rgb(), (0, 127, 255));
        assert_eq!(Colour::new(0.4, 1.6, -2.3).to_rgb(), (102, 255, 0));
    }

    #[test]
    fn adding_two_colours() {
        assert_relative_eq!(
            Colour::new(0.9, 0.6, 0.75) + Colour::new(0.7, 0.1, 0.25),
            Colour::new(1.6, 0.7, 1.0)
        );

        let mut c = Colour::new(0.0, -1.3, 5.9);
        c += Colour::new(0.6, 0.0, 2.1);

        assert_relative_eq!(c, Colour::new(0.6, -1.3, 8.0));
    }

    #[test]
    fn subtracting_two_colours() {
        assert_relative_eq!(
            Colour::new(0.9, 0.6, 0.75) - Colour::new(0.7, 0.1, 0.25),
            Colour::new(0.2, 0.5, 0.5)
        );

        let mut c = Colour::new(0.8, 0.1, 5.2);
        c -= Colour::new(0.2, 1.0, -0.2);

        assert_relative_eq!(c, Colour::new(0.6, -0.9, 5.4));
    }

    #[test]
    fn multiplying_a_colour_by_a_scaler() {
        assert_relative_eq!(
            Colour::new(0.2, 0.3, 0.4) * 2.0,
            Colour::new(0.4, 0.6, 0.8)
        );

        assert_relative_eq!(
            0.9 * Colour::new(-1.5, 0.0, 2.3),
            Colour::new(-1.35, 0.0, 2.07)
        );

        let mut c = Colour::new(1.0, 1.5, 0.11);
        c *= -2.35;

        assert_relative_eq!(c, Colour::new(-2.35, -3.525, -0.258_5));
    }

    #[test]
    fn multiplying_two_colours() {
        assert_relative_eq!(
            Colour::new(1.0, 0.2, 0.4) * Colour::new(0.9, 1.0, 0.1),
            Colour::new(0.9, 0.2, 0.04)
        );

        let mut c = Colour::new(2.74, -1.0, 1.5);
        c *= Colour::new(1.0, 0.25, 0.0);

        assert_relative_eq!(c, Colour::new(2.74, -0.25, 0.0));
    }

    #[test]
    fn dividing_a_colour_by_a_scaler() {
        assert_relative_eq!(
            Colour::new(1.0, 2.0, 3.0) / 2.0,
            Colour::new(0.5, 1.0, 1.5)
        );

        let mut c = Colour::new(0.5, 0.8, 0.3);
        c /= 1.1;

        assert_relative_eq!(c, Colour::new(0.454_546, 0.727_272, 0.272_728));
    }

    #[test]
    fn colours_are_approximately_equal() {
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
