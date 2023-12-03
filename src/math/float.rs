//! The float module provides macros for using floating point numbers based on
//! the float_cmp crate. These macros reimplement the macros in float_cmp but do
//! not require the type to be specified (we call $lhs.approx_eq($rhs) rather
//! than the fully qualified type as ApproxEq). This works for all values except
//! implicit conversion of a margin from a tuple. We are assuming the defaults
//! for epsilon and ulps are "good enough" for our usage but they can be
//! overwritten if needed in certain places.

/// Compare if two values are almost equal. See float-cmp documentation.
macro_rules! approx_eq {
    ($lhs:expr, $rhs:expr) => {
        approx_eq!($lhs, $rhs, float_cmp::F64Margin::default())
    };
    ($lhs:expr, $rhs:expr $(, $set:ident = $val:expr)*) => {{
        let margin = float_cmp::F64Margin::zero()$(.$set($val))*;
        approx_eq!($lhs, $rhs, margin)
    }};
    ($lhs:expr, $rhs:expr, $margin:expr) => {{
        use float_cmp::ApproxEq;
        $lhs.approx_eq($rhs, $margin)
    }};
}

/// Compare if two values are not almost equal. See float-cmp documentation.
macro_rules! approx_ne {
    ($($tt:tt)+) => {
        !approx_eq!($($tt)+)
    };
}

/// Helper macro so we don't have to duplicate code between eq and ne asserts.
macro_rules! _assert_approx_helper {
    ($approx:ident, $lhs:expr, $rhs:expr) => {
        _assert_approx_helper!(
            $approx, $lhs, $rhs, float_cmp::F64Margin::default()
        )
    };
    ($approx:ident, $lhs:expr, $rhs:expr $(, $set:ident = $val:expr)*) => {{
        let margin = float_cmp::F64Margin::zero()$(.$set($val))*;
        _assert_approx_helper!($approx, $lhs, $rhs, margin)
    }};
    ($approx:ident, $lhs:expr, $rhs:expr, $margin:expr) => {{
        if !$approx!($lhs, $rhs, $margin) {
            panic!("\
assertion failed: (left {} right)
  left: {:?},
 right: {:?}", stringify!($approx), $lhs, $rhs);
        }
    }};
}

/// Assert that two values are almost equal. See float-cmp documentation.
macro_rules! assert_approx_eq {
    ($($tt:tt)+) => {
        _assert_approx_helper!(approx_eq, $($tt)+);
    };
}

/// Assert that two values are not almost equal. See float-cmp documentation.
macro_rules! assert_approx_ne {
    ($($tt:tt)+) => {
        _assert_approx_helper!(approx_ne, $($tt)+);
    };
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    #[test]
    fn comparing_floats() {
        let a = 100.15 + 0.15 + 0.15;
        let b = 100.1 + 0.1 + 0.25;
        let c = 4.58;

        assert_ne!(a, b);

        assert!(approx_eq!(a, b));
        assert!(approx_eq!(a, b, epsilon = 1_000.0 * EPSILON));
        assert!(approx_eq!(a, b, ulps = 2));

        assert!(approx_eq!(b, a));
        assert!(approx_eq!(b, a, epsilon = 1_000.0 * EPSILON));
        assert!(approx_eq!(b, a, ulps = 2));

        assert!(approx_eq!(a, b, ulps = 2, epsilon = 0.05 * EPSILON));
        assert!(approx_eq!(b, a, epsilon = 0.05 * EPSILON, ulps = 2));

        assert!(approx_ne!(a, c));
        assert!(approx_ne!(c, a));

        assert!(approx_ne!(a, b, epsilon = 0.05 * EPSILON));
        assert!(approx_ne!(a, b, ulps = 1));

        assert!(approx_ne!(b, a, epsilon = 0.05 * EPSILON));
        assert!(approx_ne!(b, a, ulps = 1));

        assert!(approx_ne!(a, b, ulps = 1, epsilon = 0.05 * EPSILON));
        assert!(approx_ne!(b, a, epsilon = 0.05 * EPSILON, ulps = 1));
    }

    #[test]
    fn asserting_floats() {
        let a = 168_512.002_519_000_6;
        let b = 168_512.002_519_000_7;
        let c = 0.885;

        assert_ne!(a, b);

        assert_approx_eq!(a, b);
        assert_approx_eq!(a, b, ulps = 3);
        assert_approx_eq!(a, b, epsilon = 1_000_000.0 * EPSILON);

        assert_approx_eq!(b, a);
        assert_approx_eq!(b, a, ulps = 3);
        assert_approx_eq!(b, a, epsilon = 1_000_000.0 * EPSILON);

        assert_approx_eq!(a, b, ulps = 3, epsilon = 1_000_000.0 * EPSILON);
        assert_approx_eq!(a, b, epsilon = 1_000_000.0 * EPSILON, ulps = 3);

        assert_approx_ne!(a, c);
        assert_approx_ne!(c, a);

        assert_approx_ne!(a, b, ulps = 1);
        assert_approx_ne!(a, b, epsilon = 0.5 * EPSILON);

        assert_approx_ne!(b, a, ulps = 1);
        assert_approx_ne!(b, a, epsilon = 0.5 * EPSILON);

        assert_approx_ne!(a, b, ulps = 1, epsilon = 0.5 * EPSILON);
        assert_approx_ne!(b, a, epsilon = 0.5 * EPSILON, ulps = 1);
    }
}
