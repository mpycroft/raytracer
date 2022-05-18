use derive_new::new;

use super::PatternAt;
use crate::{
    math::{PerlinNoise, Point},
    util::float::Float,
    Colour,
};

/// A pattern that interpolates between two colours.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct PerlinPattern<T: Float> {
    noise: PerlinNoise<T>,
    colour: Colour<T>,
    scale: T,
}

impl<T: Float> PatternAt<T> for PerlinPattern<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        let scaled_point = Point::new(
            point.x * self.scale,
            point.y * self.scale,
            point.z * self.scale,
        );

        self.colour * self.noise.get_noise(&scaled_point)
    }
}

add_approx_traits!(PerlinPattern<T> { noise, colour, scale });

#[cfg(test)]
mod tests {
    use approx::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;

    // As with PerlinNoise itself, it is quite difficult to test via inspecting
    // values that the pattern is in fact a representation of Perlin noise, for
    // the moment this is only done by inspection.

    #[test]
    fn creating_a_perlin_pattern() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(12661896);
        let n = PerlinNoise::<f64>::new(&mut rng);

        let c = Colour::white();
        let s = 0.3;

        let p = PerlinPattern::new(n, c, s);

        assert_relative_eq!(p.noise, n);
        assert_relative_eq!(p.colour, c);
        assert_relative_eq!(p.scale, s);
    }

    #[test]
    fn perlin_patterns_are_approximately_equal() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(12661896);
        let n = PerlinNoise::<f64>::new(&mut rng);
        let p1 = PerlinPattern::new(n, Colour::white(), 1.6);
        let p2 = PerlinPattern::new(n, Colour::white(), 1.6);
        let p3 = PerlinPattern::new(n, Colour::white(), 1.605);

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
