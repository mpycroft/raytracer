use derive_more::Constructor;
use float_cmp::{ApproxEq, F64Margin};

use super::PatternAt;
use crate::{math::Point, Colour};

/// A testing pattern that returns the passed in `Point` as a `Colour`.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Test;

impl PatternAt for Test {
    fn pattern_at(&self, point: &Point) -> Colour {
        Colour::new(point.x, point.y, point.z)
    }
}

impl ApproxEq for &Test {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(
        self,
        _other: Self,
        _margin: M,
    ) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn test_pattern_returns_point_as_colour() {
        let t = Test::new();

        assert_approx_eq!(
            t.pattern_at(&Point::new(1.0, 2.0, 3.0)),
            Colour::new(1.0, 2.0, 3.0)
        );
    }
}
