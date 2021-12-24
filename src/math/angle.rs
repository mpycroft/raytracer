/// An Angle represents a geometric angle, it is simply a wrapper around a value
/// in radians but by using it rather than raw f64's we get some safety between
/// radians and degrees and usage become more explicit.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

add_approx_traits!(Angle { radians });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

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
