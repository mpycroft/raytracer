mod blend;
mod checker;
mod gradient;
mod kind;
mod pattern_at;
mod perturbed;
mod radial_gradient;
mod ring;
mod solid;
mod stripe;
#[cfg(test)]
mod test;
mod util;

use paste::paste;
use rand::Rng;
use typed_builder::{Optional, TypedBuilder};

#[cfg(test)]
use self::test::Test;
use self::{
    blend::Blend, checker::Checker, gradient::Gradient, kind::Kind,
    pattern_at::PatternAt, perturbed::Perturbed,
    radial_gradient::RadialGradient, ring::Ring, solid::Solid, stripe::Stripe,
};
use crate::{
    math::{float::impl_approx_eq, Point, Transformable, Transformation},
    Colour, Object,
};

/// A `Pattern` describes a specific pattern that can be applied to a `Material`
/// to change how it is rendered.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(builder_method(vis = "", name = _builder))]
#[builder(build_method(vis = "", name = _build))]
pub struct Pattern {
    #[builder(default = Transformation::new())]
    transformation: Transformation,
    #[builder(default = Transformation::new(), setter(skip))]
    inverse_transformation: Transformation,
    kind: Kind,
}

/// The `add_kind_fn` macro adds a _builder function for the given `Kind`.
macro_rules! add_kind_fn {
    ($kind:ident) => {
        add_kind_fn!($kind(a: Self, b: Self));
    };
    ($kind:ident ($($arg:ident: $ty:ty),*)) => {
        paste! {
            pub fn [<$kind:snake _builder>](
                $($arg: $ty),*
            ) -> PatternBuilder<((), (Kind,))> {
                Self::_builder().kind(Kind::$kind($kind::new($($arg),*)))
            }
        }
    };
}

impl Pattern {
    add_kind_fn!(Blend);
    add_kind_fn!(Checker);
    add_kind_fn!(Gradient);
    add_kind_fn!(RadialGradient);
    add_kind_fn!(Ring);
    add_kind_fn!(Stripe);
    add_kind_fn!(Solid(colour: Colour));
    #[cfg(test)]
    add_kind_fn!(Test());

    pub fn perturbed_builder<R: Rng>(
        scale: f64,
        pattern: Self,
        rng: &mut R,
    ) -> PatternBuilder<((), (Kind,))> {
        Self::_builder()
            .kind(Kind::Perturbed(Perturbed::new(scale, pattern, rng)))
    }

    #[must_use]
    pub fn pattern_at(&self, object: &Object, point: &Point) -> Colour {
        let object_point = object.to_object_space(point);

        self.sub_pattern_at(&object_point)
    }

    #[must_use]
    pub fn sub_pattern_at(&self, point: &Point) -> Colour {
        let pattern_point = point.apply(&self.inverse_transformation);

        self.kind.pattern_at(&pattern_point)
    }
}

/// This is a convenience conversion so we don't need to use
/// `Pattern::default_solid(Colour::new(...))` when all we want is a solid
/// `Colour`.
impl From<Colour> for Pattern {
    fn from(value: Colour) -> Self {
        Self::solid_builder(value).build()
    }
}

impl_approx_eq!(
    &Pattern { ref kind, transformation, inverse_transformation }
);

impl<T: Optional<Transformation>> PatternBuilder<(T, (Kind,))> {
    #[must_use]
    pub fn build(self) -> Pattern {
        let mut pattern = self._build();

        pattern.inverse_transformation = pattern.transformation.invert();

        pattern
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_xoshiro::Xoroshiro128PlusPlus;

    use super::*;
    use crate::{math::float::*, Object};

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn creating_a_pattern() {
        let t = Transformation::new().translate(1.0, 2.0, 3.0);
        let ti = t.invert();

        /// Test creation of `Pattern`s using new_ and default_ functions.
        macro_rules! test_pattern {
            ($kind:ident ($($arg:tt),*)) => {{
                paste! {
                    let p = Kind::$kind(
                        $kind::new($($arg.clone()),*)
                    );

                    let pn = Pattern::[<$kind:snake _builder>](
                        $($arg.clone()),*
                    )
                        .transformation(t)
                        .build();

                    assert_approx_eq!(pn.transformation, t);
                    assert_approx_eq!(pn.inverse_transformation, ti);
                    assert_approx_eq!(pn.kind, &p);

                }
            }};
        }

        let w = Pattern::solid_builder(Colour::white()).build();
        let b = Pattern::solid_builder(Colour::black()).build();

        test_pattern!(Blend(w, b));
        test_pattern!(Checker(w, b));
        test_pattern!(Gradient(w, b));
        test_pattern!(RadialGradient(w, b));
        test_pattern!(Ring(w, b));
        test_pattern!(Stripe(w, b));

        let w = Colour::white();

        test_pattern!(Solid(w));
        test_pattern!(Test());

        let mut r = Xoroshiro128PlusPlus::seed_from_u64(251);

        let p = Kind::Perturbed(Perturbed::new(0.3, w.into(), &mut r));

        let pn = Pattern::perturbed_builder(0.3, w.into(), &mut r)
            .transformation(t)
            .build();

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.kind, &p);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let o = Object::test_builder()
            .transformation(Transformation::new().translate(1.0, 0.5, 1.5))
            .build();

        let p = Pattern::test_builder().build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.0, 3.0, 4.0)),
            Colour::new(1.0, 2.5, 2.5)
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let o = Object::test_builder().build();

        let p = Pattern::test_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.0, 3.0, 4.0)),
            Colour::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let o = Object::test_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let p = Pattern::test_builder()
            .transformation(Transformation::new().translate(0.5, 1.0, 1.5))
            .build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.5, 3.0, 3.5)),
            Colour::new(0.75, 0.5, 0.25)
        );
    }

    #[test]
    fn a_stripe_pattern_with_an_object_transformation() {
        let o = Object::test_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let p = Pattern::stripe_builder(
            Colour::white().into(),
            Colour::black().into(),
        )
        .build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_a_pattern_transformation() {
        let o = Object::test_builder().build();

        let p = Pattern::stripe_builder(
            Colour::white().into(),
            Colour::black().into(),
        )
        .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
        .build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_both_an_object_and_pattern_transformation() {
        let o = Object::test_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let p = Pattern::stripe_builder(
            Colour::white().into(),
            Colour::black().into(),
        )
        .transformation(Transformation::new().translate(0.5, 0.0, 0.0))
        .build();

        assert_approx_eq!(
            p.pattern_at(&o, &Point::new(2.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn comparing_patterns() {
        let p1 = Pattern::test_builder().build();
        let p2 = Pattern::test_builder().build();
        let p3 = Pattern::test_builder()
            .transformation(Transformation::new().scale(1.0, 2.0, 2.0))
            .build();

        assert_approx_eq!(p1, &p2);

        assert_approx_ne!(p1, &p3);
    }
}
