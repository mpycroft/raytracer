use derive_more::Constructor;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A pattern that interpolates between two colours.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Checker<T: Float> {
    a: Colour<T>,
    b: Colour<T>,
}

impl<T: Float> PatternAt<T> for Checker<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % T::two()
            == T::zero()
        {
            self.a
        } else {
            self.b
        }
    }
}

add_approx_traits!(Checker<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_checker() {
        let c1 = Colour::<f64>::white();
        let c2 = Colour::red();

        let c = Checker::new(c1, c2);

        assert_relative_eq!(c.a, c1);
        assert_relative_eq!(c.b, c2);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let c = Checker::new(Colour::white(), Colour::black());

        assert_relative_eq!(c.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            c.pattern_at(&Point::new(0.99, 0.0, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            c.pattern_at(&Point::new(1.01, 0.0, 0.0)),
            Colour::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let c = Checker::new(Colour::white(), Colour::black());

        assert_relative_eq!(c.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            c.pattern_at(&Point::new(0.0, 0.99, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            c.pattern_at(&Point::new(0.0, 1.01, 0.0)),
            Colour::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let c = Checker::new(Colour::white(), Colour::black());

        assert_relative_eq!(c.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            c.pattern_at(&Point::new(0.0, 0.0, 0.99)),
            Colour::white()
        );
        assert_relative_eq!(
            c.pattern_at(&Point::new(0.0, 0.0, 1.01)),
            Colour::black()
        );
    }

    #[test]
    fn checkers_are_approximately_equal() {
        let c1 = Checker::<f64>::new(Colour::white(), Colour::black());
        let c2 = Checker::new(Colour::white(), Colour::black());
        let c3 = Checker::new(Colour::new(1.1, 1.0, 1.0), Colour::black());

        assert_abs_diff_eq!(c1, c2);
        assert_abs_diff_ne!(c1, c3);

        assert_relative_eq!(c1, c2);
        assert_relative_ne!(c1, c3);

        assert_ulps_eq!(c1, c2);
        assert_ulps_ne!(c1, c3);
    }
}
