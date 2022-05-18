use derive_new::new;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// An alternating striped pattern that switches between two different colours.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Stripe<T: Float> {
    a: Colour<T>,
    b: Colour<T>,
}

impl<T: Float> PatternAt<T> for Stripe<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        if point.x.floor() % T::two() == T::zero() {
            self.a
        } else {
            self.b
        }
    }
}

add_approx_traits!(Stripe<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_stripe() {
        let c1 = Colour::new(0.6, 0.3, 1.0);
        let c2 = Colour::new(1.0, 1.0, 0.8);

        let s = Stripe::new(c1, c2);

        assert_relative_eq!(s.a, c1);
        assert_relative_eq!(s.b, c2);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let s = Stripe::<f64>::new(Colour::white(), Colour::black());

        assert_relative_eq!(s.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 2.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let s = Stripe::<f64>::new(Colour::white(), Colour::black());

        assert_relative_eq!(s.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::white()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.0, 1.0, 2.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let s = Stripe::<f64>::new(Colour::white(), Colour::black());

        assert_relative_eq!(s.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            s.pattern_at(&Point::new(0.9, 0.0, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::black()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(-0.1, 0.0, 0.0)),
            Colour::black()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Colour::black()
        );
        assert_relative_eq!(
            s.pattern_at(&Point::new(-1.1, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn stripes_are_approximately_equal() {
        let s1 = Stripe::<f64>::new(Colour::white(), Colour::black());
        let s2 = Stripe::new(Colour::white(), Colour::black());
        let s3 = Stripe::new(Colour::new(0.999_998, 1.0, 1.0), Colour::black());

        assert_abs_diff_eq!(s1, s2);
        assert_abs_diff_ne!(s1, s3);

        assert_relative_eq!(s1, s2);
        assert_relative_ne!(s1, s3);

        assert_ulps_eq!(s1, s2);
        assert_ulps_ne!(s1, s3);
    }
}
