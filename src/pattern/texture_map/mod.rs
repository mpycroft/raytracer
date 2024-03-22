mod cubic_mapping;
mod uv_align_check;
mod uv_checker;
mod uv_mapping;
mod uv_pattern;
mod uv_pattern_at;

use float_cmp::{ApproxEq, F64Margin};

use self::{
    cubic_mapping::CubicMapping, uv_align_check::UvAlignCheck,
    uv_checker::UvChecker, uv_pattern_at::UvPatternAt,
};
pub use self::{uv_mapping::UvMapping, uv_pattern::UvPattern};
use super::PatternAt;
use crate::{math::Point, Colour};

#[derive(Clone, Debug)]
pub enum TextureMap {
    SingleMapping { pattern: UvPattern, mapping: UvMapping },
    CubicMapping(Box<CubicMapping>),
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
        Self::SingleMapping {
            pattern: UvPattern::new_uv_checker(width, height, a, b),
            mapping,
        }
    }

    #[must_use]
    pub fn new_align_check(
        main: Colour,
        upper_left: Colour,
        upper_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
        mapping: UvMapping,
    ) -> Self {
        Self::SingleMapping {
            pattern: UvPattern::new_align_check(
                main,
                upper_left,
                upper_right,
                bottom_left,
                bottom_right,
            ),
            mapping,
        }
    }

    #[must_use]
    pub fn new_cubic_mapping(
        left: UvPattern,
        right: UvPattern,
        front: UvPattern,
        back: UvPattern,
        up: UvPattern,
        down: UvPattern,
    ) -> Self {
        TextureMap::CubicMapping(Box::new(CubicMapping::new(
            left, right, front, back, up, down,
        )))
    }
}

impl PatternAt for TextureMap {
    fn pattern_at(&self, point: &Point) -> Colour {
        match self {
            Self::SingleMapping { pattern, mapping } => {
                let (u, v) = mapping.get_u_v(point);

                pattern.uv_pattern_at(u, v)
            }
            Self::CubicMapping(cubic_mapping) => {
                cubic_mapping.pattern_at(point)
            }
        }
    }
}

impl ApproxEq for &TextureMap {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (
                TextureMap::SingleMapping {
                    pattern: lhs_pattern,
                    mapping: lhs_mapping,
                },
                TextureMap::SingleMapping {
                    pattern: rhs_pattern,
                    mapping: rhs_mapping,
                },
            ) => {
                lhs_pattern.approx_eq(rhs_pattern, margin)
                    && lhs_mapping == rhs_mapping
            }
            (TextureMap::CubicMapping(lhs), TextureMap::CubicMapping(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (_, _) => false,
        }
    }
}

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
        let t4 = TextureMap::new_align_check(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::white(),
            Colour::black(),
            UvMapping::Planar,
        );
        let t5 = TextureMap::new_align_check(
            Colour::red(),
            Colour::green(),
            Colour::blue(),
            Colour::white(),
            Colour::black(),
            UvMapping::Planar,
        );

        assert_approx_eq!(t1, &t2);

        assert_approx_ne!(t1, &t3);

        assert_approx_eq!(t4, &t5);

        assert_approx_ne!(t1, &t4);
    }
}
