use crate::util::float::Float;

/// Linearly interpolate t distance between a and b.
pub fn lerp<T: Float>(a: T, b: T, t: T) -> T {
    a + t * (b - a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_between_two_values() {
        assert_float_relative_eq!(lerp(1.0, 2.0, 0.5), 1.5);
        assert_float_relative_eq!(lerp(0.0, 1.0, 0.32), 0.32);
        assert_float_relative_eq!(lerp(-5.0, 10.0, 0.0), -5.0);
        assert_float_relative_eq!(lerp(-2.0, 4.0, 1.0), 4.0);
    }

    #[test]
    fn lerp_when_first_value_is_higher() {
        assert_float_relative_eq!(lerp(3.4, 2.1, 0.3), 3.01);
        assert_float_relative_eq!(lerp(4.0, 0.0, 0.0), 4.0);
        assert_float_relative_eq!(lerp(4.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn lerp_when_t_outside_zero_to_one() {
        assert_float_relative_eq!(lerp(1.0, 2.0, 1.5), 2.5);
        assert_float_relative_eq!(lerp(1.0, 2.0, -1.8), -0.8);
    }
}
