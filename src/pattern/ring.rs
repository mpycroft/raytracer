use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `Ring` patterns alternates two `Colour`s in concentric rings in x and
    /// z.
    Ring
);

impl PatternAt for Ring {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        if point.x.hypot(point.z).floor() % 2.0 == 0.0 {
            return self.a.sub_pattern_at(point);
        }

        self.b.sub_pattern_at(point)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::{math::float::*, pattern::util::add_pattern_tests};

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let r = Ring::new(Colour::white().into(), Colour::black().into());

        assert_approx_eq!(r.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            r.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::black()
        );
        assert_approx_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::black()
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            r.pattern_at(&Point::new(
                sqrt_2_div_2 + 0.001,
                0.0,
                sqrt_2_div_2 + 0.001
            )),
            Colour::black()
        );
    }

    #[test]
    fn a_ring_should_be_constant_in_y() {
        let r = Ring::new(Colour::white().into(), Colour::black().into());

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

    add_pattern_tests!(Ring);
}
