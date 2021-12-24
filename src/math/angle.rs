use std::ops::Mul;

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// An Angle represents a geometric angle, it is simply a wrapper around a value
/// in radians but by using it rather than raw f64's we get some safety between
/// radians and degrees and usage become more explicit.
#[rustfmt::skip] // Don't merge these derives or we get a huge vertical list
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[derive(Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub fn from_radians(radians: f64) -> Self {
        Angle { radians }
    }

    pub fn from_degrees(degrees: f64) -> Self {
        Angle { radians: degrees.to_radians() }
    }

    pub fn to_radians(self) -> f64 {
        self.radians
    }

    pub fn to_degrees(self) -> f64 {
        self.radians.to_degrees()
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        rhs * self
    }
}

add_approx_traits!(Angle { radians });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    use approx::*;

    use super::*;

    #[test]
    fn from_radians() {
        assert_float_relative_eq!(
            Angle::from_radians(FRAC_PI_2).radians,
            FRAC_PI_2
        );
    }

    #[test]
    fn from_degrees() {
        assert_float_relative_eq!(Angle::from_degrees(90.0).radians, FRAC_PI_2);
    }

    #[test]
    fn to_radians() {
        assert_float_relative_eq!(
            Angle::from_radians(FRAC_PI_2).to_radians(),
            FRAC_PI_2
        );
    }

    #[test]
    fn to_degrees() {
        assert_float_relative_eq!(Angle::from_degrees(25.0).to_degrees(), 25.0);
    }

    #[test]
    fn add() {
        assert_relative_eq!(
            Angle::from_radians(FRAC_PI_2) + Angle::from_degrees(90.0),
            Angle::from_radians(PI)
        );
    }

    #[test]
    fn add_assign() {
        let mut a = Angle::from_degrees(20.3);
        a += Angle::from_degrees(0.7);

        assert_relative_eq!(a, Angle::from_degrees(21.0));
    }

    #[test]
    fn div() {
        assert_relative_eq!(
            Angle::from_radians(PI) / 2.0,
            Angle::from_radians(FRAC_PI_2)
        );
    }

    #[test]
    fn div_assign() {
        let mut a = Angle::from_radians(2.0 * PI);
        a /= 3.0;

        assert_relative_eq!(a, Angle::from_degrees(120.0));
    }

    #[test]
    fn mul() {
        assert_relative_eq!(
            Angle::from_radians(FRAC_PI_3) * 3.0,
            Angle::from_radians(PI)
        );

        assert_relative_eq!(
            1.5 * Angle::from_radians(PI),
            Angle::from_degrees(270.0)
        );
    }

    #[test]
    fn mul_assign() {
        let mut a = Angle::from_radians(FRAC_PI_3);
        a *= 3.0;

        assert_relative_eq!(a, Angle::from_radians(PI));
    }

    #[test]
    fn neg() {
        assert_relative_eq!(
            -Angle::from_degrees(20.3),
            Angle::from_degrees(-20.3)
        );
    }

    #[test]
    fn sub() {
        assert_relative_eq!(
            Angle::from_degrees(90.0) - Angle::from_radians(FRAC_PI_4),
            Angle::from_degrees(45.0)
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = Angle::from_radians(FRAC_PI_2);
        a -= Angle::from_radians(FRAC_PI_3);

        assert_relative_eq!(a, Angle::from_radians(FRAC_PI_6));
    }

    #[test]
    fn approx() {
        let a1 = Angle::from_radians(FRAC_PI_3);
        let a2 = Angle::from_degrees(60.0);
        let a3 = Angle::from_radians(FRAC_PI_2);

        assert_abs_diff_eq!(a1, a2);
        assert_abs_diff_ne!(a1, a3);

        assert_relative_eq!(a1, a2);
        assert_relative_ne!(a1, a3);

        assert_ulps_eq!(a1, a2);
        assert_ulps_ne!(a1, a3);
    }
}
