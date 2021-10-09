/// Arbitrary constant to use for absolute and relative floating point
/// comparisons.
pub const FLOAT_EPSILON: f64 = 0.000_001;

/// Arbitrary number of ulps to use when comparing floating point values.
pub const FLOAT_ULPS: u32 = 6;

/// Compare two floating point values, this just saves having to pass our
/// epsilon every time we call the approx macro.
#[cfg(test)]
macro_rules! assert_float_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        assert_relative_eq!(
            $lhs,
            $rhs,
            epsilon = crate::math::float::FLOAT_EPSILON,
            max_relative = crate::math::float::FLOAT_EPSILON
        );
    };
}
