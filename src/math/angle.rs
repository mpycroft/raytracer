use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use paste::paste;

use crate::util::float::Float;

/// An Angle represents a geometric angle, it is simply a wrapper around a value
/// in radians but by using it rather than raw f64's we get some safety between
/// radians and degrees and usage become more explicit.
#[rustfmt::skip] // Don't merge these derives or we get a huge vertical list
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[derive(Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign)]
pub struct Angle<T: Float> {
    radians: T,
}

/// Add both trigonometric functions (sin, etc.) that operate on an Angle and
/// return a raw f64 ratio and inverse (asin, etc.) functions that work on a
/// ratio and return an Angle.
macro_rules! add_trigonometric_fns {
    (@impl $func:ident, $inv_func:ident) => {
        pub fn $func(&self) -> T {
            self.radians.$func()
        }

        pub fn $inv_func(ratio: T) -> Angle<T> {
            Angle::from_radians(ratio.$inv_func())
        }
    };
    ($($op:ident),+) => {
        $(
            paste! {
                add_trigonometric_fns!(@impl $op, [<a $op>]);
            }
        )+
    };
}

impl<T: Float> Angle<T> {
    pub fn from_radians(radians: T) -> Self {
        Angle { radians }
    }

    pub fn from_degrees(degrees: T) -> Self {
        Angle { radians: degrees.to_radians() }
    }

    pub fn to_radians(self) -> T {
        self.radians
    }

    pub fn to_degrees(self) -> T {
        self.radians.to_degrees()
    }

    add_trigonometric_fns!(sin, cos, tan);

    pub fn sin_cos(&self) -> (T, T) {
        self.radians.sin_cos()
    }

    pub fn atan2(y: T, x: T) -> Angle<T> {
        Angle::from_radians(y.atan2(x))
    }
}

add_left_mul_scaler!(Angle<T>);

add_approx_traits!(Angle<T> { radians });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    use approx::*;

    use super::*;

    #[test]
    fn creating_an_angle_from_radians() {
        assert_float_relative_eq!(
            Angle::from_radians(FRAC_PI_2).radians,
            FRAC_PI_2
        );
    }

    #[test]
    fn creating_an_angle_from_degrees() {
        assert_float_relative_eq!(Angle::from_degrees(90.0).radians, FRAC_PI_2);
    }

    #[test]
    fn accessing_an_angle_as_radians() {
        assert_float_relative_eq!(
            Angle::from_radians(FRAC_PI_2).to_radians(),
            FRAC_PI_2
        );
    }

    #[test]
    fn accessing_an_angle_as_degrees() {
        assert_float_relative_eq!(Angle::from_degrees(25.0).to_degrees(), 25.0);
    }

    #[test]
    fn standard_trigonometric_functions_are_passed_through() {
        assert_float_relative_eq!(
            Angle::from_radians(0.851).sin(),
            0.851f64.sin()
        );

        assert_float_relative_eq!(
            Angle::from_radians(2.41).cos(),
            2.41f64.cos()
        );

        assert_float_relative_eq!(
            Angle::from_radians(FRAC_PI_3).tan(),
            FRAC_PI_3.tan()
        );

        let (s1, c1) = Angle::from_degrees(43.1).sin_cos();
        let (s2, c2) = 43.1f64.to_radians().sin_cos();

        assert_float_relative_eq!(s1, s2);
        assert_float_relative_eq!(c1, c2);
    }

    #[test]
    fn inverse_trigonometric_functions_are_passed_through() {
        assert_relative_eq!(
            Angle::asin(0.3),
            Angle::from_radians(0.3f64.asin())
        );

        assert_relative_eq!(
            Angle::acos(0.915),
            Angle::from_radians(0.915f64.acos())
        );

        assert_relative_eq!(
            Angle::atan(0.72),
            Angle::from_radians(0.72f64.atan())
        );

        assert_relative_eq!(
            Angle::atan2(2.0, -1.5),
            Angle::from_radians(2.0f64.atan2(-1.5))
        );
    }

    #[test]
    fn adding_two_angles() {
        assert_relative_eq!(
            Angle::from_radians(FRAC_PI_2) + Angle::from_degrees(90.0),
            Angle::from_radians(PI)
        );

        let mut a = Angle::from_degrees(20.3);
        a += Angle::from_degrees(0.7);

        assert_relative_eq!(a, Angle::from_degrees(21.0));
    }

    #[test]
    fn dividing_an_angle_by_a_scaler() {
        assert_relative_eq!(
            Angle::from_radians(PI) / 2.0,
            Angle::from_radians(FRAC_PI_2)
        );

        let mut a = Angle::from_radians(2.0 * PI);
        a /= 3.0;

        assert_relative_eq!(a, Angle::from_degrees(120.0));
    }

    #[test]
    fn multiplying_an_angle_by_a_scaler() {
        assert_relative_eq!(
            Angle::from_radians(FRAC_PI_3) * 3.0,
            Angle::from_radians(PI)
        );

        assert_relative_eq!(
            1.5 * Angle::from_radians(PI),
            Angle::from_degrees(270.0)
        );

        let mut a = Angle::from_radians(FRAC_PI_3);
        a *= 3.0;

        assert_relative_eq!(a, Angle::from_radians(PI));
    }

    #[test]
    fn negating_an_angle() {
        assert_relative_eq!(
            -Angle::from_degrees(20.3),
            Angle::from_degrees(-20.3)
        );
    }

    #[test]
    fn subtracting_two_angles() {
        assert_relative_eq!(
            Angle::from_degrees(90.0) - Angle::from_radians(FRAC_PI_4),
            Angle::from_degrees(45.0)
        );

        let mut a = Angle::from_radians(FRAC_PI_2);
        a -= Angle::from_radians(FRAC_PI_3);

        assert_relative_eq!(a, Angle::from_radians(FRAC_PI_6));
    }

    #[test]
    fn angles_are_approximately_equal() {
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
