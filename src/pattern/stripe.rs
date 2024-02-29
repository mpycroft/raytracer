use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `Stripe` pattern alternates between two different `Colour`s as the x
    /// value changes.
    Stripe
);

impl PatternAt for Stripe {
    fn pattern_at(&self, point: &Point) -> Colour {
        if point.x.floor().rem_euclid(2.0) == 0.0 {
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
    fn a_stripe_pattern_is_constant_in_y() {
        let s = Stripe::new(Colour::white().into(), Colour::black().into());

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
        let s = Stripe::new(Colour::white().into(), Colour::black().into());

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
        let s = Stripe::new(Colour::white().into(), Colour::black().into());

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

    add_kind_tests!(Stripe);
}
