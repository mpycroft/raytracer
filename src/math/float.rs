/// Arbitrary constant to use for absolute and relative floating point
/// comparisons.
pub const FLOAT_EPSILON: f64 = 0.000_001;

/// Arbitrary number of ulps to use when comparing floating point values.
pub const FLOAT_ULPS: u32 = 6;

/// Compare if two floating point values are equal.
macro_rules! float_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::relative_eq!(
            $lhs,
            $rhs,
            epsilon = crate::math::float::FLOAT_EPSILON,
            max_relative = crate::math::float::FLOAT_EPSILON
        )
    };
}

/// Assert that two floating point values are equal.
#[cfg(test)]
macro_rules! assert_float_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::assert_relative_eq!(
            $lhs,
            $rhs,
            epsilon = crate::math::float::FLOAT_EPSILON,
            max_relative = crate::math::float::FLOAT_EPSILON
        );
    };
}
