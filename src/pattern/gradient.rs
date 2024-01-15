use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `Gradient` pattern smoothly changes between two `Colour`s as the x
    /// value changes.
    Gradient
);

impl PatternAt for Gradient {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        let distance =
            self.b.sub_pattern_at(point) - self.a.sub_pattern_at(point);

        let fraction = point.x - point.x.floor();

        self.a.sub_pattern_at(point) + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, pattern::util::add_kind_tests};

    #[test]
    fn a_gradient_linearly_interpolates_between_colours() {
        let g = Gradient::new(Colour::white().into(), Colour::black().into());

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

        assert_approx_eq!(
            g.pattern_at(&Point::new(-0.25, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(-0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );

        assert_approx_eq!(
            g.pattern_at(&Point::new(-0.75, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
    }

    #[test]
    fn a_gradient_is_constant_in_y_and_z() {
        let g = Gradient::new(Colour::white().into(), Colour::black().into());

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

    add_kind_tests!(Gradient);
}
