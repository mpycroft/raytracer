use super::{Pattern, PatternAt};
use crate::{
    math::{PerlinNoise, Point},
    util::float::Float,
    Colour,
};

/// A marble like pattern using `PerlinNoise`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Perturbed<T: Float> {
    noise: PerlinNoise<T>,
    pattern: Box<Pattern<T>>,
    scale: T,
}

impl<T: Float> Perturbed<T> {
    pub fn new(noise: PerlinNoise<T>, pattern: Pattern<T>, scale: T) -> Self {
        Self { noise, pattern: Box::new(pattern), scale }
    }
}

impl<T: Float> PatternAt<T> for Perturbed<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        let noise = self.noise.get_noise_signed(point) * self.scale;

        let perturbed_point =
            Point::new(point.x + noise, point.y + noise, point.z + noise);

        self.pattern.sub_pattern_at(&perturbed_point)
    }
}

add_approx_traits!(Perturbed<T> { noise, pattern, scale });

#[cfg(test)]
mod tests {
    use approx::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;

    #[test]
    fn creating_a_perturbed_pattern() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(882);
        let n = PerlinNoise::<f64>::new(&mut rng);

        let sp = Pattern::default_ring(Colour::blue(), Colour::black());
        let s = 0.3;

        let p = Perturbed::new(n, sp.clone(), s);

        assert_relative_eq!(p.noise, n);
        assert_relative_eq!(*p.pattern, sp);
        assert_relative_eq!(p.scale, s);
    }

    #[test]
    fn perturbed_patterns_are_approximately_equal() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(127);
        let n = PerlinNoise::<f64>::new(&mut rng);
        let p = Pattern::default_gradient(Colour::green(), Colour::red());
        let p1 = Perturbed::new(n, p.clone(), 3.2);
        let p2 = Perturbed::new(n, p.clone(), 3.2);
        let p3 = Perturbed::new(PerlinNoise::new(&mut rng), p, 3.2);

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
