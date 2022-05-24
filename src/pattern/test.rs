use std::marker::PhantomData;

use super::PatternAt;
use crate::{math::Point, util::float::Float, Colour};

/// A pattern for testing purposes.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Test<T: Float> {
    _phantom: PhantomData<T>,
}

impl<T: Float> Test<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Float> PatternAt<T> for Test<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        Colour::new(point.x, point.y, point.z)
    }
}

add_approx_traits!(Test<T> { true });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn pattern_at_of_a_test_object() {
        assert_relative_eq!(
            Test::new().pattern_at(&Point::new(1.0, 0.6, 2.0)),
            Colour::new(1.0, 0.6, 2.0)
        );
    }

    #[test]
    fn test_patterns_are_approximately_equal() {
        let t1 = Test::<f64>::new();
        let t2 = Test::new();

        assert_abs_diff_eq!(t1, t2);

        assert_relative_eq!(t1, t2);

        assert_ulps_eq!(t1, t2);
    }
}
