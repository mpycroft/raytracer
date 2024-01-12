use derive_more::Constructor;

use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `Solid` pattern returns the same colour everywhere, it allows us to
/// replace direct `Colour` usage in `Material` with a `Pattern` and simplify
/// code.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Solid {
    colour: Colour,
}

impl PatternAt for Solid {
    #[must_use]
    fn pattern_at(&self, _point: &Point) -> Colour {
        self.colour
    }
}

impl_approx_eq!(&Solid { colour });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_solid_pattern() {
        let s = Solid::new(Colour::red());

        assert_approx_eq!(s.colour, Colour::red());
    }

    #[test]
    fn a_solid_pattern_always_returns_the_same_colour() {
        let s = Solid::new(Colour::green());

        assert_approx_eq!(s.pattern_at(&Point::origin()), Colour::green());

        assert_approx_eq!(
            s.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::green()
        );
        assert_approx_eq!(
            s.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Colour::green()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 1.5, 0.0)),
            Colour::green()
        );
        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, -0.2, 0.0)),
            Colour::green()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, 3.5)),
            Colour::green()
        );
        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, -2.1)),
            Colour::green()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(1.6, 2.1, -5.2)),
            Colour::green()
        );
    }

    #[test]
    fn comparing_solid_patterns() {
        let s1 = Solid::new(Colour::cyan());
        let s2 = Solid::new(Colour::cyan());
        let s3 = Solid::new(Colour::white());

        assert_approx_eq!(s1, &s2);

        assert_approx_ne!(s1, &s3);
    }
}
