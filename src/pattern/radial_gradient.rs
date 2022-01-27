use derive_more::Constructor;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A gradient pattern that interpolates in rings.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct RadialGradient<T: Float> {
    a: Colour<T>,
    b: Colour<T>,
}

impl<T: Float> PatternAt<T> for RadialGradient<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        let radial_distance = (point.x * point.x + point.z * point.z).sqrt();

        let distance = self.b - self.a;
        let fraction = radial_distance - radial_distance.floor();

        self.a + distance * fraction
    }
}

add_approx_traits!(RadialGradient<T> { a, b });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_radial_gradient() {
        let c1 = Colour::<f64>::green();
        let c2 = Colour::blue();

        let r = RadialGradient::new(c1, c2);

        assert_relative_eq!(r.a, c1);
        assert_relative_eq!(r.b, c2);
    }

    #[test]
    fn a_radial_gradient_extends_in_both_x_and_z() {
        let r = RadialGradient::new(Colour::white(), Colour::black());

        assert_relative_eq!(r.pattern_at(&Point::origin()), Colour::white());
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.25, 0.0, 0.25)),
            Colour::new(0.646_447, 0.646_447, 0.646_447)
        );
        assert_relative_eq!(
            r.pattern_at(&Point::new(0.9, 0.0, 0.9)),
            Colour::new(0.727_208, 0.727_208, 0.727_208)
        );
        assert_relative_eq!(
            r.pattern_at(&Point::new(2.3, 0.0, 2.3)),
            Colour::new(0.747_309, 0.747_309, 0.747_309)
        );
    }

    #[test]
    fn a_radial_gradient_is_constant_in_y() {
        let r = RadialGradient::new(Colour::white(), Colour::black());

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
    fn radial_gradients_are_approximately_equal() {
        let r1 = RadialGradient::<f64>::new(Colour::white(), Colour::black());
        let r2 = RadialGradient::new(Colour::white(), Colour::black());
        let r3 = RadialGradient::new(
            Colour::new(1.0, 1.0, 0.999_98),
            Colour::black(),
        );

        assert_abs_diff_eq!(r1, r2);
        assert_abs_diff_ne!(r1, r3);

        assert_relative_eq!(r1, r2);
        assert_relative_ne!(r1, r3);

        assert_ulps_eq!(r1, r2);
        assert_ulps_ne!(r1, r3);
    }
}
