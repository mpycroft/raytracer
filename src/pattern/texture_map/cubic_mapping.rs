use enum_map::{enum_map, Enum, EnumMap};
use float_cmp::{ApproxEq, F64Margin};

use super::{UvPattern, UvPatternAt};
use crate::{
    math::{float::approx_eq, Point},
    pattern::pattern_at::PatternAt,
    Colour,
};

#[derive(Clone, Copy, Debug)]
pub struct CubicMapping(EnumMap<Face, UvPattern>);

impl CubicMapping {
    #[must_use]
    pub fn new(
        left: UvPattern,
        right: UvPattern,
        front: UvPattern,
        back: UvPattern,
        up: UvPattern,
        down: UvPattern,
    ) -> Self {
        Self(enum_map! {
            Face::Left => left,
            Face::Right => right,
            Face::Front => front,
            Face::Back => back,
            Face::Up => up,
            Face::Down => down,
        })
    }

    #[must_use]
    fn cube_uv(face: Face, point: &Point) -> (f64, f64) {
        match face {
            Face::Left => {
                let u = (point.z + 1.0).rem_euclid(2.0) / 2.0;
                let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;

                (u, v)
            }
            Face::Right => {
                let u = (1.0 - point.z).rem_euclid(2.0) / 2.0;
                let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;

                (u, v)
            }
            Face::Front => {
                let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
                let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;

                (u, v)
            }
            Face::Back => {
                let u = (1.0 - point.x).rem_euclid(2.0) / 2.0;
                let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;

                (u, v)
            }
            Face::Up => {
                let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
                let v = (1.0 - point.z).rem_euclid(2.0) / 2.0;

                (u, v)
            }
            Face::Down => {
                let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
                let v = (point.z + 1.0).rem_euclid(2.0) / 2.0;

                (u, v)
            }
        }
    }
}

impl PatternAt for CubicMapping {
    fn pattern_at(&self, point: &Point) -> Colour {
        let face = Face::from(*point);

        let (u, v) = Self::cube_uv(face, point);

        self.0[face].uv_pattern_at(u, v)
    }
}

/// A representation of the faces of a cube.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
enum Face {
    Left,
    Right,
    Front,
    Back,
    Up,
    Down,
}

impl From<Point> for Face {
    fn from(value: Point) -> Self {
        let abs_x = value.x.abs();
        let abs_y = value.y.abs();
        let abs_z = value.z.abs();

        let coord = abs_x.max(abs_y.max(abs_z));

        if approx_eq!(coord, value.x) {
            Self::Right
        } else if approx_eq!(coord, -value.x) {
            Self::Left
        } else if approx_eq!(coord, value.y) {
            Self::Up
        } else if approx_eq!(coord, -value.y) {
            Self::Down
        } else if approx_eq!(coord, value.z) {
            Self::Front
        } else {
            Self::Back
        }
    }
}

impl ApproxEq for &CubicMapping {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        for (lhs, rhs) in self.0.values().zip(other.0.values()) {
            if !lhs.approx_eq(rhs, margin) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Colour};

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn creating_a_cubic_mapping() {
        let l = UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::green());
        let r =
            UvPattern::new_uv_checker(2, 2, Colour::blue(), Colour::green());
        let f =
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::purple());
        let b =
            UvPattern::new_uv_checker(2, 2, Colour::white(), Colour::green());
        let u = UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black());
        let d = UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::blue());

        let c = CubicMapping::new(l, r, f, b, u, d);

        assert_approx_eq!(c.0[Face::Left], &l);
        assert_approx_eq!(c.0[Face::Right], &r);
        assert_approx_eq!(c.0[Face::Front], &f);
        assert_approx_eq!(c.0[Face::Back], &b);
        assert_approx_eq!(c.0[Face::Up], &u);
        assert_approx_eq!(c.0[Face::Down], &d);
    }

    #[test]
    fn identifying_the_face_of_a_cube_from_a_point() {
        assert_eq!(Face::from(Point::new(-1.0, 0.5, -0.25)), Face::Left);
        assert_eq!(Face::from(Point::new(1.1, -0.75, 0.8)), Face::Right);
        assert_eq!(Face::from(Point::new(0.1, 0.6, 0.9)), Face::Front);
        assert_eq!(Face::from(Point::new(-0.7, 0.0, -2.0)), Face::Back);
        assert_eq!(Face::from(Point::new(0.5, 1.0, 0.9)), Face::Up);
        assert_eq!(Face::from(Point::new(-0.2, -1.3, 1.1)), Face::Down);
    }

    #[test]
    fn uv_mapping_a_cube() {
        let test = |point, cu, cv| {
            let f = Face::from(point);
            let (u, v) = CubicMapping::cube_uv(f, &point);

            assert_approx_eq!(u, cu);
            assert_approx_eq!(v, cv);
        };

        test(Point::new(-0.5, 0.5, 1.0), 0.25, 0.75);
        test(Point::new(0.5, -0.5, 1.0), 0.75, 0.25);

        test(Point::new(0.5, 0.5, -1.0), 0.25, 0.75);
        test(Point::new(-0.5, -0.5, -1.0), 0.75, 0.25);

        test(Point::new(-1.0, 0.5, -0.5), 0.25, 0.75);
        test(Point::new(-1.0, -0.5, 0.5), 0.75, 0.25);

        test(Point::new(1.0, 0.5, 0.5), 0.25, 0.75);
        test(Point::new(1.0, -0.5, -0.5), 0.75, 0.25);

        test(Point::new(-0.5, 1.0, -0.5), 0.25, 0.75);
        test(Point::new(0.5, 1.0, 0.5), 0.75, 0.25);

        test(Point::new(-0.5, -1.0, 0.5), 0.25, 0.75);
        test(Point::new(0.5, -1.0, -0.5), 0.75, 0.25);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn finding_the_colours_on_a_mapped_cube() {
        let brown = Colour::new(1.0, 0.5, 0.0);

        let c = CubicMapping::new(
            UvPattern::new_align_check(
                Colour::yellow(),
                Colour::cyan(),
                Colour::red(),
                Colour::blue(),
                brown,
            ),
            UvPattern::new_align_check(
                Colour::red(),
                Colour::yellow(),
                Colour::purple(),
                Colour::green(),
                Colour::white(),
            ),
            UvPattern::new_align_check(
                Colour::cyan(),
                Colour::red(),
                Colour::yellow(),
                brown,
                Colour::green(),
            ),
            UvPattern::new_align_check(
                Colour::green(),
                Colour::purple(),
                Colour::cyan(),
                Colour::white(),
                Colour::blue(),
            ),
            UvPattern::new_align_check(
                brown,
                Colour::cyan(),
                Colour::purple(),
                Colour::red(),
                Colour::yellow(),
            ),
            UvPattern::new_align_check(
                Colour::purple(),
                brown,
                Colour::green(),
                Colour::blue(),
                Colour::white(),
            ),
        );

        assert_approx_eq!(
            c.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Colour::yellow()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-1.0, 0.9, -0.9)),
            Colour::cyan()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-1.0, 0.9, 0.9)),
            Colour::red()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-1.0, -0.9, -0.9)),
            Colour::blue()
        );
        assert_approx_eq!(c.pattern_at(&Point::new(-1.0, -0.9, 0.9)), brown);

        assert_approx_eq!(
            c.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Colour::cyan()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, 0.9, 1.0)),
            Colour::red()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, 0.9, 1.0)),
            Colour::yellow()
        );
        assert_approx_eq!(c.pattern_at(&Point::new(-0.9, -0.9, 1.0)), brown);
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, -0.9, 1.0)),
            Colour::green()
        );

        assert_approx_eq!(
            c.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Colour::red()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(1.0, 0.9, 0.9)),
            Colour::yellow()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(1.0, 0.9, -0.9)),
            Colour::purple()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(1.0, -0.9, 0.9)),
            Colour::green()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(1.0, -0.9, -0.9)),
            Colour::white()
        );

        assert_approx_eq!(
            c.pattern_at(&Point::new(0.0, 0.0, -1.0)),
            Colour::green()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, 0.9, -1.0)),
            Colour::purple()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, 0.9, -1.0)),
            Colour::cyan()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, -0.9, -1.0)),
            Colour::white()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, -0.9, -1.0)),
            Colour::blue()
        );

        assert_approx_eq!(c.pattern_at(&Point::new(0.0, 1.0, 0.0)), brown);
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, 1.0, -0.9)),
            Colour::cyan()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, 1.0, -0.9)),
            Colour::purple()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, 1.0, 0.9)),
            Colour::red()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, 1.0, 0.9)),
            Colour::yellow()
        );

        assert_approx_eq!(
            c.pattern_at(&Point::new(0.0, -1.0, 0.0)),
            Colour::purple()
        );
        assert_approx_eq!(c.pattern_at(&Point::new(-0.9, -1.0, 0.9)), brown);
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, -1.0, 0.9)),
            Colour::green()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(-0.9, -1.0, -0.9)),
            Colour::blue()
        );
        assert_approx_eq!(
            c.pattern_at(&Point::new(0.9, -1.0, -0.9)),
            Colour::white()
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn comparing_cubes() {
        let c1 = CubicMapping::new(
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::blue(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::purple()),
            UvPattern::new_uv_checker(2, 2, Colour::white(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::blue()),
        );
        let c2 = CubicMapping::new(
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::blue(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::purple()),
            UvPattern::new_uv_checker(2, 2, Colour::white(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::blue()),
        );
        let c3 = CubicMapping::new(
            UvPattern::new_uv_checker(2, 2, Colour::yellow(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::blue(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::purple()),
            UvPattern::new_uv_checker(2, 2, Colour::white(), Colour::green()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::blue()),
        );

        assert_approx_eq!(c1, &c2);
        assert_approx_ne!(c1, &c3);
    }
}
