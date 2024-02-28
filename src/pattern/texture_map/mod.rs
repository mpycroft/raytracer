mod spherical_mapping;
mod uv_checker;
mod uv_pattern_at;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};

pub use self::spherical_mapping::spherical_mapping;
use self::{uv_checker::UvChecker, uv_pattern_at::UvPatternAt};
use super::PatternAt;
use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

#[derive(Clone, Copy, Debug)]
#[enum_dispatch(UvPatternAt)]
enum UvPattern {
    UvChecker(UvChecker),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UvMapping {
    Spherical,
}

#[derive(Clone, Copy, Debug)]
pub struct TextureMap {
    pattern: UvPattern,
    mapping: UvMapping,
}

impl TextureMap {
    #[must_use]
    pub fn new_checker(
        width: u32,
        height: u32,
        a: Colour,
        b: Colour,
        mapping: UvMapping,
    ) -> Self {
        Self {
            pattern: UvPattern::UvChecker(UvChecker::new(width, height, a, b)),
            mapping,
        }
    }
}

impl PatternAt for TextureMap {
    fn pattern_at(&self, point: &Point) -> Colour {
        let (u, v) = match self.mapping {
            UvMapping::Spherical => spherical_mapping(point),
        };

        self.pattern.uv_pattern_at(u, v)
    }
}

impl ApproxEq for UvPattern {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (UvPattern::UvChecker(lhs), UvPattern::UvChecker(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
        }
    }
}

impl_approx_eq!(&TextureMap { pattern, eq mapping });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn using_a_texture_map_pattern_with_a_spherical_map() {
        let t = TextureMap::new_checker(
            16,
            8,
            Colour::black(),
            Colour::white(),
            UvMapping::Spherical,
        );

        assert_approx_eq!(
            t.pattern_at(&Point::new(0.431_5, 0.467, 0.771_9)),
            Colour::white()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(-0.965_4, 0.255_2, -0.053_4)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(0.103_9, 0.709, 0.697_5)),
            Colour::white()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(-0.498_6, -0.785_6, -0.366_3)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(-0.031_7, -0.939_5, 0.341_1)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(0.480_9, -0.772_1, 0.415_4)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(0.028_5, -0.961_2, -0.274_5)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(-0.573_4, -0.216_2, -0.790_3)),
            Colour::white()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(0.768_8, -0.147, 0.622_3)),
            Colour::black()
        );
        assert_approx_eq!(
            t.pattern_at(&Point::new(-0.765_2, 0.217_5, 0.606)),
            Colour::black()
        );
    }

    #[test]
    fn comparing_texture_maps() {
        let t1 = TextureMap::new_checker(
            3,
            3,
            Colour::red(),
            Colour::black(),
            UvMapping::Spherical,
        );
        let t2 = TextureMap::new_checker(
            3,
            3,
            Colour::red(),
            Colour::black(),
            UvMapping::Spherical,
        );
        let t3 = TextureMap::new_checker(
            3,
            3,
            Colour::red(),
            Colour::green(),
            UvMapping::Spherical,
        );

        assert_approx_eq!(t1, &t2);

        assert_approx_ne!(t1, &t3);
    }
}
