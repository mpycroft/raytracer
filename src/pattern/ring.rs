use derive_new::new;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A pattern of concentric rings.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Ring<T: Float> {
    a: Colour<T>,
    b: Colour<T>,
}

impl<T: Float> PatternAt<T> for Ring<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        if (point.x * point.x + point.z * point.z).floor() % T::two()
            == T::zero()
        {
            self.a
        } else {
            self.b
        }
    }
}

add_approx_traits!(Ring<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_ring() {
        let c1 = Colour::white();
        let c2 = Colour::new(0.3, 0.5, 0.8);

        let r = Ring::new(c1, c2);

        assert_relative_eq!(r.a, c1);
        assert_relative_eq!(r.b, c2);
    }

    #[test]
    fn a_ring_extends_in_both_x_and_z() {
        let r = Ring::new(Colour::white(), Colour::black());

        assert_relative_eq!(r.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            r.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::black()
        );
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::black()
        );
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.708, 0.0, 0.708)),
            Colour::black()
        );
    }

    #[test]
    fn a_ring_is_constant_in_y() {
        let r = Ring::new(Colour::white(), Colour::black());

        assert_relative_eq!(r.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Colour::white()
        );
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.0, 1.1, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn rings_are_approximately_equal() {
        let r1 = Ring::<f64>::new(Colour::white(), Colour::black());
        let r2 = Ring::new(Colour::white(), Colour::black());
        let r3 = Ring::new(Colour::new(1.1, 1.0, 1.0), Colour::black());

        assert_abs_diff_eq!(r1, r2);
        assert_abs_diff_ne!(r1, r3);

        assert_relative_eq!(r1, r2);
        assert_relative_ne!(r1, r3);

        assert_ulps_eq!(r1, r2);
        assert_ulps_ne!(r1, r3);
    }
}
