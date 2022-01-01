/// Arbitrary constant to use for absolute and relative floating point
/// comparisons.
pub const FLOAT_EPSILON: f64 = 0.000_001;

/// Arbitrary number of ulps to use when comparing floating point values.
pub const FLOAT_ULPS: u32 = 6;

/// Add the approx AbsDiffEq, RelativeEq and UlpsEq traits to a struct. This
/// handles the very simple case of a series of struct members that are tested
/// in order. It can also generate a test that always returns true - primarily
/// for unit structs.
macro_rules! add_approx_traits {
    (@add_cmp { true }, $fn:ident, $args:tt) => {
        true
    };
    // Unpack the arguments to the function, we can't do this with unpacking the
    // variables since we can't nest different repetition operators.
    (@add_single_cmp $var:ident, $fn:ident, ($self:ident, $other:ident,
        $($rest:ident),+)
    ) => {
        $self.$var.$fn(&$other.$var, $($rest),+)
    };
    // Unpack the variables but keep the arguments as a tt.
    (@add_cmp { $init:ident $(, $var:ident)* }, $fn:ident, $args:tt) => {
        add_approx_traits!(@add_single_cmp $init, $fn, $args)
            $(&& add_approx_traits!(@add_single_cmp $var, $fn, $args))*

    };
    (@add $type:ty, $rest:tt) => {
        impl approx::AbsDiffEq for $type {
            type Epsilon = f64;

            fn default_epsilon() -> Self::Epsilon {
                crate::math::approx::FLOAT_EPSILON
            }

            #[allow(unused_variables)]
            fn abs_diff_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
            ) -> bool {
                add_approx_traits!(@add_cmp $rest, abs_diff_eq, (self, other, epsilon))
            }
        }

        impl approx::RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                crate::math::approx::FLOAT_EPSILON
            }

            #[allow(unused_variables)]
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                add_approx_traits!(
                    @add_cmp $rest, relative_eq, (self, other, epsilon, max_relative)
                )
            }
        }

        impl approx::UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                crate::math::approx::FLOAT_ULPS
            }

            #[allow(unused_variables)]
            fn ulps_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_ulps: u32,
            ) -> bool {
                add_approx_traits!(
                    @add_cmp $rest, ulps_eq, (self, other, epsilon, max_ulps)
                )
            }
        }
    };
    ($type:ty { $init:ident $(, $var:ident)* }) => {
        add_approx_traits!(@add $type, { $init $(, $var)* });
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
