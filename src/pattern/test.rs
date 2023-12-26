use crate::{math::Point, Colour};

#[derive(Clone, Copy, Debug)]
pub struct Test;

impl Test {
    #[must_use]
    pub fn pattern_at(&self, point: &Point) -> Colour {
        Colour::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::assert_approx_eq;

    #[test]
    fn test_pattern_returns_point_as_colour() {
        let t = Test;

        assert_approx_eq!(
            t.pattern_at(&Point::new(1.0, 2.0, 3.0)),
            Colour::new(1.0, 2.0, 3.0)
        );
    }
}
