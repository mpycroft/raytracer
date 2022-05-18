use derive_new::new;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A pattern that is one solid colour.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Uniform<T: Float> {
    colour: Colour<T>,
}

impl<T: Float> PatternAt<T> for Uniform<T> {
    fn pattern_at(&self, _: &Point<T>) -> Colour<T> {
        self.colour
    }
}

add_approx_traits!(Uniform<T> { colour });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_uniform_pattern() {
        let c = Colour::<f64>::white();

        assert_relative_eq!(Uniform::new(c).colour, c);
    }

    #[test]
    fn a_uniform_pattern_should_always_return_the_same_colour() {
        let s = Uniform::new(Colour::green());

        assert_relative_eq!(s.pattern_at(&Point::origin()), Colour::green());
        assert_relative_eq!(
            s.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::green()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Colour::green()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::green()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.6, 0.3, 0.1)),
            Colour::green()
        );
    }

    #[test]
    fn uniform_patterns_are_approximately_equal() {
        let s1 = Uniform::<f64>::new(Colour::red());
        let s2 = Uniform::new(Colour::red());
        let s3 = Uniform::new(Colour::new(0.999, 0.0, 0.0));

        assert_abs_diff_eq!(s1, s2);
        assert_abs_diff_ne!(s1, s3);

        assert_relative_eq!(s1, s2);
        assert_relative_ne!(s1, s3);

        assert_ulps_eq!(s1, s2);
        assert_ulps_ne!(s1, s3);
    }
}
