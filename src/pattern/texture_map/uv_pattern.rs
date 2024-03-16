use enum_dispatch::enum_dispatch;

use super::{UvAlignCheck, UvChecker};
use crate::{math::float::impl_approx_eq, Colour};

/// A `UvPattern` is a type of pattern that uses u, v coordinates that are
/// mapped in some way from a 3d `Point`.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch(UvPatternAt)]
pub enum UvPattern {
    UvChecker(UvChecker),
    UvAlignCheck(UvAlignCheck),
}

impl UvPattern {
    #[must_use]
    pub fn new_uv_checker(
        width: u32,
        height: u32,
        a: Colour,
        b: Colour,
    ) -> Self {
        Self::UvChecker(UvChecker::new(width, height, a, b))
    }

    #[must_use]
    pub fn new_align_check(
        main: Colour,
        upper_left: Colour,
        upper_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
    ) -> Self {
        Self::UvAlignCheck(UvAlignCheck::new(
            main,
            upper_left,
            upper_right,
            bottom_left,
            bottom_right,
        ))
    }
}

impl_approx_eq!(
    enum UvPattern {
        UvChecker,
        UvAlignCheck,
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn comparing_uv_patterns() {
        let p1 =
            UvPattern::new_uv_checker(3, 3, Colour::green(), Colour::black());
        let p2 =
            UvPattern::new_uv_checker(3, 3, Colour::green(), Colour::black());
        let p3 =
            UvPattern::new_uv_checker(3, 3, Colour::red(), Colour::black());
        let p4 = UvPattern::new_align_check(
            Colour::black(),
            Colour::white(),
            Colour::red(),
            Colour::green(),
            Colour::blue(),
        );
        let p5 = UvPattern::new_align_check(
            Colour::black(),
            Colour::white(),
            Colour::red(),
            Colour::green(),
            Colour::blue(),
        );

        assert_approx_eq!(p1, &p2);

        assert_approx_ne!(p1, &p3);

        assert_approx_eq!(p4, &p5);

        assert_approx_ne!(p1, &p4);
    }
}
