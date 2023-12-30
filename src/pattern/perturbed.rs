use libnoise::{Generator, Simplex, Source};
use rand::random;

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
    pub fn new(scale: f64, pattern: Pattern) -> Self {
        let noise = Source::simplex(random());

        Self { noise: Box::new(noise), scale, pattern: Box::new(pattern) }
    }
}

impl PatternAt for Perturbed {
    #[must_use]
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
    use super::*;
    use crate::math::float::*;

    // It is difficult to actually test that values are perturbed by inspecting
    // the f64 outputs so testing is just left up to rendering and seeing if it
    // "looks right".

    #[test]
    fn creating_a_perturbed_pattern() {
        let p = Perturbed::new(0.4, Colour::red().into());

        assert_approx_eq!(p.scale, 0.4);
        assert_approx_eq!(p.pattern, &Pattern::default_solid(Colour::red()));
    }

    #[test]
    fn comparing_perturbed_patterns() {
        let p1 = Perturbed::new(0.2, Colour::cyan().into());
        let p2 = Perturbed::new(0.2, Colour::cyan().into());
        let p3 = Perturbed::new(0.3, Colour::cyan().into());

        assert_approx_eq!(p1, &p2);

        assert_approx_ne!(p1, &p3);
    }
}
