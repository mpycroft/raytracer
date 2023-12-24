use derive_more::Constructor;

use crate::{math::Point, Colour};

/// A `Stripe` pattern alternates between two different `Colour`s as the x value
/// changes.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Stripe {
    a: Colour,
    b: Colour,
}

impl Stripe {
    pub fn pattern_at(&self, point: &Point) -> Colour {
        if point.x.floor() % 2.0 == 0.0 {
            return self.a;
        }

        self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let s = Stripe::new(Colour::white(), Colour::black());

        assert_approx_eq!(s.a, Colour::white());
        assert_approx_eq!(s.b, Colour::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let s = Stripe::new(Colour::white(), Colour::black());

        assert_approx_eq!(s.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 2.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let s = Stripe::new(Colour::white(), Colour::black());

        assert_approx_eq!(s.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, 2.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let s = Stripe::new(Colour::white(), Colour::black());

        assert_approx_eq!(s.pattern_at(&Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&Point::new(0.9, 0.0, 0.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(-0.1, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&Point::new(-1.1, 0.0, 0.0)),
            Colour::white()
        );
    }
}
