use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `Checker` pattern produces a checker board pattern where no square
    /// touches another of the same `Colour`.
    Checker
);

impl PatternAt for Checker {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            return self.a.sub_pattern_at(point);
        }

        self.b.sub_pattern_at(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, pattern::util::add_kind_tests};

    #[test]
    fn a_checker_pattern_should_repeat_in_x() {
        let r = Checker::new(Colour::white().into(), Colour::black().into());

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
        let r = Checker::new(Colour::white().into(), Colour::black().into());

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
        let r = Checker::new(Colour::white().into(), Colour::black().into());

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

    add_kind_tests!(Checker);
}
