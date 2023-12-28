use derive_more::Constructor;

use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `RadialGradient` pattern interpolates between two `Colour`s as x and z
/// change.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct RadialGradient {
    a: Colour,
    b: Colour,
}

impl PatternAt for RadialGradient {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        let distance = self.b - self.a;

        let radial_distance = point.x.hypot(point.z);
        let fraction = radial_distance - radial_distance.floor();

        self.a + distance * fraction
    }
}

impl_approx_eq!(RadialGradient { a, b });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_radial_gradient_pattern() {
        let r = RadialGradient::new(Colour::red(), Colour::blue());

        assert_approx_eq!(r.a, Colour::red());
        assert_approx_eq!(r.b, Colour::blue());
    }

    #[test]
    fn a_radial_gradient_should_extend_in_both_x_and_z() {
        let r = RadialGradient::new(Colour::white(), Colour::black());

        assert_approx_eq!(r.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.25, 0.0, 0.25)),
            Colour::new(0.646_447, 0.646_447, 0.646_447),
            epsilon = 0.000_001
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(1.5, 0.0, 1.5)),
            Colour::new(0.878_68, 0.878_68, 0.878_68),
            epsilon = 0.000_001
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(-0.25, 0.0, -0.25)),
            Colour::new(0.646_447, 0.646_447, 0.646_447),
            epsilon = 0.000_001
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(-1.5, 0.0, -1.5)),
            Colour::new(0.878_68, 0.878_68, 0.878_68),
            epsilon = 0.000_001
        );
    }

    #[test]
    fn a_radial_gradient_should_be_constant_in_y() {
        let r = RadialGradient::new(Colour::white(), Colour::black());

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 4.6, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, -1.5, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn comparing_radial_gradient_patterns() {
        let r1 = RadialGradient::new(Colour::purple(), Colour::cyan());
        let r2 = RadialGradient::new(Colour::purple(), Colour::cyan());
        let r3 = RadialGradient::new(Colour::purple(), Colour::green());

        assert_approx_eq!(r1, r2);

        assert_approx_ne!(r1, r3);
    }
}
