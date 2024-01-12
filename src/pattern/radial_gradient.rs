use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `RadialGradient` pattern interpolates between two `Colour`s as x and z
    /// change.
    RadialGradient
);

impl PatternAt for RadialGradient {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        let distance =
            self.b.sub_pattern_at(point) - self.a.sub_pattern_at(point);

        let radial_distance = point.x.hypot(point.z);
        let fraction = radial_distance - radial_distance.floor();

        self.a.sub_pattern_at(point) + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, pattern::util::add_pattern_tests};

    #[test]
    fn a_radial_gradient_should_extend_in_both_x_and_z() {
        let r =
            RadialGradient::new(Colour::white().into(), Colour::black().into());

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
        let r =
            RadialGradient::new(Colour::white().into(), Colour::black().into());

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

    add_pattern_tests!(RadialGradient);
}
