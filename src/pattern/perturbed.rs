use libnoise::{Generator, Simplex, Source};
use rand::prelude::*;

use super::{Pattern, PatternAt};
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `Perturbed` pattern uses `Simplex` noise to perturb or move the position of
/// each point in x and z.
#[derive(Clone, Debug)]
pub struct Perturbed {
    noise: Box<Simplex<2>>,
    scale: f64,
    pattern: Box<Pattern>,
}

impl Perturbed {
    #[must_use]
    pub fn new<R: Rng>(scale: f64, pattern: Pattern, rng: &mut R) -> Self {
        let noise = Source::simplex(rng.gen::<u64>());

        Self { noise: Box::new(noise), scale, pattern: Box::new(pattern) }
    }
}

impl PatternAt for Perturbed {
    fn pattern_at(&self, point: &Point) -> Colour {
        let value = self.noise.sample([point.x, point.z]) * self.scale;

        self.pattern.sub_pattern_at(&Point::new(
            point.x + value,
            point.y,
            point.z + value,
        ))
    }
}

// Ignore the actual noise function when comparing `Perturbed` patterns since it
// isn't implemented in libnoise.
impl_approx_eq!(&Perturbed { scale, ref pattern });

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoroshiro128PlusPlus;

    use super::*;
    use crate::math::float::*;

    // It is difficult to actually test that values are perturbed by inspecting
    // the f64 outputs so testing is just left up to rendering and seeing if it
    // "looks right".

    #[test]
    fn creating_a_perturbed_pattern() {
        let mut r = Xoroshiro128PlusPlus::seed_from_u64(1);

        let p = Perturbed::new(0.4, Colour::red().into(), &mut r);

        assert_approx_eq!(p.scale, 0.4);
        assert_approx_eq!(
            p.pattern,
            &Pattern::solid_builder(Colour::red()).build()
        );
    }

    #[test]
    fn a_perturbed_pattern() {
        let mut r = Xoroshiro128PlusPlus::seed_from_u64(2);

        let p = Perturbed::new(0.4, Colour::red().into(), &mut r);

        assert_approx_eq!(p.pattern_at(&Point::origin()), Colour::red());
    }

    #[test]
    fn comparing_perturbed_patterns() {
        let mut r = Xoroshiro128PlusPlus::seed_from_u64(3);

        let p1 = Perturbed::new(0.2, Colour::cyan().into(), &mut r);
        let p2 = Perturbed::new(0.2, Colour::cyan().into(), &mut r);
        let p3 = Perturbed::new(0.3, Colour::cyan().into(), &mut r);

        assert_approx_eq!(p1, &p2);

        assert_approx_ne!(p1, &p3);
    }
}
