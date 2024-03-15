use super::UvPattern;
use crate::math::{
    float::{approx_eq, impl_approx_eq},
    Point,
};

#[derive(Clone, Copy, Debug)]
pub struct CubicMapping {
    left: UvPattern,
    right: UvPattern,
    front: UvPattern,
    back: UvPattern,
    up: UvPattern,
    down: UvPattern,
}

impl CubicMapping {
    #[must_use]
    pub fn cube_uv(point: Point) -> (f64, f64) {
        match Face::from(point) {
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

/// A representation of the faces of a cube.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl_approx_eq!(&CubicMapping {
    ref left,
    ref right,
    ref front,
    ref back,
    ref up,
    ref down
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Colour};

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
            let (u, v) = CubicMapping::cube_uv(point);

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
    fn comparing_cubes() {
        let c1 = CubicMapping {
            left: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::green(),
            ),
            right: UvPattern::new_uv_checker(
                2,
                2,
                Colour::blue(),
                Colour::green(),
            ),
            front: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::purple(),
            ),
            back: UvPattern::new_uv_checker(
                2,
                2,
                Colour::white(),
                Colour::green(),
            ),
            up: UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            down: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::blue(),
            ),
        };
        let c2 = CubicMapping {
            left: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::green(),
            ),
            right: UvPattern::new_uv_checker(
                2,
                2,
                Colour::blue(),
                Colour::green(),
            ),
            front: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::purple(),
            ),
            back: UvPattern::new_uv_checker(
                2,
                2,
                Colour::white(),
                Colour::green(),
            ),
            up: UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            down: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::blue(),
            ),
        };
        let c3 = CubicMapping {
            left: UvPattern::new_uv_checker(
                2,
                2,
                Colour::yellow(),
                Colour::green(),
            ),
            right: UvPattern::new_uv_checker(
                2,
                2,
                Colour::blue(),
                Colour::green(),
            ),
            front: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::purple(),
            ),
            back: UvPattern::new_uv_checker(
                2,
                2,
                Colour::white(),
                Colour::green(),
            ),
            up: UvPattern::new_uv_checker(2, 2, Colour::red(), Colour::black()),
            down: UvPattern::new_uv_checker(
                2,
                2,
                Colour::red(),
                Colour::blue(),
            ),
        };

        assert_approx_eq!(c1, &c2);
        assert_approx_ne!(c1, &c3);
    }
}
