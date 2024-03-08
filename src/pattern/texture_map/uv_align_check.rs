use derive_new::new;

use super::uv_pattern_at::UvPatternAt;
use crate::{math::float::impl_approx_eq, Colour};

#[derive(Clone, Copy, Debug, new)]
pub struct UvAlignCheck {
    main: Colour,
    upper_left: Colour,
    upper_right: Colour,
    bottom_left: Colour,
    bottom_right: Colour,
}

impl UvPatternAt for UvAlignCheck {
    fn uv_pattern_at(&self, u: f64, v: f64) -> Colour {
        if v > 0.8 {
            if u < 0.2 {
                return self.upper_left;
            }
            if u > 0.8 {
                return self.upper_right;
            }
        } else if v < 0.2 {
            if u < 0.2 {
                return self.bottom_left;
            }
            if u > 0.8 {
                return self.bottom_right;
            }
        }

        self.main
    }
}

impl_approx_eq!(&UvAlignCheck {
    main,
    upper_left,
    upper_right,
    bottom_left,
    bottom_right
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn layout_of_the_align_check_pattern() {
        let a = UvAlignCheck::new(
            Colour::white(),
            Colour::red(),
            Colour::yellow(),
            Colour::green(),
            Colour::purple(),
        );

        assert_approx_eq!(a.uv_pattern_at(0.5, 0.5), Colour::white());
        assert_approx_eq!(a.uv_pattern_at(0.1, 0.9), Colour::red());
        assert_approx_eq!(a.uv_pattern_at(0.9, 0.9), Colour::yellow());
        assert_approx_eq!(a.uv_pattern_at(0.1, 0.1), Colour::green());
        assert_approx_eq!(a.uv_pattern_at(0.9, 0.1), Colour::purple());
    }

    #[test]
    fn comparing_uv_align_checks() {
        let u1 = UvAlignCheck::new(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::white(),
            Colour::black(),
        );
        let u2 = UvAlignCheck::new(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::white(),
            Colour::black(),
        );
        let u3 = UvAlignCheck::new(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::purple(),
            Colour::black(),
        );

        assert_approx_eq!(u1, &u2);

        assert_approx_ne!(u1, &u3);
    }
}
