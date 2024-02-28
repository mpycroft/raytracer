use derive_new::new;

use crate::Colour;

#[derive(Clone, Copy, Debug, new)]
pub struct UvChecker {
    width: u32,
    height: u32,
    a: Colour,
    b: Colour,
}

impl UvChecker {
    pub fn uv_pattern_at(&self, u: f64, v: f64) -> Colour {
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
}
