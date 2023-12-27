use derive_more::Constructor;

use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `Ring` patterns alternates two `Colour`s in concentric rings in x and z.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Ring {
    a: Colour,
    b: Colour,
}

impl PatternAt for Ring {
    #[must_use]
    fn pattern_at(&self, point: &Point) -> Colour {
        if (point.x * point.x + point.z * point.z).sqrt().floor() % 2.0 == 0.0 {
            return self.a;
        }

        self.b
    }
}

impl_approx_eq!(Ring { a, b });

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_ring_pattern() {
        let r = Ring::new(Colour::red(), Colour::black());

        assert_approx_eq!(r.a, Colour::red());
        assert_approx_eq!(r.b, Colour::black());
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let r = Ring::new(Colour::white(), Colour::black());

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
        let r = Ring::new(Colour::white(), Colour::black());

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
    fn comparing_ring_patterns() {
        let r1 = Ring::new(Colour::cyan(), Colour::blue());
        let r2 = Ring::new(Colour::cyan(), Colour::blue());
        let r3 = Ring::new(Colour::white(), Colour::blue());

        assert_approx_eq!(r1, r2);

        assert_approx_ne!(r1, r3);
    }
}
