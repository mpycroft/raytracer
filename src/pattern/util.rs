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

/// This macro adds in pattern creation tests and comparison tests.
#[cfg(test)]
macro_rules! add_pattern_tests {
    ($pattern:ident) => {
        paste::paste! {
            #[test]
            fn [<creating_a_ $pattern:snake _pattern>]() {
                let p = $pattern::new(
                    Colour::white().into(), Colour::black().into()
                );

                assert_approx_eq!(
                    p.a, &crate::Pattern::default_solid(Colour::white())
                );
                assert_approx_eq!(
                    p.b, &crate::Pattern::default_solid(Colour::black())
                );
            }

            #[test]
            fn [<comparing_ $pattern:snake _patterns>]() {
                let p1 = $pattern::new(
                    Colour::white().into(), Colour::purple().into()
                );
                let p2 = $pattern::new(
                    Colour::white().into(), Colour::purple().into()
                );
                let p3 = $pattern::new(
                    Colour::white().into(), Colour::blue().into());

                assert_approx_eq!(p1, &p2);

                assert_approx_ne!(p1, &p3);
            }
        }
    };
}
#[cfg(test)]
pub(super) use add_pattern_tests;

/// This macro implements the `ApproxEq` trait for the `Patterns` enum as it is
/// quite tedious.
macro_rules! impl_approx_eq_patterns {
    ($($(#[$outer:meta])* $pattern:ident $(,)?)+) => {
        impl ApproxEq for &Patterns {
            type Margin = F64Margin;

            fn approx_eq<M: Into<Self::Margin>>(
                self, other: Self, margin: M
            ) -> bool {
                let margin = margin.into();

                match (self, other) {
                    $(
                        $(#[$outer])*
                        (Patterns::$pattern(lhs), Patterns::$pattern(rhs)) => {
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
