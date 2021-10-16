/// Arbitrary constant to use for absolute and relative floating point
/// comparisons.
pub const FLOAT_EPSILON: f64 = 0.000_001;

/// Arbitrary number of ulps to use when comparing floating point values.
pub const FLOAT_ULPS: u32 = 6;

/// Add the approx AbsDiffEq, RelativeEq and UlpsEq traits to a struct. This
/// handles the very simple of cases of a series of struct members that are
/// tested in order.
macro_rules! add_approx_traits {
    ($type:ty { $init:ident $(, $var:ident)* }) => {
        impl approx::AbsDiffEq for $type {
            type Epsilon = f64;

            fn default_epsilon() -> Self::Epsilon {
                crate::math::approx::FLOAT_EPSILON
            }

            fn abs_diff_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
            ) -> bool {
                self.$init.abs_diff_eq(&other.$init, epsilon)
                    $(&& self.$var.abs_diff_eq(&other.$var, epsilon))*
            }
        }

        impl approx::RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                crate::math::approx::FLOAT_EPSILON
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.$init.relative_eq(&other.$init, epsilon, max_relative)
                    $(&& self.$var.relative_eq(
                        &other.$var,
                        epsilon,
                        max_relative
                    ))*
            }
        }

        impl approx::UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                crate::math::approx::FLOAT_ULPS
            }

            fn ulps_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_ulps: u32,
            ) -> bool {
                self.$init.ulps_eq(&other.$init, epsilon, max_ulps)
                    $(&& self.$var.ulps_eq(&other.$var, epsilon, max_ulps))*
            }
        }
    };
}

/// Compare if two floating point values are equal.
macro_rules! float_relative_eq {
    ($lhs:expr, $rhs:expr) => {
        approx::relative_eq!(
            $lhs,
            $rhs,
            epsilon = crate::math::approx::FLOAT_EPSILON,
            max_relative = crate::math::approx::FLOAT_EPSILON
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
            epsilon = crate::math::approx::FLOAT_EPSILON,
            max_relative = crate::math::approx::FLOAT_EPSILON
        );
    };
}
