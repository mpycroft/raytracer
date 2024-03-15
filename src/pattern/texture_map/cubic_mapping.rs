use crate::math::{float::approx_eq, Point};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifying_the_face_of_a_cube_from_a_point() {
        assert_eq!(Face::from(Point::new(-1.0, 0.5, -0.25)), Face::Left);
        assert_eq!(Face::from(Point::new(1.1, -0.75, 0.8)), Face::Right);
        assert_eq!(Face::from(Point::new(0.1, 0.6, 0.9)), Face::Front);
        assert_eq!(Face::from(Point::new(-0.7, 0.0, -2.0)), Face::Back);
        assert_eq!(Face::from(Point::new(0.5, 1.0, 0.9)), Face::Up);
        assert_eq!(Face::from(Point::new(-0.2, -1.3, 1.1)), Face::Down);
    }
}
