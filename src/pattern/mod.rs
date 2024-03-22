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
mod texture_map;
mod util;

use std::fmt::Display;

use paste::paste;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Deserializer,
};
use serde_yaml::from_value;
use typed_builder::{Optional, TypedBuilder};

#[cfg(test)]
use self::test::Test;
use self::{
    blend::Blend,
    checker::Checker,
    gradient::Gradient,
    kind::Kind,
    pattern_at::PatternAt,
    perturbed::Perturbed,
    radial_gradient::RadialGradient,
    ring::Ring,
    solid::Solid,
    stripe::Stripe,
    texture_map::{TextureMap, UvMapping, UvPattern},
};
use crate::{
    math::{float::impl_approx_eq, Point, Transformable, Transformation},
    scene::HashValue,
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

    pub fn uv_checker_builder(
        width: u32,
        height: u32,
        a: Colour,
        b: Colour,
        mapping: UvMapping,
    ) -> PatternBuilder<((), (Kind,))> {
        Self::_builder().kind(Kind::TextureMap(TextureMap::new_checker(
            width, height, a, b, mapping,
        )))
    }

    pub fn uv_align_check_builder(
        main: Colour,
        upper_left: Colour,
        upper_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
        mapping: UvMapping,
    ) -> PatternBuilder<((), (Kind,))> {
        Self::_builder().kind(Kind::TextureMap(TextureMap::new_align_check(
            main,
            upper_left,
            upper_right,
            bottom_left,
            bottom_right,
            mapping,
        )))
    }

    pub fn uv_cubic_mapping_builder(
        left: UvPattern,
        right: UvPattern,
        front: UvPattern,
        back: UvPattern,
        up: UvPattern,
        down: UvPattern,
    ) -> PatternBuilder<((), (Kind,))> {
        Self::_builder().kind(Kind::TextureMap(TextureMap::new_cubic_mapping(
            left, right, front, back, up, down,
        )))
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

#[allow(clippy::too_many_lines)]
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        type Builder = fn(Pattern, Pattern) -> PatternBuilder<((), (Kind,))>;

        fn err<T, E: Display, R: Error>(err: E) -> Result<T, R> {
            Err(Error::custom(err))
        }

        fn get_value<T: DeserializeOwned, E: Error>(
            hashmap: &mut HashValue,
            key: &str,
        ) -> Result<T, E> {
            let value = hashmap.remove(key).ok_or_else(|| {
                Error::custom(format!("Unable to find value '{key}'"))
            })?;

            from_value(value).map_or_else(err, Ok)
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ColourPattern {
            Colour(Colour),
            Pattern(Pattern),
        }

        #[derive(Deserialize)]
        struct PatternData {
            #[serde(rename = "type")]
            kind: String,
            #[serde(flatten)]
            data: HashValue,
        }

        let mut pattern = PatternData::deserialize(deserializer)?;

        let get_pattern = |pattern| match pattern {
            ColourPattern::Colour(colour) => colour.into(),
            ColourPattern::Pattern(pattern) => pattern,
        };

        let build = |pattern: PatternBuilder<((), (Kind,))>,
                     data: &mut HashValue| {
            if let Some(transformation) = data.remove("transform") {
                Ok(pattern
                    .transformation(
                        from_value(transformation).map_or_else(err, Ok)?,
                    )
                    .build())
            } else {
                Ok(pattern.build())
            }
        };

        let build_simple = |builder: Builder, data: &mut HashValue| {
            let [a, b] = get_value(data, "colors")?;

            let pattern = builder(get_pattern(a), get_pattern(b));

            build(pattern, data)
        };

        match &*pattern.kind {
            "blend" => build_simple(Self::blend_builder, &mut pattern.data),
            "checker" => build_simple(Self::checker_builder, &mut pattern.data),
            "gradient" => {
                build_simple(Self::gradient_builder, &mut pattern.data)
            }
            "radial-gradient" => {
                build_simple(Self::radial_gradient_builder, &mut pattern.data)
            }
            "ring" => build_simple(Self::ring_builder, &mut pattern.data),
            "stripe" => build_simple(Self::stripe_builder, &mut pattern.data),
            "perturbed" => {
                let scale: f64 = get_value(&mut pattern.data, "scale")?;
                let pattern_data: Self =
                    get_value(&mut pattern.data, "pattern")?;
                let seed: u64 = get_value(&mut pattern.data, "seed")?;

                let builder = Self::perturbed_builder(
                    scale,
                    pattern_data,
                    &mut Xoshiro256PlusPlus::seed_from_u64(seed),
                );

                build(builder, &mut pattern.data)
            }
            "map" => {
                let mapping: UvMapping =
                    get_value(&mut pattern.data, "mapping")?;
                let mut uv_pattern: PatternData =
                    get_value(&mut pattern.data, "uv_pattern")?;

                match &*uv_pattern.kind {
                    "checkers" => {
                        let width: u32 =
                            get_value(&mut uv_pattern.data, "width")?;
                        let height: u32 =
                            get_value(&mut uv_pattern.data, "height")?;
                        let [a, b] = get_value(&mut uv_pattern.data, "colors")?;

                        let builder = Self::uv_checker_builder(
                            width, height, a, b, mapping,
                        );

                        build(builder, &mut pattern.data)
                    }
                    "align_check" => {
                        let mut colours: HashValue =
                            get_value(&mut uv_pattern.data, "colors")?;

                        let main: Colour = get_value(&mut colours, "main")?;
                        let upper_left: Colour = get_value(&mut colours, "ul")?;
                        let upper_right: Colour =
                            get_value(&mut colours, "ur")?;
                        let bottom_left: Colour =
                            get_value(&mut colours, "bl")?;
                        let bottom_right: Colour =
                            get_value(&mut colours, "br")?;

                        let builder = Self::uv_align_check_builder(
                            main,
                            upper_left,
                            upper_right,
                            bottom_left,
                            bottom_right,
                            mapping,
                        );

                        build(builder, &mut pattern.data)
                    }
                    _ => err(format!("Unknown pattern '{}'", uv_pattern.kind,)),
                }
            }
            _ => err(format!("Unknown pattern '{}'", pattern.kind,)),
        }
    }
}

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoroshiro128PlusPlus;
    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::{float::*, Angle},
        Object,
    };

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

        let p = Kind::TextureMap(TextureMap::new_checker(
            2,
            2,
            Colour::red(),
            Colour::green(),
            UvMapping::Spherical,
        ));

        let pn = Pattern::uv_checker_builder(
            2,
            2,
            Colour::red(),
            Colour::green(),
            UvMapping::Spherical,
        )
        .transformation(t)
        .build();

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.kind, &p);

        let p = Kind::TextureMap(TextureMap::new_align_check(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::yellow(),
            Colour::purple(),
            UvMapping::Planar,
        ));

        let pn = Pattern::uv_align_check_builder(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::yellow(),
            Colour::purple(),
            UvMapping::Planar,
        )
        .transformation(t)
        .build();

        assert_approx_eq!(pn.transformation, t);
        assert_approx_eq!(pn.inverse_transformation, ti);
        assert_approx_eq!(pn.kind, &p);

        let p = Kind::TextureMap(TextureMap::new_cubic_mapping(
            UvPattern::new_uv_checker(3, 3, Colour::red(), Colour::green()),
            UvPattern::new_uv_checker(3, 3, Colour::blue(), Colour::black()),
            UvPattern::new_uv_checker(3, 3, Colour::white(), Colour::yellow()),
            UvPattern::new_uv_checker(3, 3, Colour::purple(), Colour::cyan()),
            UvPattern::new_uv_checker(3, 3, Colour::red(), Colour::purple()),
            UvPattern::new_uv_checker(3, 3, Colour::green(), Colour::black()),
        ));

        let pn = Pattern::uv_cubic_mapping_builder(
            UvPattern::new_uv_checker(3, 3, Colour::red(), Colour::green()),
            UvPattern::new_uv_checker(3, 3, Colour::blue(), Colour::black()),
            UvPattern::new_uv_checker(3, 3, Colour::white(), Colour::yellow()),
            UvPattern::new_uv_checker(3, 3, Colour::purple(), Colour::cyan()),
            UvPattern::new_uv_checker(3, 3, Colour::red(), Colour::purple()),
            UvPattern::new_uv_checker(3, 3, Colour::green(), Colour::black()),
        )
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

    #[test]
    fn parse_blend_pattern() {
        let p: Pattern = from_str(
            "\
type: blend
colors:
    - type: checker
      colors:
          - [1, 1, 1]
          - [1, 1, 0]
    - [0, 1, 0]
transform:
    - [scale, 2, 2, 2]
    - [rotate-x, 0.5]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::blend_builder(
                crate::Pattern::checker_builder(
                    Colour::white().into(),
                    Colour::yellow().into()
                )
                .build(),
                Colour::green().into()
            )
            .transformation(
                Transformation::new().scale(2.0, 2.0, 2.0).rotate_x(Angle(0.5))
            )
            .build()
        );
    }

    #[test]
    fn parse_checker_pattern() {
        let p: Pattern = from_str(
            "\
type: checker
colors:
    - [0, 0, 1]
    - [0, 0, 0]
transform:
    - [translate, 0, 1, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::checker_builder(
                Colour::blue().into(),
                Colour::black().into()
            )
            .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
            .build()
        );
    }

    #[test]
    fn parse_gradient_pattern() {
        let p: Pattern = from_str(
            "\
type: gradient
colors:
    - [1, 0, 0]
    - [0, 1, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::gradient_builder(
                Colour::red().into(),
                Colour::green().into()
            )
            .build()
        );
    }

    #[test]
    fn parse_radial_gradient_pattern() {
        let p: Pattern = from_str(
            "\
type: radial-gradient
colors:
    - [1, 0, 0]
    - [0, 1, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::radial_gradient_builder(
                Colour::red().into(),
                Colour::green().into()
            )
            .build()
        );
    }

    #[test]
    fn parse_ring_pattern() {
        let p: Pattern = from_str(
            "\
type: ring
colors:
    - [1, 0, 0]
    - [0, 1, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::ring_builder(
                Colour::red().into(),
                Colour::green().into()
            )
            .build()
        );
    }

    #[test]
    fn parse_stripe_pattern() {
        let p: Pattern = from_str(
            "\
type: stripe
colors:
    - [0, 1, 0]
    - [0, 0, 1]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::stripe_builder(
                Colour::green().into(),
                Colour::blue().into()
            )
            .build()
        );
    }

    #[test]
    fn deserialize_perturbed_pattern() {
        let p: Pattern = from_str(
            "\
type: perturbed
scale: 1.2
pattern:
    type: checker
    colors:
        - [0, 1, 0]
        - [0, 0, 1]
    transform:
        - [rotate-x, 0.8]
seed: 515
transform:
    - [translate, 0, 2, 2]
    - [scale, 1.5, 1.5, 1.5]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::perturbed_builder(
                1.2,
                crate::Pattern::checker_builder(
                    Colour::green().into(),
                    Colour::blue().into()
                )
                .transformation(Transformation::new().rotate_x(Angle(0.8)))
                .build(),
                &mut Xoshiro256PlusPlus::seed_from_u64(515)
            )
            .transformation(
                Transformation::new()
                    .translate(0.0, 2.0, 2.0)
                    .scale(1.5, 1.5, 1.5)
            )
            .build()
        );
    }

    #[test]
    fn deserialize_uv_checkers_pattern() {
        let p: Pattern = from_str(
            "\
type: map
mapping: spherical
uv_pattern:
    type: checkers
    width: 4
    height: 5
    colors:
        - [1, 1, 1]
        - [0, 0, 0]
transform:
    - [translate, 1, 2, 3]",
        )
        .unwrap();

        assert_approx_eq!(
            p,
            &crate::Pattern::uv_checker_builder(
                4,
                5,
                Colour::white(),
                Colour::black(),
                UvMapping::Spherical
            )
            .transformation(Transformation::new().translate(1.0, 2.0, 3.0))
            .build()
        );
    }

    #[test]
    fn deserialize_invalid_pattern() {
        assert_eq!(
            from_str::<Pattern>(
                "\
type: foo
colors:
    - [1, 0, 0]
    - [0, 1, 0]",
            )
            .unwrap_err()
            .to_string(),
            "Unknown pattern 'foo'"
        );

        assert_eq!(
            from_str::<Pattern>(
                "\
type: stripe
bar:
    - [1, 0, 0]
    - [0, 1, 0]",
            )
            .unwrap_err()
            .to_string(),
            "Unable to find value 'colors'"
        );

        assert_eq!(
            from_str::<Pattern>(
                "\
type: map
mapping: spherical
uv_pattern:
    type: foo
    width: 2
    height: 2
    colors:
        - [1, 1, 1]
        - [0, 0, 0]",
            )
            .unwrap_err()
            .to_string(),
            "Unknown pattern 'foo'"
        );
    }
}
