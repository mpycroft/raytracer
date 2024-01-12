//! This module contains helper macros used to generate code for the `Pattern`
//! module.

/// This macro implements a given pattern that contains two sub patterns. It
/// defines the struct, a new function and implements `ApproxEq` for the struct.
/// The only additional thing that should be needed is implementing the actual
/// Pattern trait.
macro_rules! impl_pattern {
    ($(#[$outer:meta])* $pattern:ident) => {
        $(#[$outer])*
        #[derive(Clone, Debug)]
        pub struct $pattern {
            a: Box<crate::Pattern>,
            b: Box<crate::Pattern>,
        }

        impl $pattern {
            #[must_use]
            pub fn new(a: crate::Pattern, b: crate::Pattern) -> Self {
                Self { a: Box::new(a), b: Box::new(b) }
            }
        }

        crate::math::float::impl_approx_eq!(&$pattern { ref a, ref b });
    };
}
pub(super) use impl_pattern;

/// This macro adds in `Kind` creation tests and comparison tests.
#[cfg(test)]
macro_rules! add_kind_tests {
    ($kind:ident) => {
        paste::paste! {
            #[test]
            fn [<creating_a_ $kind:snake>]() {
                let p = $kind::new(
                    Colour::white().into(), Colour::black().into()
                );

                assert_approx_eq!(
                    p.a,
                    &crate::Pattern::solid_builder(Colour::white()).build()
                );
                assert_approx_eq!(
                    p.b,
                    &crate::Pattern::solid_builder(Colour::black()).build()
                );
            }

            #[test]
            fn [<comparing_ $kind:snake s>]() {
                let k1 = $kind::new(
                    Colour::white().into(), Colour::purple().into()
                );
                let k2 = $kind::new(
                    Colour::white().into(), Colour::purple().into()
                );
                let k3 = $kind::new(
                    Colour::white().into(), Colour::blue().into());

                assert_approx_eq!(k1, &k2);

                assert_approx_ne!(k1, &k3);
            }
        }
    };
}
#[cfg(test)]
pub(super) use add_kind_tests;

/// This macro implements the `ApproxEq` trait for the `Kind` enum as it is
/// quite tedious.
macro_rules! impl_approx_eq_patterns {
    ($($(#[$outer:meta])* $pattern:ident $(,)?)+) => {
        impl float_cmp::ApproxEq for &Kind {
            type Margin = float_cmp::F64Margin;

            fn approx_eq<M: Into<Self::Margin>>(
                self, other: Self, margin: M
            ) -> bool {
                let margin = margin.into();

                match (self, other) {
                    $(
                        $(#[$outer])*
                        (Kind::$pattern(lhs), Kind::$pattern(rhs)) => {
                            lhs.approx_eq(rhs, margin)
                        }
                    )+
                    (_, _) => false,
                }
            }
        }
    }
}
pub(super) use impl_approx_eq_patterns;
