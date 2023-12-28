use derive_more::Constructor;

use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `Checker` patterns produces a checker board pattern where no square
/// touches another of the same `Colour`.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Checker {
    a: Colour,
    b: Colour,
}

impl PatternAt for Checker {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            return self.a;
        }

        self.b
    }
}

impl_approx_eq!(Checker { a, b });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_checker_pattern() {
        let r = Checker::new(Colour::green(), Colour::yellow());

        assert_approx_eq!(r.a, Colour::green());
        assert_approx_eq!(r.b, Colour::yellow());
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_x() {
        let r = Checker::new(Colour::white(), Colour::black());

        assert_approx_eq!(r.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.99, 0.0, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(1.01, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(-1.99, 0.0, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(-2.01, 0.0, 0.0)),
            Colour::black()
        );
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_y() {
        let r = Checker::new(Colour::white(), Colour::black());

        assert_approx_eq!(r.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.99, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 1.01, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, -1.99, 0.0)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, -2.01, 0.0)),
            Colour::black()
        );
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_z() {
        let r = Checker::new(Colour::white(), Colour::black());

        assert_approx_eq!(r.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, 0.99)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, 1.01)),
            Colour::black()
        );

        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, -1.99)),
            Colour::white()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, -2.01)),
            Colour::black()
        );
    }

    #[test]
    fn comparing_checker_patterns() {
        let c1 = Checker::new(Colour::white(), Colour::purple());
        let c2 = Checker::new(Colour::white(), Colour::purple());
        let c3 = Checker::new(Colour::white(), Colour::blue());

        assert_approx_eq!(c1, c2);

        assert_approx_ne!(c1, c3);
    }
}
