use float_cmp::{ApproxEq, F64Margin};

/// An `Angle` represents a geometric angle, it is simply a wrapper around a
/// value in radians but by using it rather than raw f64's we get type safety
/// and can more easily mix radians and degrees.
#[derive(Clone, Copy, Debug)]
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

impl ApproxEq for Angle {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.0.approx_eq(other.0, margin)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

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
    fn comparing_angles() {
        let a1 = Angle(FRAC_PI_3);
        let a2 = Angle::from_degrees(60.0);
        let a3 = Angle(FRAC_PI_3 + 0.000_000_1);

        assert_approx_eq!(a1, a2);

        assert_approx_ne!(a1, a3);
    }
}
