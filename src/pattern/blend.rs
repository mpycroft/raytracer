use super::{Pattern, PatternAt};
use crate::{math::Point, util::float::Float, Colour};

/// A pattern that averages the colour of two different patterns.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Blend<T: Float> {
    a: Box<Pattern<T>>,
    b: Box<Pattern<T>>,
}

impl<T: Float> Blend<T> {
    pub fn new(a: Pattern<T>, b: Pattern<T>) -> Self {
        Self { a: Box::new(a), b: Box::new(b) }
    }
}

impl<T: Float> PatternAt<T> for Blend<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        let a = self.a.sub_pattern_at(point);
        let b = self.b.sub_pattern_at(point);

        (a + b) / T::two()
    }
}

add_approx_traits!(Blend<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_blend() {
        let p1 =
            Pattern::<f64>::default_checker(Colour::white(), Colour::black());
        let p2 =
            Pattern::default_radial_gradient(Colour::red(), Colour::green());

        let c = Blend::new(p1.clone(), p2.clone());

        assert_relative_eq!(*c.a, p1);
        assert_relative_eq!(*c.b, p2);
    }

    #[test]
    fn blend_averages_two_patterns() {
        let b = Blend::<f64>::new(
            Pattern::default_stripe(Colour::red(), Colour::blue()),
            Pattern::default_uniform(Colour::green()),
        );

        assert_relative_eq!(
            b.pattern_at(&Point::origin()),
            Colour::new(0.5, 0.5, 0.0)
        );
        assert_relative_eq!(
            b.pattern_at(&Point::new(0.9, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.0)
        );
        assert_relative_eq!(
            b.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::new(0.0, 0.5, 0.5)
        );
        assert_relative_eq!(
            b.pattern_at(&Point::new(1.1, 0.0, 0.0)),
            Colour::new(0.0, 0.5, 0.5)
        );
    }

    #[test]
    fn blends_are_approximately_equal() {
        let b1 = Blend::<f64>::new(
            Pattern::default_gradient(Colour::white(), Colour::black()),
            Pattern::default_ring(Colour::red(), Colour::green()),
        );
        let b2 = Blend::<f64>::new(
            Pattern::default_gradient(Colour::white(), Colour::black()),
            Pattern::default_ring(Colour::red(), Colour::green()),
        );
        let b3 = Blend::<f64>::new(
            Pattern::default_gradient(
                Colour::white(),
                Colour::new(0.0, 0.001, 0.0),
            ),
            Pattern::default_ring(Colour::red(), Colour::green()),
        );

        assert_abs_diff_eq!(b1, b2);
        assert_abs_diff_ne!(b1, b3);

        assert_relative_eq!(b1, b2);
        assert_relative_ne!(b1, b3);

        assert_ulps_eq!(b1, b2);
        assert_ulps_ne!(b1, b3);
    }
}
