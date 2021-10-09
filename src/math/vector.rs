use super::float::{FLOAT_EPSILON, FLOAT_ULPS};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// A Vector is a representation of a geometric vector, pointing in a given
/// direction and with a magnitude.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl AbsDiffEq for Vector {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl RelativeEq for Vector {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.x.relative_eq(&other.x, epsilon, max_relative)
            && self.y.relative_eq(&other.y, epsilon, max_relative)
            && self.z.relative_eq(&other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Vector {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        self.x.ulps_eq(&other.x, epsilon, max_ulps)
            && self.y.ulps_eq(&other.y, epsilon, max_ulps)
            && self.z.ulps_eq(&other.z, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let v = Vector::new(4.3, -4.2, 3.1);

        assert_float_relative_eq!(v.x, 4.3);
        assert_float_relative_eq!(v.y, -4.2);
        assert_float_relative_eq!(v.z, 3.1);
    }

    #[test]
    fn approx() {
        let v1 = Vector::new(0.004, 126.610_1, 9.61);
        let v2 = Vector::new(0.004, 126.610_1, 9.61);
        let v3 = Vector::new(0.004_1, 126.610_1, 9.22);

        assert_abs_diff_eq!(v1, v2);
        assert_abs_diff_ne!(v1, v3);

        assert_relative_eq!(v1, v2);
        assert_relative_ne!(v1, v3);

        assert_ulps_eq!(v1, v2);
        assert_ulps_ne!(v1, v3);
    }
}
