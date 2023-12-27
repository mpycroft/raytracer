use derive_more::Constructor;

use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `Gradient` pattern smoothly changes between two `Colour`s as the x value
/// changes.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Gradient {
    a: Colour,
    b: Colour,
}

impl PatternAt for Gradient {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        let distance = self.b - self.a;

        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

impl_approx_eq!(Gradient { a, b });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_gradient_pattern() {
        let g = Gradient::new(Colour::red(), Colour::blue());

        assert_approx_eq!(g.a, Colour::red());
        assert_approx_eq!(g.b, Colour::blue());
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colours() {
        let g = Gradient::new(Colour::white(), Colour::black());

        assert_approx_eq!(g.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_gradient_is_constant_in_y_and_z() {
        let g = Gradient::new(Colour::white(), Colour::black());

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.0, 3.0, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            g.pattern_at(&Point::new(0.0, -2.1, 0.0)),
            Colour::white()
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.0, 0.0, 1.5)),
            Colour::white()
        );
        assert_approx_eq!(
            g.pattern_at(&Point::new(0.0, 0.0, -0.1)),
            Colour::white()
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(0.0, -4.2, 2.4)),
            Colour::white()
        );
    }

    #[test]
    fn comparing_gradient_patterns() {
        let g1 = Gradient::new(Colour::purple(), Colour::cyan());
        let g2 = Gradient::new(Colour::purple(), Colour::cyan());
        let g3 = Gradient::new(Colour::purple(), Colour::green());

        assert_approx_eq!(g1, g2);

        assert_approx_ne!(g1, g3);
    }
}
