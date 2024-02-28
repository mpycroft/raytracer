use std::ops::Mul;

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use exmex::eval_str;
use paste::paste;
use serde::{de::Error, Deserialize, Deserializer};
use serde_yaml::Value;

use super::float::impl_approx_eq;

/// An `Angle` represents a geometric angle, it is simply a wrapper around a
/// value in radians but by using it rather than raw f64's we get type safety
/// and can more easily mix radians and degrees.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
#[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg)]
pub struct Angle(pub f64);

/// This macro helps to add standard trigonometric functions (sin, cos, etc.) to
/// `Angle`.
macro_rules! add_trigonometric_fns {
    ($fn:ident) => {
        #[must_use]
        pub fn $fn(&self) -> f64 {
            self.0.$fn()
        }

        paste! {
            #[must_use]
            pub fn [<a $fn>](ratio: f64) -> Self {
                Self(ratio.[<a $fn>]())
            }
        }
    };
}

impl Angle {
    #[must_use]
    pub fn from_degrees(degrees: f64) -> Self {
        Self(degrees.to_radians())
    }

    #[must_use]
    pub fn to_degrees(&self) -> f64 {
        self.0.to_degrees()
    }

    add_trigonometric_fns!(sin);
    add_trigonometric_fns!(cos);
    add_trigonometric_fns!(tan);

    #[must_use]
    pub fn sin_cos(&self) -> (f64, f64) {
        self.0.sin_cos()
    }

    #[must_use]
    pub fn atan2(y: f64, x: f64) -> Self {
        Self(y.atan2(x))
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        rhs * self
    }
}

impl_approx_eq!(Angle { newtype });

impl<'de> Deserialize<'de> for Angle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum Angle {
            Degrees { degrees: Value },
            Radians(Value),
        }

        let parse = |value| match value {
            Value::Number(number) => {
                number.as_f64().map_or_else(|| unreachable!(), Ok)
            }
            Value::String(string) => {
                eval_str::<f64>(&string).map_err(Error::custom)
            }
            _ => Err(Error::custom(format!(
                "Unable to parse '{value:?}' as a float"
            ))),
        };

        match Angle::deserialize(deserializer)? {
            Angle::Radians(radians) => Ok(Self(parse(radians)?)),
            Angle::Degrees { degrees } => {
                Ok(Self::from_degrees(parse(degrees)?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    use serde_yaml::from_str;

    use super::*;
    use crate::math::float::*;

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
    fn trigonometric_functions_pass_through() {
        assert_approx_eq!(Angle(PI).cos(), PI.cos());
        assert_approx_eq!(Angle(FRAC_PI_2).sin(), FRAC_PI_2.sin());
        assert_approx_eq!(Angle(FRAC_PI_3).tan(), FRAC_PI_3.tan());

        assert_approx_eq!(Angle::acos(Angle(PI).cos()), Angle(PI.cos().acos()));
        assert_approx_eq!(
            Angle::asin(Angle(FRAC_PI_2).sin()),
            Angle(FRAC_PI_2.sin().asin())
        );
        assert_approx_eq!(
            Angle::atan(Angle(FRAC_PI_3).tan()),
            Angle(FRAC_PI_3.tan().atan())
        );

        let (s1, c1) = Angle::from_degrees(163.5).sin_cos();
        let (s2, c2) = 163.5f64.to_radians().sin_cos();
        assert_approx_eq!(s1, s2);
        assert_approx_eq!(c1, c2);

        assert_approx_eq!(
            Angle::atan2(FRAC_PI_4, FRAC_PI_6).0,
            FRAC_PI_4.atan2(FRAC_PI_6)
        );
    }

    #[test]
    fn comparing_angles() {
        let a1 = Angle(FRAC_PI_3);
        let a2 = Angle::from_degrees(60.0);
        let a3 = Angle(FRAC_PI_3 + 0.000_000_1);

        assert_approx_eq!(a1, a2);

        assert_approx_ne!(a1, a3);
    }

    #[test]
    fn deserialize_angle() {
        let a: Angle = from_str("0.5").unwrap();

        assert_approx_eq!(a, Angle(0.5));

        let a: Angle = from_str("-1").unwrap();

        assert_approx_eq!(a, Angle(-1.0));

        let a: Angle = from_str("degrees: 45.0").unwrap();

        assert_approx_eq!(a, Angle::from_degrees(45.0));

        let a: Angle = from_str("-PI / 3").unwrap();

        assert_approx_eq!(a, Angle(-FRAC_PI_3));

        let a: Angle = from_str("degrees: 3 * 10.5").unwrap();

        assert_approx_eq!(a, Angle::from_degrees(31.5));

        assert_eq!(
            from_str::<Angle>("true").unwrap_err().to_string(),
            "Unable to parse 'Bool(true)' as a float"
        );
    }
}
