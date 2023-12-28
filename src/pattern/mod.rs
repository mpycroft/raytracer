mod checker;
mod gradient;
mod radial_gradient;
mod ring;
mod solid;
mod stripe;
#[cfg(test)]
mod test;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use paste::paste;

#[cfg(test)]
use self::test::Test;
use self::{
    checker::Checker, gradient::Gradient, radial_gradient::RadialGradient,
    ring::Ring, solid::Solid, stripe::Stripe,
};
use crate::{
    math::{float::impl_approx_eq, Point, Transformable, Transformation},
    Colour, Object,
};

/// A trait that all types of `Patterns` are required to implement.
#[enum_dispatch]
#[allow(clippy::module_name_repetitions)]
pub trait PatternAt {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour;
}

/// A `Pattern` describes a specific pattern that can be applied to a `Material`
/// to change how it is rendered.
#[derive(Clone, Copy, Debug)]
pub struct Pattern {
    transformation: Transformation,
    inverse_transformation: Transformation,
    pattern: Patterns,
}

macro_rules! add_pattern_fns {
    ($pattern:ident ($($arg:ident: $ty:ty),+)) => {
        paste! {
            #[must_use]
            pub fn [<new_ $pattern:snake>](
                transformation: Transformation, $($arg: $ty),+
            ) -> Self {
                Self::new(
                    transformation, Patterns::$pattern($pattern::new($($arg),+))
                )
            }

            #[must_use]
            pub fn [<default_ $pattern:snake>]($($arg: $ty),+) -> Self {
                Self::[<new_ $pattern:snake>](Transformation::new(), $($arg),+)
            }
        }
    };
}

impl Pattern {
    #[must_use]
    fn new(transformation: Transformation, pattern: Patterns) -> Self {
        Self {
            transformation,
            inverse_transformation: transformation.invert(),
            pattern,
        }
    }

    add_pattern_fns!(Checker(a: Colour, b: Colour));
    add_pattern_fns!(Gradient(a: Colour, b: Colour));
    add_pattern_fns!(RadialGradient(a: Colour, b: Colour));
    add_pattern_fns!(Ring(a: Colour, b: Colour));
    add_pattern_fns!(Stripe(a: Colour, b: Colour));
    add_pattern_fns!(Solid(colour: Colour));

    #[cfg(test)]
    #[must_use]
    pub fn new_test(transformation: Transformation) -> Self {
        Self::new(transformation, Patterns::Test(Test))
    }

    #[cfg(test)]
    #[must_use]
    pub fn default_test() -> Self {
        Self::new_test(Transformation::new())
    }

    #[must_use]
    pub fn pattern_at(&self, object: &Object, point: &Point) -> Colour {
        let object_point = object.to_object_space(point);

        let pattern_point = object_point.apply(&self.inverse_transformation);

        self.pattern.pattern_at(&pattern_point)
    }
}

/// This is a convenience conversion so we don't need to use
/// `Pattern::default_solid(Colour::new(...))` when all we want is a solid
/// `Colour`.
impl From<Colour> for Pattern {
    fn from(value: Colour) -> Self {
        Self::default_solid(value)
    }
}

impl_approx_eq!(Pattern { pattern, transformation, inverse_transformation });

/// The set of all patterns we know how to render.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch(PatternAt)]
pub enum Patterns {
    Checker(Checker),
    Gradient(Gradient),
    RadialGradient(RadialGradient),
    Ring(Ring),
    Stripe(Stripe),
    Solid(Solid),
    #[cfg(test)]
    Test(Test),
}

impl ApproxEq for Patterns {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Checker(lhs), Self::Checker(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Self::Gradient(lhs), Self::Gradient(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Self::RadialGradient(lhs), Self::RadialGradient(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Self::Ring(lhs), Self::Ring(rhs)) => lhs.approx_eq(rhs, margin),
            (Self::Stripe(lhs), Self::Stripe(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Self::Solid(lhs), Self::Solid(rhs)) => lhs.approx_eq(rhs, margin),
            #[cfg(test)]
            (Self::Test(_), Self::Test(_)) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Material};

    #[test]
    fn creating_a_pattern() {
        let t = Transformation::new().translate(1.0, 2.0, 3.0);
        let ti = t.invert();

        macro_rules! test_pattern {
            ($pattern:ident ($($arg:tt),*)) => {{
                paste! {
                    let p = Patterns::$pattern($pattern::new($($arg),*));

                    let pn = Pattern::[<new_ $pattern:snake>](t, $($arg),*);

                    assert_approx_eq!(pn.transformation, t);
                    assert_approx_eq!(pn.inverse_transformation, ti);
                    assert_approx_eq!(pn.pattern, p);

                    let pn = Pattern::[<default_ $pattern:snake>]($($arg),*);

                    assert_approx_eq!(pn.transformation, Transformation::new());
                    assert_approx_eq!(
                        pn.inverse_transformation, Transformation::new()
                    );
                    assert_approx_eq!(pn.pattern, p);
                }
            }};
        }

        let p = Patterns::Stripe(Stripe::new(Colour::white(), Colour::green()));

        let pn = Pattern::new(t, p);

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.pattern, p);

        let w = Colour::white();
        let b = Colour::black();

        test_pattern!(Checker(w, b));
        test_pattern!(Gradient(w, b));
        test_pattern!(RadialGradient(w, b));
        test_pattern!(Ring(w, b));
        test_pattern!(Stripe(w, b));
        test_pattern!(Solid(w));
        test_pattern!(Test());
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let o = Object::new_test(
            Transformation::new().translate(1.0, 0.5, 1.5),
            Material::default(),
        );

        let p = Pattern::default_test();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.0, 3.0, 4.0)),
            Colour::new(1.0, 2.5, 2.5)
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let o = Object::default_test();

        let p = Pattern::new_test(Transformation::new().scale(2.0, 2.0, 2.0));

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.0, 3.0, 4.0)),
            Colour::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let o = Object::new_test(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        let p =
            Pattern::new_test(Transformation::new().translate(0.5, 1.0, 1.5));

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.5, 3.0, 3.5)),
            Colour::new(0.75, 0.5, 0.25)
        );
    }

    #[test]
    fn a_stripe_pattern_with_an_object_transformation() {
        let o = Object::new_test(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        let p = Pattern::default_stripe(Colour::white(), Colour::black());

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_a_pattern_transformation() {
        let o = Object::default_test();

        let p = Pattern::new_stripe(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_both_an_object_and_pattern_transformation() {
        let o = Object::new_test(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        let p = Pattern::new_stripe(
            Transformation::new().translate(0.5, 0.0, 0.0),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn comparing_patterns() {
        let p1 = Pattern::default_test();
        let p2 = Pattern::default_test();
        let p3 = Pattern::new_test(Transformation::new().scale(1.0, 2.0, 2.0));

        assert_approx_eq!(p1, p2);

        assert_approx_ne!(p1, p3);

        let p1 = Patterns::Stripe(Stripe::new(Colour::black(), Colour::blue()));
        let p2 = Patterns::Stripe(Stripe::new(Colour::black(), Colour::blue()));
        let p3 = Patterns::Test(Test);

        assert_approx_eq!(p1, p2);

        assert_approx_ne!(p1, p3);
    }
}
