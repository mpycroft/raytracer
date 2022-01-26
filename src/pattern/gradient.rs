use derive_more::Constructor;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A pattern that interpolates between two colours.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Gradient<T: Float> {
    a: Colour<T>,
    b: Colour<T>,
}

impl<T: Float> PatternAt<T> for Gradient<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

add_approx_traits!(Gradient<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_gradient() {
        let c1 = Colour::new(0.3, 2.6, 0.9);
        let c2 = Colour::new(2.0, 2.0, 1.0);

        let g = Gradient::new(c1, c2);

        assert_relative_eq!(g.a, c1);
        assert_relative_eq!(g.b, c2);
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colours() {
        let g = Gradient::new(Colour::white(), Colour::black());

        assert_relative_eq!(g.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_gradient_is_constant_in_y() {
        let g = Gradient::new(Colour::white(), Colour::black());

        assert_relative_eq!(g.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.0, 0.25, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.0, 0.80, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_gradient_is_constant_in_z() {
        let g = Gradient::new(Colour::white(), Colour::black());

        assert_relative_eq!(g.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.0, 0.0, 0.5)),
            Colour::white()
        );
        assert_relative_eq!(
            g.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::white()
        );
    }

    #[test]
    fn gradients_are_approximately_equal() {
        let g1 = Gradient::<f64>::new(Colour::white(), Colour::black());
        let g2 = Gradient::new(Colour::white(), Colour::black());
        let g3 =
            Gradient::new(Colour::new(1.0, 1.000_03, 1.0), Colour::black());

        assert_abs_diff_eq!(g1, g2);
        assert_abs_diff_ne!(g1, g3);

        assert_relative_eq!(g1, g2);
        assert_relative_ne!(g1, g3);

        assert_ulps_eq!(g1, g2);
        assert_ulps_ne!(g1, g3);
    }
}
