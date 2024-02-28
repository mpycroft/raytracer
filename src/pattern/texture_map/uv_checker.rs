use derive_new::new;

use super::uv_pattern_at::UvPatternAt;
use crate::{math::float::impl_approx_eq, Colour};

#[derive(Clone, Copy, Debug, new)]
pub struct UvChecker {
    width: u32,
    height: u32,
    a: Colour,
    b: Colour,
}

impl UvPatternAt for UvChecker {
    fn uv_pattern_at(&self, u: f64, v: f64) -> Colour {
        #[allow(clippy::cast_lossless)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let u = (u * self.width as f64).floor() as u32;
        #[allow(clippy::cast_lossless)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let v = (v * self.height as f64).floor() as u32;

        if (u + v) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl_approx_eq!(UvChecker { eq width, eq height, a, b });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn checker_pattern_in_uv() {
        let c = UvChecker::new(2, 2, Colour::black(), Colour::white());

        assert_approx_eq!(c.uv_pattern_at(0.0, 0.0), Colour::black());
        assert_approx_eq!(c.uv_pattern_at(0.5, 0.0), Colour::white());
        assert_approx_eq!(c.uv_pattern_at(0.0, 0.5), Colour::white());
        assert_approx_eq!(c.uv_pattern_at(0.5, 0.5), Colour::black());
        assert_approx_eq!(c.uv_pattern_at(1.0, 1.0), Colour::black());
    }

    #[test]
    fn comparing_uv_checkers() {
        let u1 = UvChecker::new(4, 5, Colour::blue(), Colour::green());
        let u2 = UvChecker::new(4, 5, Colour::blue(), Colour::green());
        let u3 = UvChecker::new(5, 5, Colour::blue(), Colour::green());

        assert_approx_eq!(u1, u2);

        assert_approx_ne!(u1, u3);
    }
}
