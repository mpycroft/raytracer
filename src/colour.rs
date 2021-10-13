use crate::math::float::{FLOAT_EPSILON, FLOAT_ULPS};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

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
