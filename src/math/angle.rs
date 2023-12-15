use std::ops::Mul;

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use float_cmp::{ApproxEq, F64Margin};

/// An `Angle` represents a geometric angle, it is simply a wrapper around a
/// value in radians but by using it rather than raw f64's we get type safety
/// and can more easily mix radians and degrees.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
#[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg)]
pub struct Angle(pub f64);

impl Angle {
    #[must_use]
    pub fn from_degrees(degrees: f64) -> Self {
        Self(degrees.to_radians())
    }

    #[must_use]
    pub fn to_degrees(&self) -> f64 {
        self.0.to_degrees()
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        rhs * self
    }
}

impl ApproxEq for Angle {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.0.approx_eq(other.0, margin)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    use super::*;
    use crate::math::float::{assert_approx_eq, assert_approx_ne};

    #[test]
    fn creating_angles() {
        let a = Angle(PI);

        assert_approx_eq!(a.0, PI);
        assert_approx_eq!(a.to_degrees(), 180.0);

        let a = Angle::from_degrees(90.0);

        assert_approx_eq!(a.0, FRAC_PI_2);
        assert_approx_eq!(a.to_degrees(), 90.0);
    }

    #[test]
    fn adding_two_angles() {
        assert_approx_eq!(
            Angle(FRAC_PI_2) + Angle::from_degrees(90.0),
            Angle(PI)
        );

        let mut a = Angle::from_degrees(26.4);
        a += Angle::from_degrees(3.6);

        assert_approx_eq!(a.0, FRAC_PI_6);
    }

    #[test]
    fn subtracting_two_angles() {
        assert_approx_eq!(Angle(PI) - Angle(FRAC_PI_2), Angle(FRAC_PI_2));

        let mut a = Angle(FRAC_PI_2);
        a -= Angle::from_degrees(15.0);

        assert_approx_eq!(a, Angle::from_degrees(75.0));
    }

    #[test]
    fn multiplying_an_angle_by_a_scaler() {
        assert_approx_eq!(Angle::from_degrees(45.0) * 2.0, Angle(FRAC_PI_2));
        assert_approx_eq!(
            0.5 * Angle::from_degrees(20.2),
            Angle::from_degrees(10.1)
        );

        let mut a = Angle(FRAC_PI_6);
        a *= 3.11;

        assert_approx_eq!(a, Angle::from_degrees(93.3));
    }

    #[test]
    fn dividing_an_angle_by_a_scaler() {
        assert_approx_eq!(Angle(PI) / 4.0, Angle(FRAC_PI_4));

        let mut a = Angle::from_degrees(215.8);
        a /= 2.156;

        assert_approx_eq!(
            a,
            Angle::from_degrees(100.092_764),
            epsilon = 0.000_001
        );
    }

    #[test]
    fn negating_an_angle() {
        assert_approx_eq!(-Angle(PI), Angle::from_degrees(-180.0));
    }

    #[test]
    fn comparing_angles() {
        let a1 = Angle(FRAC_PI_3);
        let a2 = Angle::from_degrees(60.0);
        let a3 = Angle(FRAC_PI_3 + 0.000_000_1);

        assert_approx_eq!(a1, a2);

        assert_approx_ne!(a1, a3);
    }
}
