mod stripe;
#[cfg(test)]
mod test;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};

pub use self::stripe::Stripe;
#[cfg(test)]
pub use self::test::Test;
use crate::{
    math::{float::impl_approx_eq, Point, Transformable, Transformation},
    Colour, Object,
};

#[enum_dispatch]
#[allow(clippy::module_name_repetitions)]
pub trait PatternAt {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour;
}

#[derive(Clone, Copy, Debug)]
pub struct Pattern {
    transformation: Transformation,
    inverse_transformation: Transformation,
    pattern: Patterns,
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

    #[must_use]
    pub fn new_stripe(
        transformation: Transformation,
        a: Colour,
        b: Colour,
    ) -> Self {
        Self::new(transformation, Patterns::Stripe(Stripe::new(a, b)))
    }

    #[must_use]
    pub fn default_stripe(a: Colour, b: Colour) -> Self {
        Self::new_stripe(Transformation::new(), a, b)
    }

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

impl_approx_eq!(Pattern { pattern, transformation, inverse_transformation });

#[derive(Clone, Copy, Debug)]
#[enum_dispatch(PatternAt)]
pub enum Patterns {
    Stripe(Stripe),
    #[cfg(test)]
    Test(Test),
}

impl ApproxEq for Patterns {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Patterns::Stripe(lhs), Patterns::Stripe(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            #[cfg(test)]
            (Patterns::Test(_), Patterns::Test(_)) => true,
            #[cfg(test)]
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

        let p = Patterns::Stripe(Stripe::new(Colour::white(), Colour::green()));

        let pn = Pattern::new(t, p);

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.pattern, p);

        let pn = Pattern::new_stripe(t, Colour::white(), Colour::green());

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.pattern, p);

        let pn = Pattern::default_stripe(Colour::white(), Colour::green());

        assert_approx_eq!(pn.transformation, Transformation::new());
        assert_approx_eq!(pn.inverse_transformation, Transformation::new());
        assert_approx_eq!(pn.pattern, p);

        let p = Patterns::Test(Test);

        let pn = Pattern::new_test(t);

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.pattern, p);

        let pn = Pattern::default_test();

        assert_approx_eq!(pn.transformation, Transformation::new());
        assert_approx_eq!(pn.inverse_transformation, Transformation::new());
        assert_approx_eq!(pn.pattern, p);
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
